//! Rate limiting for authentication attempts
//! 
//! Provides protection against brute force attacks by limiting
//! the number of authentication attempts within a time window.

pub mod limiter;
pub mod models;
pub mod storage;

// Fix the exports - export the right items
pub use limiter::{RateLimiter, RateLimitResult};
pub use models::{RateLimitConfig, RateLimitState};
pub use storage::InMemoryRateLimitStorage;
