//! Main rate limiter implementation

use crate::rate_limiter::models::{RateLimitConfig, RateLimitState};
use crate::rate_limiter::storage::{InMemoryRateLimitStorage, RateLimitStorage};
use std::sync::Arc;

/// Main rate limiter that coordinates rate limiting logic
pub struct RateLimiter {
    storage: Arc<dyn RateLimitStorage>,
    config: RateLimitConfig,
}

impl RateLimiter {
    /// Create a new rate limiter with default in-memory storage
    pub fn new(config: RateLimitConfig) -> Self {
        Self {
            storage: Arc::new(InMemoryRateLimitStorage::new()),
            config,
        }
    }

    /// Create a new rate limiter with custom storage
    pub fn with_storage(config: RateLimitConfig, storage: Arc<dyn RateLimitStorage>) -> Self {
        Self { storage, config }
    }

    /// Check if an identifier is allowed to make an authentication attempt
    pub async fn check_rate_limit(&self, identifier: &str) -> RateLimitResult {
        let state = match self.storage.get_state(identifier).await {
            Some(existing_state) => existing_state,
            None => RateLimitState::new(identifier.to_string()),
        };

        // Check if currently rate limited
        if state.is_rate_limited() {
            return RateLimitResult::Denied {
                remaining_lockout_time: state.lockout_time_remaining(),
                max_attempts: self.config.max_attempts,
                attempts_used: state.attempt_count,
            };
        }

        // Check remaining attempts
        let remaining_attempts = state.remaining_attempts(&self.config);
        if remaining_attempts == 0 {
            // This shouldn't happen normally, but let's be safe
            return RateLimitResult::Denied {
                remaining_lockout_time: state.lockout_time_remaining(),
                max_attempts: self.config.max_attempts,
                attempts_used: state.attempt_count,
            };
        }

        RateLimitResult::Allowed {
            remaining_attempts,
            max_attempts: self.config.max_attempts,
        }
    }

    /// Record a failed authentication attempt
    pub async fn record_failure(&self, identifier: &str) -> Result<(), RateLimitError> {
        let mut state = match self.storage.get_state(identifier).await {
        Some(existing_state) => existing_state,
        None => RateLimitState::new(identifier.to_string()),
    };

    state.record_failure(&self.config);
    self.storage.save_state(state).await
        .map_err(|e| RateLimitError::StorageError(e.to_string()))
}


    /// Record a successful authentication attempt
    pub async fn record_success(&self, identifier: &str) -> Result<(), RateLimitError> {
        let mut state = match self.storage.get_state(identifier).await {
            Some(existing_state) => existing_state,
            None => RateLimitState::new(identifier.to_string()),
        };

        state.record_success(&self.config);
        self.storage
            .save_state(state)
            .await
            .map_err(|e| RateLimitError::StorageError(e.to_string()))
    }

    /// Reset rate limit state for an identifier
    pub async fn reset_identifier(&self, identifier: &str) -> Result<(), RateLimitError> {
        self.storage
            .remove_state(identifier)
            .await
            .map_err(|e| RateLimitError::StorageError(e.to_string()))
            .map(|_| ())
    }

    /// Get current rate limit state for an identifier
    pub async fn get_current_state(&self, identifier: &str) -> Option<RateLimitState> {
        self.storage.get_state(identifier).await
    }

    /// Clear all rate limit states
    pub async fn clear_all_states(&self) -> Result<(), RateLimitError> {
        self.storage
            .clear_all()
            .await
            .map_err(|e| RateLimitError::StorageError(e.to_string()))
    }
}

/// Result of a rate limit check
#[derive(Debug, Clone)]
pub enum RateLimitResult {
    /// Authentication attempt is allowed
    Allowed {
        /// Number of attempts remaining before lockout
        remaining_attempts: u32,
        /// Maximum attempts allowed
        max_attempts: u32,
    },
    /// Authentication attempt is denied due to rate limiting
    Denied {
        /// Time remaining until lockout expires
        remaining_lockout_time: Option<std::time::Duration>,
        /// Maximum attempts allowed
        max_attempts: u32,
        /// Attempts used so far
        attempts_used: u32,
    },
}

/// Rate limiting errors
#[derive(Debug)]
pub enum RateLimitError {
    /// Storage backend error
    StorageError(String),
    /// Configuration error
    ConfigError(String),
}

impl std::fmt::Display for RateLimitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RateLimitError::StorageError(msg) => write!(f, "Storage error: {}", msg),
            RateLimitError::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
        }
    }
}

impl std::error::Error for RateLimitError {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_rate_limiter_allowed() {
        let config = RateLimitConfig::default();
        let limiter = RateLimiter::new(config);
        let identifier = "192.168.1.1";

        // First check should be allowed
        let result = limiter.check_rate_limit(identifier).await;

        match result {
            RateLimitResult::Allowed {
                remaining_attempts, ..
            } => {
                assert_eq!(remaining_attempts, 5); // Default max_attempts
            }
            _ => panic!("Should be allowed"),
        }
    }

    #[tokio::test]
    async fn test_rate_limiter_denied_after_failures() {
        let config = RateLimitConfig {
            max_attempts: 2,
            window_duration: Duration::from_secs(60),
            lockout_duration: Duration::from_secs(300),
            reset_on_success: true,
        };
        let limiter = RateLimiter::new(config);
        let identifier = "test_user";

        // Record failures
        limiter.record_failure(identifier).await.unwrap();
        limiter.record_failure(identifier).await.unwrap();

        // Should be denied now
        let result = limiter.check_rate_limit(identifier).await;

        match result {
            RateLimitResult::Denied { .. } => assert!(true),
            _ => panic!("Should be denied after max attempts"),
        }
    }

    #[tokio::test]
    async fn test_rate_limiter_reset_on_success() {
        let config = RateLimitConfig::default();
        let limiter = RateLimiter::new(config);
        let identifier = "test_user";

        // Record some failures
        limiter.record_failure(identifier).await.unwrap();
        limiter.record_failure(identifier).await.unwrap();

        // Record success - should reset counters
        limiter.record_success(identifier).await.unwrap();

        // Should be allowed again
        let result = limiter.check_rate_limit(identifier).await;

        match result {
            RateLimitResult::Allowed { .. } => assert!(true),
            _ => panic!("Should be allowed after success"),
        }
    }

    #[tokio::test]
    async fn test_rate_limiter_reset_identifier() {
        let config = RateLimitConfig::default();
        let limiter = RateLimiter::new(config);
        let identifier = "test_user";

        // Record some failures
        limiter.record_failure(identifier).await.unwrap();
        limiter.record_failure(identifier).await.unwrap();

        // Reset identifier
        limiter.reset_identifier(identifier).await.unwrap();

        // Should be allowed again (fresh start)
        let result = limiter.check_rate_limit(identifier).await;

        match result {
            RateLimitResult::Allowed {
                remaining_attempts, ..
            } => {
                assert_eq!(remaining_attempts, 5); // Back to max
            }
            _ => panic!("Should be allowed after reset"),
        }
    }
}
