//! Session manager for creating, validating, and managing user sessions

use crate::session::models::{Session, SessionValidation};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Manages user sessions with in-memory storage
///
/// This is a basic implementation. For production use,
/// consider implementing database-backed storage.
#[derive(Clone)]
pub struct SessionManager {
    /// In-memory session storage
    sessions: Arc<RwLock<HashMap<Uuid, Session>>>,
    /// Default session duration (seconds)
    default_duration: Duration,
    /// Maximum number of sessions per user
    max_sessions_per_user: usize,
}

impl SessionManager {
    /// Create a new session manager with default settings
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
            default_duration: Duration::from_secs(3600), // 1 hour
            max_sessions_per_user: 10,
        }
    }

    /// Create a new session manager with custom settings
    pub fn with_config(default_duration: Duration, max_sessions_per_user: usize) -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
            default_duration,
            max_sessions_per_user,
        }
    }

    /// Create a new session for a user
    pub async fn create_session(
        &self,
        user_id: Uuid,
        persistent: bool,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Result<Session, SessionError> {
        // Clean up expired sessions first
        self.cleanup_expired_sessions().await;

        // Check session limit for user
        let session_count = self.count_user_sessions(user_id).await;
        if session_count >= self.max_sessions_per_user {
            return Err(SessionError::SessionLimitExceeded);
        }

        // Create new session
        let mut session = Session::new(user_id, self.default_duration, persistent);
        session.ip_address = ip_address;
        session.user_agent = user_agent;

        // Store session
        {
            let mut sessions = self.sessions.write().await;
            sessions.insert(session.id, session.clone());
        }

        Ok(session)
    }

    /// Validate a session by ID
    pub async fn validate_session(&self, session_id: Uuid) -> SessionValidation {
        let sessions = self.sessions.read().await;

        match sessions.get(&session_id) {
            Some(session) => {
                if session.is_expired() {
                    SessionValidation::Expired
                } else {
                    SessionValidation::Valid(session.clone())
                }
            }
            None => SessionValidation::Invalid,
        }
    }

    /// Revoke a session by ID
    pub async fn revoke_session(&self, session_id: Uuid) -> Result<(), SessionError> {
        let mut sessions = self.sessions.write().await;
        if sessions.remove(&session_id).is_some() {
            Ok(())
        } else {
            Err(SessionError::SessionNotFound)
        }
    }

    /// Revoke all sessions for a user
    pub async fn revoke_user_sessions(&self, user_id: Uuid) -> usize {
        let mut sessions = self.sessions.write().await;
        let mut count = 0;

        sessions.retain(|_, session| {
            if session.user_id == user_id {
                count += 1;
                false // Remove this session
            } else {
                true // Keep this session
            }
        });

        count
    }

    /// Extend a session's lifetime
    pub async fn extend_session(&self, session_id: Uuid) -> Result<(), SessionError> {
        let mut sessions = self.sessions.write().await;

        match sessions.get_mut(&session_id) {
            Some(session) => {
                if session.is_expired() {
                    Err(SessionError::SessionExpired)
                } else {
                    session.extend(self.default_duration);
                    Ok(())
                }
            }
            None => Err(SessionError::SessionNotFound),
        }
    }

    /// Get all active sessions for a user
    pub async fn get_user_sessions(&self, user_id: Uuid) -> Vec<Session> {
        let sessions = self.sessions.read().await;
        sessions
            .values()
            .filter(|session| session.user_id == user_id && !session.is_expired())
            .cloned()
            .collect()
    }

    /// Count active sessions for a user
    pub async fn count_user_sessions(&self, user_id: Uuid) -> usize {
        let sessions = self.sessions.read().await;
        sessions
            .values()
            .filter(|session| session.user_id == user_id && !session.is_expired())
            .count()
    }

    /// Clean up expired sessions from memory
    pub async fn cleanup_expired_sessions(&self) {
        let mut sessions = self.sessions.write().await;
        sessions.retain(|_, session| !session.is_expired());
    }

    /// Get total number of active sessions
    pub async fn active_session_count(&self) -> usize {
        let sessions = self.sessions.read().await;
        sessions
            .values()
            .filter(|session| !session.is_expired())
            .count()
    }
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Session management errors
#[derive(Debug, Clone, PartialEq)]
pub enum SessionError {
    /// Session not found
    SessionNotFound,
    /// Session has expired
    SessionExpired,
    /// User has reached maximum session limit
    SessionLimitExceeded,
    /// Storage error (for database implementations)
    StorageError(String),
}

impl std::fmt::Display for SessionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SessionError::SessionNotFound => write!(f, "Session not found"),
            SessionError::SessionExpired => write!(f, "Session has expired"),
            SessionError::SessionLimitExceeded => write!(f, "Maximum session limit exceeded"),
            SessionError::StorageError(msg) => write!(f, "Storage error: {}", msg),
        }
    }
}

impl std::error::Error for SessionError {}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_create_session() {
        let manager = SessionManager::new();
        let user_id = Uuid::new_v4();

        let session = manager
            .create_session(
                user_id,
                false,
                Some("127.0.0.1".to_string()),
                Some("Mozilla/5.0".to_string()),
            )
            .await
            .unwrap();

        assert_eq!(session.user_id, user_id);
        assert_eq!(session.ip_address, Some("127.0.0.1".to_string()));
        assert_eq!(session.user_agent, Some("Mozilla/5.0".to_string()));
    }

    #[tokio::test]
    async fn test_validate_session() {
        let manager = SessionManager::new();
        let user_id = Uuid::new_v4();

        let session = manager
            .create_session(user_id, false, None, None)
            .await
            .unwrap();
        let validation = manager.validate_session(session.id).await;

        match validation {
            SessionValidation::Valid(_) => assert!(true),
            _ => assert!(false, "Session should be valid"),
        }
    }

    #[tokio::test]
    async fn test_revoke_session() {
        let manager = SessionManager::new();
        let user_id = Uuid::new_v4();

        let session = manager
            .create_session(user_id, false, None, None)
            .await
            .unwrap();
        assert!(manager.revoke_session(session.id).await.is_ok());

        let validation = manager.validate_session(session.id).await;
        match validation {
            SessionValidation::Invalid => assert!(true),
            _ => assert!(false, "Session should be invalid after revocation"),
        }
    }

    #[tokio::test]
    async fn test_cleanup_expired_sessions() {
        let manager = SessionManager::with_config(Duration::from_secs(0), 10);
        let user_id = Uuid::new_v4();

        // Create an expired session
        let _session = manager
            .create_session(user_id, false, None, None)
            .await
            .unwrap();

        // Small delay to ensure expiration
        tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;

        // Cleanup should remove expired sessions
        manager.cleanup_expired_sessions().await;

        assert_eq!(manager.active_session_count().await, 0);
    }
}
