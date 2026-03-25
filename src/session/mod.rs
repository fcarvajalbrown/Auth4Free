//! Session management for Auth4Free
//!
//! Provides secure session handling with expiration, refresh tokens,
//! and session validation.

pub mod manager;
pub mod models;
pub mod storage;

// Re-export commonly used items
pub use manager::SessionManager;
pub use models::{Session, SessionValidation};
