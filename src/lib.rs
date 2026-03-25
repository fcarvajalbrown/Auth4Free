// src/lib.rs

pub mod auth;
pub mod password_hasher;
pub mod password_validation;
pub mod rate_limiter;
pub mod session;
pub mod token_generator;
pub mod user;

// Re-export commonly used items
pub use password_validation::PasswordValidationConfig;
pub use password_validation::PasswordValidationError;
pub use password_validation::password_strength_category;
pub use password_validation::password_strength_score;
pub use password_validation::validate_password;
pub use rate_limiter::RateLimiter;
pub use rate_limiter::models::{RateLimitConfig, RateLimitResult, RateLimitState};
pub use session::Session;
