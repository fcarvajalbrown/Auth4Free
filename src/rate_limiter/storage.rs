//! Storage implementations for rate limiting state

use crate::rate_limiter::models::RateLimitState;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Trait for rate limit storage implementations
#[async_trait::async_trait]
pub trait RateLimitStorage: Send + Sync {
    /// Get rate limit state for an identifier
    async fn get_state(&self, identifier: &str) -> Option<RateLimitState>;

    /// Save rate limit state for an identifier
    async fn save_state(&self, state: RateLimitState) -> Result<(), RateLimitStorageError>;

    /// Remove rate limit state for an identifier
    async fn remove_state(&self, identifier: &str) -> Result<bool, RateLimitStorageError>;

    /// Clear all rate limit states
    async fn clear_all(&self) -> Result<(), RateLimitStorageError>;

    /// Get count of tracked identifiers
    async fn count_tracked(&self) -> usize;
}

/// In-memory rate limit storage implementation
#[derive(Clone)]
pub struct InMemoryRateLimitStorage {
    states: Arc<RwLock<HashMap<String, RateLimitState>>>,
}

impl InMemoryRateLimitStorage {
    pub fn new() -> Self {
        Self {
            states: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for InMemoryRateLimitStorage {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl RateLimitStorage for InMemoryRateLimitStorage {
    async fn get_state(&self, identifier: &str) -> Option<RateLimitState> {
        let states = self.states.read().await;
        states.get(identifier).cloned()
    }

    async fn save_state(&self, state: RateLimitState) -> Result<(), RateLimitStorageError> {
        let mut states = self.states.write().await;
        states.insert(state.identifier.clone(), state);
        Ok(())
    }

    async fn remove_state(&self, identifier: &str) -> Result<bool, RateLimitStorageError> {
        let mut states = self.states.write().await;
        Ok(states.remove(identifier).is_some())
    }

    async fn clear_all(&self) -> Result<(), RateLimitStorageError> {
        let mut states = self.states.write().await;
        states.clear();
        Ok(())
    }

    async fn count_tracked(&self) -> usize {
        let states = self.states.read().await;
        states.len()
    }
}

/// Rate limit storage errors
#[derive(Debug)]
pub enum RateLimitStorageError {
    /// Storage backend error
    BackendError(String),
    /// Serialization error
    SerializationError(String),
    /// Connection error
    ConnectionError(String),
}

impl std::fmt::Display for RateLimitStorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RateLimitStorageError::BackendError(msg) => write!(f, "Backend error: {}", msg),
            RateLimitStorageError::SerializationError(msg) => {
                write!(f, "Serialization error: {}", msg)
            }
            RateLimitStorageError::ConnectionError(msg) => write!(f, "Connection error: {}", msg),
        }
    }
}

impl std::error::Error for RateLimitStorageError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rate_limiter::models::RateLimitState;

    #[tokio::test]
    async fn test_in_memory_storage() {
        let storage = InMemoryRateLimitStorage::new();
        let mut state = RateLimitState::new("test_user".to_string());
        state.attempt_count = 3;

        // Save state
        assert!(storage.save_state(state.clone()).await.is_ok());

        // Retrieve state
        let retrieved = storage.get_state("test_user").await;
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().attempt_count, 3);

        // Remove state
        assert!(storage.remove_state("test_user").await.unwrap());
        assert!(!storage.remove_state("test_user").await.unwrap()); // Already removed

        // State should not be found
        let retrieved = storage.get_state("test_user").await;
        assert!(retrieved.is_none());
    }

    #[tokio::test]
    async fn test_clear_all() {
        let storage = InMemoryRateLimitStorage::new();

        // Add some states
        let state1 = RateLimitState::new("user1".to_string());
        let state2 = RateLimitState::new("user2".to_string());

        storage.save_state(state1).await.unwrap();
        storage.save_state(state2).await.unwrap();

        assert_eq!(storage.count_tracked().await, 2);

        // Clear all
        storage.clear_all().await.unwrap();

        assert_eq!(storage.count_tracked().await, 0);
    }
}
