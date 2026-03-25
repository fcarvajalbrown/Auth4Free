//! Session storage implementations for different backends

use crate::session::models::Session;
use std::collections::HashMap;
use uuid::Uuid;

/// Trait for session storage implementations
#[async_trait::async_trait]
pub trait SessionStorage: Send + Sync {
    /// Store a session
    async fn store_session(&self, session: Session) -> Result<(), SessionStorageError>;

    /// Retrieve a session by ID
    async fn get_session(&self, session_id: Uuid) -> Result<Option<Session>, SessionStorageError>;

    /// Remove a session by ID
    async fn remove_session(&self, session_id: Uuid) -> Result<bool, SessionStorageError>;

    /// Remove all sessions for a user
    async fn remove_user_sessions(&self, user_id: Uuid) -> Result<usize, SessionStorageError>;

    /// Clean up expired sessions
    async fn cleanup_expired_sessions(&self) -> Result<usize, SessionStorageError>;

    /// Count active sessions for a user
    async fn count_user_sessions(&self, user_id: Uuid) -> Result<usize, SessionStorageError>;
}

/// In-memory session storage implementation
#[derive(Clone)]
pub struct InMemorySessionStorage {
    sessions: std::sync::Arc<tokio::sync::RwLock<HashMap<Uuid, Session>>>,
}

impl InMemorySessionStorage {
    pub fn new() -> Self {
        Self {
            sessions: std::sync::Arc::new(tokio::sync::RwLock::new(HashMap::new())),
        }
    }
}

impl Default for InMemorySessionStorage {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl SessionStorage for InMemorySessionStorage {
    async fn store_session(&self, session: Session) -> Result<(), SessionStorageError> {
        let mut sessions = self.sessions.write().await;
        sessions.insert(session.id, session);
        Ok(())
    }

    async fn get_session(&self, session_id: Uuid) -> Result<Option<Session>, SessionStorageError> {
        let sessions = self.sessions.read().await;
        Ok(sessions.get(&session_id).cloned())
    }

    async fn remove_session(&self, session_id: Uuid) -> Result<bool, SessionStorageError> {
        let mut sessions = self.sessions.write().await;
        Ok(sessions.remove(&session_id).is_some())
    }

    async fn remove_user_sessions(&self, user_id: Uuid) -> Result<usize, SessionStorageError> {
        let mut sessions = self.sessions.write().await;
        let mut count = 0;
        sessions.retain(|_, session| {
            if session.user_id == user_id {
                count += 1;
                false
            } else {
                true
            }
        });
        Ok(count)
    }

    async fn cleanup_expired_sessions(&self) -> Result<usize, SessionStorageError> {
        let mut sessions = self.sessions.write().await;
        let mut count = 0;
        sessions.retain(|_, session| {
            if session.is_expired() {
                count += 1;
                false
            } else {
                true
            }
        });
        Ok(count)
    }

    async fn count_user_sessions(&self, user_id: Uuid) -> Result<usize, SessionStorageError> {
        let sessions = self.sessions.read().await;
        let count = sessions
            .values()
            .filter(|session| session.user_id == user_id && !session.is_expired())
            .count();
        Ok(count)
    }
}

/// Session storage errors
#[derive(Debug)]
pub enum SessionStorageError {
    /// Storage backend error
    BackendError(String),
    /// Serialization error
    SerializationError(String),
    /// Connection error
    ConnectionError(String),
}

impl std::fmt::Display for SessionStorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SessionStorageError::BackendError(msg) => write!(f, "Backend error: {}", msg),
            SessionStorageError::SerializationError(msg) => {
                write!(f, "Serialization error: {}", msg)
            }
            SessionStorageError::ConnectionError(msg) => write!(f, "Connection error: {}", msg),
        }
    }
}

impl std::error::Error for SessionStorageError {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_in_memory_storage() {
        let storage = InMemorySessionStorage::new();
        let user_id = Uuid::new_v4();
        let session = Session::new(user_id, Duration::from_secs(3600), false);

        // Store session
        assert!(storage.store_session(session.clone()).await.is_ok());

        // Retrieve session
        let retrieved = storage.get_session(session.id).await.unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().id, session.id);

        // Remove session
        assert!(storage.remove_session(session.id).await.unwrap());
        assert!(!storage.remove_session(session.id).await.unwrap()); // Already removed

        // Session should not be found
        let retrieved = storage.get_session(session.id).await.unwrap();
        assert!(retrieved.is_none());
    }
}
