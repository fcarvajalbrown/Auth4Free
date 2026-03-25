//! Rate limiting data models and configurations

use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};

/// Configuration for rate limiting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    /// Maximum number of attempts allowed
    pub max_attempts: u32,
    /// Time window for attempt counting (in seconds)
    pub window_duration: Duration,
    /// Duration to lock out after exceeding limits (in seconds)
    pub lockout_duration: Duration,
    /// Whether to reset counters after successful auth
    pub reset_on_success: bool,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            max_attempts: 5,
            window_duration: Duration::from_secs(300), // 5 minutes
            lockout_duration: Duration::from_secs(900), // 15 minutes
            reset_on_success: true,
        }
    }
}

/// State tracking for a specific identifier (IP, user, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitState {
    /// Identifier being tracked (IP address, user ID, etc.)
    pub identifier: String,
    /// Number of attempts in current window
    pub attempt_count: u32,
    /// Timestamp of first attempt in current window
    pub window_start: SystemTime,
    /// Timestamp when lockout expires (if currently locked)
    pub lockout_until: Option<SystemTime>,
    /// Timestamp of last successful authentication
    pub last_success: Option<SystemTime>,
}

impl RateLimitState {
    /// Create new rate limit state for an identifier
    pub fn new(identifier: String) -> Self {
        Self {
            identifier,
            attempt_count: 0,
            window_start: SystemTime::now(),
            lockout_until: None,
            last_success: None,
        }
    }

    /// Record a failed authentication attempt
    pub fn record_failure(&mut self, config: &RateLimitConfig) {
        let now = SystemTime::now();

        // Check if we're currently locked out
        if let Some(lockout_until) = self.lockout_until {
            if now < lockout_until {
                return; // Still locked out
            } else {
                // Lockout period expired, reset
                self.lockout_until = None;
                self.attempt_count = 0;
                self.window_start = now;
            }
        }

        // Check if we're still in the same window
        if let Ok(elapsed) = now.duration_since(self.window_start) {
            if elapsed > config.window_duration {
                // Window has expired, reset counter
                self.attempt_count = 0;
                self.window_start = now;
            }
        }

        // Increment attempt count
        self.attempt_count += 1;

        // Check if we've exceeded the limit
        if self.attempt_count >= config.max_attempts {
            // Apply lockout
            self.lockout_until = Some(now + config.lockout_duration);
        }
    }

    /// Record a successful authentication
    pub fn record_success(&mut self, config: &RateLimitConfig) {
        self.last_success = Some(SystemTime::now());

        if config.reset_on_success {
            // Reset all counters on success
            self.attempt_count = 0;
            self.window_start = SystemTime::now();
            self.lockout_until = None;
        }
    }

    /// Check if the identifier is currently rate limited
    pub fn is_rate_limited(&self) -> bool {
        if let Some(lockout_until) = self.lockout_until {
            SystemTime::now() < lockout_until
        } else {
            false
        }
    }

    /// Get remaining attempts before lockout
    pub fn remaining_attempts(&self, config: &RateLimitConfig) -> u32 {
        if self.is_rate_limited() {
            0
        } else {
            config.max_attempts.saturating_sub(self.attempt_count)
        }
    }

    /// Get time until lockout expires (if locked)
    pub fn lockout_time_remaining(&self) -> Option<Duration> {
        if let Some(lockout_until) = self.lockout_until {
            lockout_until.duration_since(SystemTime::now()).ok()
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration as StdDuration;

    #[test]
    fn test_rate_limit_state_creation() {
        let state = RateLimitState::new("192.168.1.1".to_string());
        assert_eq!(state.identifier, "192.168.1.1");
        assert_eq!(state.attempt_count, 0);
        assert!(state.lockout_until.is_none());
    }

    #[test]
    fn test_record_failure_and_lockout() {
        let mut state = RateLimitState::new("test_user".to_string());
        let config = RateLimitConfig {
            max_attempts: 3,
            window_duration: StdDuration::from_secs(60),
            lockout_duration: StdDuration::from_secs(300),
            reset_on_success: true,
        };

        // Record 3 failures
        for _ in 0..3 {
            state.record_failure(&config);
        }

        // Should be locked out now
        assert!(state.is_rate_limited());
        assert_eq!(state.attempt_count, 3);
        assert!(state.lockout_until.is_some());
    }

    #[test]
    fn test_record_success_resets_counters() {
        let mut state = RateLimitState::new("test_user".to_string());
        let config = RateLimitConfig::default();

        // Record some failures
        state.record_failure(&config);
        state.record_failure(&config);

        assert_eq!(state.attempt_count, 2);

        // Record success
        state.record_success(&config);

        if config.reset_on_success {
            assert_eq!(state.attempt_count, 0);
            assert!(state.lockout_until.is_none());
        }
    }

    #[test]
    fn test_window_reset() {
        let mut state = RateLimitState::new("test_user".to_string());
        let config = RateLimitConfig {
            max_attempts: 2,
            window_duration: StdDuration::from_secs(1), // Very short window
            lockout_duration: StdDuration::from_secs(300),
            reset_on_success: true,
        };

        // Record failures
        state.record_failure(&config);
        state.record_failure(&config);

        assert_eq!(state.attempt_count, 2);

        // Wait for window to expire
        thread::sleep(StdDuration::from_secs(2));

        // Record another failure - should reset window
        state.record_failure(&config);
        assert_eq!(state.attempt_count, 1); // Reset to 1
    }
}
