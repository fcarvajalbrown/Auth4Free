//! Example showing user registration with password validation
//!
//! This demonstrates integrating password validation into a user
//! registration workflow.

use authlib::password_validation::*;
use authlib::password_hasher;
use authlib::user::User;
use uuid::Uuid;

#[derive(Debug)]
struct RegistrationError {
    message: String,
}

impl std::fmt::Display for RegistrationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for RegistrationError {}

struct UserService {
    password_config: PasswordValidationConfig,
}

impl UserService {
    fn new() -> Self {
        Self {
            password_config: PasswordValidationConfig::default(),
        }
    }

    async fn register_user(
        &self,
        username: String,
        email: String,
        password: String,
    ) -> Result<User, Box<dyn std::error::Error>> {
        // Validate password first
        validate_password(&password, &self.password_config)
            .map_err(|e| RegistrationError { 
                message: format!("Password validation failed: {}", e) 
            })?;

        // Check password strength (optional warning)
        let score = password_strength_score(&password);
        let category = password_strength_category(score);
        
        if score < 50 {
            println!("⚠️  Warning: Password strength is '{}'. Consider using a stronger password.", category);
        }

        // Hash password (simulated)
        let _hashed_password = password_hasher::hash_password(&password).await
            .map_err(|e| RegistrationError { 
                message: format!("Failed to hash password: {}", e) 
            })?;

        // Create user (simulated)
        let user = User {
            id: Uuid::new_v4(),
            username,
            email,
        };

        println!("✅ User '{}' registered successfully!", user.username);
        println!("   Password strength: {} ({}/100)", category, score);
        
        Ok(user)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== User Registration with Password Validation ===\n");

    let user_service = UserService::new();

    // Successful registration
    println!("1. Registering user with strong password:");
    let user = user_service
        .register_user(
            "john_doe".to_string(),
            "john@example.com".to_string(),
            "MySecureP@ssw0rd!".to_string(),
        )
        .await?;
    println!("   Created user: {:?}", user);
    println!();

    // Failed registration due to weak password
    println!("2. Attempting to register with weak password:");
    match user_service
        .register_user(
            "jane_doe".to_string(),
            "jane@example.com".to_string(),
            "password".to_string(), // Weak password
        )
        .await
    {
        Ok(_) => println!("Unexpected success!"),
        Err(e) => println!("   Registration failed as expected: {}", e),
    }

    Ok(())
}
