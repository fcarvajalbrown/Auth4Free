// src/lib.rs

pub mod auth;
pub mod password_hasher;
pub mod password_validation;
pub mod token_generator;
pub mod user;
pub mod session;

// Re-export commonly used items
pub use password_validation::PasswordValidationConfig;
pub use password_validation::PasswordValidationError;
pub use password_validation::password_strength_category;
pub use password_validation::password_strength_score;
pub use password_validation::validate_password;
pub use session::Session;
