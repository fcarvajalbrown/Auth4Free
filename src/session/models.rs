//! Session data structures and models

use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};
use uuid::Uuid;

/// Represents an active user session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    /// Unique session identifier
    pub id: Uuid,
    /// Associated user ID
    pub user_id: Uuid,
    /// Session creation timestamp
    pub created_at: SystemTime,
    /// Session expiration timestamp
    pub expires_at: SystemTime,
    /// Refresh token for session renewal
    pub refresh_token: String,
    /// IP address where session was created
    pub ip_address: Option<String>,
    /// User agent of the client
    pub user_agent: Option<String>,
    /// Whether this is a remember-me session
    pub is_persistent: bool,
}

impl Session {
    /// Create a new session for a user
    pub fn new(user_id: Uuid, duration: Duration, persistent: bool) -> Self {
        let now = SystemTime::now();
        let expires_at = now + duration;

        Self {
            id: Uuid::new_v4(),
            user_id,
            created_at: now,
            expires_at,
            refresh_token: generate_refresh_token(),
            ip_address: None,
            user_agent: None,
            is_persistent: persistent,
        }
    }

    /// Check if session has expired
    pub fn is_expired(&self) -> bool {
        SystemTime::now() > self.expires_at
    }

    /// Get time remaining until expiration
    pub fn time_remaining(&self) -> Option<Duration> {
        self.expires_at.duration_since(SystemTime::now()).ok()
    }

    /// Extend session expiration
    pub fn extend(&mut self, duration: Duration) {
        self.expires_at = SystemTime::now() + duration;
    }
}

/// Generate a secure refresh token
fn generate_refresh_token() -> String {
    use rand::{Rng, distributions::Alphanumeric};

    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect()
}

/// Session validation result
#[derive(Debug, Clone)]
pub enum SessionValidation {
    /// Session is valid
    Valid(Session),
    /// Session has expired
    Expired,
    /// Session is invalid (malformed, revoked, etc.)
    Invalid,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_session_creation() {
        let user_id = Uuid::new_v4();
        let session = Session::new(user_id, Duration::from_secs(3600), false);

        assert_eq!(session.user_id, user_id);
        assert!(!session.is_expired());
        assert!(!session.refresh_token.is_empty());
        assert_eq!(session.is_persistent, false);
    }

    #[test]
    fn test_session_expiration() {
        let user_id = Uuid::new_v4();
        let session = Session::new(user_id, Duration::from_secs(0), false);

        // Small delay to ensure expiration
        std::thread::sleep(std::time::Duration::from_millis(1));

        assert!(session.is_expired());
    }

    #[test]
    fn test_session_extension() {
        let user_id = Uuid::new_v4();
        let mut session = Session::new(user_id, Duration::from_secs(1), false);

        // Extend by 1 hour
        session.extend(Duration::from_secs(3600));

        // Should not be expired now
        assert!(!session.is_expired());
    }
}
