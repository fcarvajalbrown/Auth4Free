//! Rate limiting for authentication attempts
//!
//! Provides protection against brute force attacks by limiting
//! the number of authentication attempts within a time window.

pub mod limiter;
pub mod models;
pub mod storage;

pub use limiter::RateLimiter;
pub use models::{RateLimitConfig, RateLimitState};
pub use storage::{InMemoryStorage, Storage};
