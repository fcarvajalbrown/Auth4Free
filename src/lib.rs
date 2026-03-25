// src/lib.rs

pub mod user;
pub mod auth;
pub mod token_generator;
pub mod password_hasher;
pub mod password_validation; // Add this line

// Re-export commonly used items
pub use password_validation::validate_password;
pub use password_validation::password_strength_score;
pub use password_validation::password_strength_category;
pub use password_validation::PasswordValidationConfig;
pub use password_validation::PasswordValidationError;
