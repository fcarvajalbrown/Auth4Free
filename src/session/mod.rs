//! Session management for Auth4Free
//! 
//! Provides secure session handling with expiration, refresh tokens,
//! and session validation.

pub mod models;
pub mod manager;
pub mod storage;

// Re-export commonly used items
pub use models::{Session, SessionValidation};
pub use manager::SessionManager;