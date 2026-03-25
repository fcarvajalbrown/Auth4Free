//! Auth4Free - A modern, secure authentication library for Rust applications.
//! 
//! Features:
//! - Password validation and strength checking
//! - JWT token authentication
//! - Password hashing with bcrypt
//! - User management
//! - Session management (planned)
//! - Multi-factor authentication (planned)

// Re-export all public modules
pub mod user;
pub mod auth;
pub mod token_generator;
pub mod password_hasher;
pub mod password_validation;

// Re-export commonly used items for convenience
pub use password_validation::{
    validate_password,
    password_strength_score,
    password_strength_category,
    PasswordValidationConfig,
    PasswordValidationError,
};

pub use user::models::User;
pub use auth::authenticate_user;
pub use token_generator::generate_token;
pub use password_hasher::hash_password;

/// Auth4Free library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
