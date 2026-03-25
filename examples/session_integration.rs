//! Example demonstrating session integration with authentication
//!
//! This example shows how to combine password validation, user authentication,
//! and session management in a complete authentication flow.

use auth4free::auth::authenticate_user;
use auth4free::password_validation::*;
use auth4free::session::SessionManager;
use auth4free::user::User;
use std::time::Duration;
use uuid::Uuid;

/// Main authentication service that combines all components
pub struct AuthService {
    session_manager: SessionManager,
    password_config: PasswordValidationConfig,
}

impl AuthService {
    pub fn new() -> Self {
        Self {
            session_manager: SessionManager::new(),
            password_config: PasswordValidationConfig::default(),
        }
    }

    /// Register a new user with password validation
    pub async fn register_user(
        &self,
        username: String,
        email: String,
        password: String,
    ) -> Result<RegistrationResult, AuthError> {
        // Validate password strength
        validate_password(&password, &self.password_config)
            .map_err(|e| AuthError::PasswordValidation(e.to_string()))?;

        // In a real app, you'd save the user to a database here
        let user = User::new(username, email);

        // Hash password (in real app)
        // let hashed_password = hash_password(&password).await?;

        println!("✅ User '{}' registered successfully", user.username);

        Ok(RegistrationResult {
            user_id: user.id,
            message: "User registered successfully".to_string(),
        })
    }

    /// Authenticate user and create session
    pub async fn login(
        &self,
        email: String,
        _password: String,
        remember_me: bool,
        ip_address: Option<String>,
    ) -> Result<LoginResult, AuthError> {
        // In a real app, you'd verify credentials against database
        // For demo, we'll simulate successful authentication

        // Create a mock user (in real app, fetch from database)
        let user = User::new("demo_user".to_string(), email.clone());

        // Simulate authentication
        let _token = authenticate_user(user.clone())
            .await
            .map_err(|e| AuthError::AuthenticationFailed(e))?;

        // Create session
        let session = self
            .session_manager
            .create_session(
                user.id,
                remember_me,
                ip_address,
                Some("Demo Client".to_string()),
            )
            .await
            .map_err(|e| AuthError::SessionError(e.to_string()))?;

        println!("✅ User '{}' logged in successfully", user.username);

        Ok(LoginResult {
            session_id: session.id,
            user_id: session.user_id,
            refresh_token: session.refresh_token.clone(), // Clone instead of move
            expires_in: session
                .time_remaining()
                .unwrap_or(Duration::from_secs(0))
                .as_secs(),
        })
    }

    /// Validate an existing session
    pub async fn validate_session(
        &self,
        session_id: Uuid,
    ) -> Result<SessionValidationResult, AuthError> {
        let validation = self.session_manager.validate_session(session_id).await;

        match validation {
            auth4free::session::SessionValidation::Valid(session) => {
                // Use the session parameter
                Ok(SessionValidationResult::Valid(SessionInfo {
                    session_id: session.id,
                    user_id: session.user_id,
                    expires_in: session
                        .time_remaining()
                        .unwrap_or(Duration::from_secs(0))
                        .as_secs(),
                    is_persistent: session.is_persistent,
                }))
            }
            auth4free::session::SessionValidation::Expired => Ok(SessionValidationResult::Expired),
            auth4free::session::SessionValidation::Invalid => Ok(SessionValidationResult::Invalid),
        }
    }

    /// Logout and revoke session
    pub async fn logout(&self, session_id: Uuid) -> Result<(), AuthError> {
        self.session_manager
            .revoke_session(session_id)
            .await
            .map_err(|e| AuthError::SessionError(e.to_string()))?;

        println!("✅ Session revoked successfully");
        Ok(())
    }

    /// Get all active sessions for a user
    pub async fn get_user_sessions(&self, user_id: Uuid) -> Result<Vec<SessionInfo>, AuthError> {
        let sessions = self.session_manager.get_user_sessions(user_id).await;

        let session_infos = sessions
            .into_iter()
            .map(|session| SessionInfo {
                session_id: session.id,
                user_id: session.user_id,
                expires_in: session
                    .time_remaining()
                    .unwrap_or(Duration::from_secs(0))
                    .as_secs(),
                is_persistent: session.is_persistent,
            })
            .collect();

        Ok(session_infos)
    }
}

/// Registration result
#[derive(Debug)]
pub struct RegistrationResult {
    pub user_id: Uuid,
    pub message: String,
}

/// Login result
#[derive(Debug)]
pub struct LoginResult {
    pub session_id: Uuid,
    pub user_id: Uuid,
    pub refresh_token: String,
    pub expires_in: u64, // seconds until expiration
}

/// Session information
#[derive(Debug)]
pub struct SessionInfo {
    pub session_id: Uuid,
    pub user_id: Uuid,
    pub expires_in: u64,
    pub is_persistent: bool,
}

/// Session validation result
#[derive(Debug)]
pub enum SessionValidationResult {
    Valid(SessionInfo),
    Expired,
    Invalid,
}

/// Authentication errors
#[derive(Debug)]
pub enum AuthError {
    PasswordValidation(String),
    AuthenticationFailed(String),
    SessionError(String),
    UserNotFound,
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthError::PasswordValidation(msg) => write!(f, "Password validation error: {}", msg),
            AuthError::AuthenticationFailed(msg) => write!(f, "Authentication failed: {}", msg),
            AuthError::SessionError(msg) => write!(f, "Session error: {}", msg),
            AuthError::UserNotFound => write!(f, "User not found"),
        }
    }
}

impl std::error::Error for AuthError {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Auth4Free Session Integration Example ===\n");

    let auth_service = AuthService::new();

    // 1. Register a new user
    println!("1. Registering new user:");
    match auth_service
        .register_user(
            "john_doe".to_string(),
            "john@example.com".to_string(),
            "MySecureP@ssw0rd!".to_string(),
        )
        .await
    {
        Ok(result) => println!("   Registered user ID: {}", result.user_id),
        Err(e) => println!("   Registration failed: {}", e),
    }
    println!();

    // 2. Login and create session
    println!("2. Logging in user:");
    let login_result = auth_service
        .login(
            "john@example.com".to_string(),
            "MySecureP@ssw0rd!".to_string(),
            true,                            // remember me
            Some("192.168.1.1".to_string()), // IP address
        )
        .await?;

    println!("   Session created: {}", login_result.session_id);
    println!("   Expires in: {} seconds", login_result.expires_in);
    println!("   Refresh token: {}", login_result.refresh_token);
    println!();

    // 3. Validate session
    println!("3. Validating session:");
    let validation_result = auth_service
        .validate_session(login_result.session_id)
        .await?;

    match validation_result {
        SessionValidationResult::Valid(info) => {
            println!("   Session is valid!");
            println!("   User ID: {}", info.user_id);
            println!("   Expires in: {} seconds", info.expires_in);
        }
        SessionValidationResult::Expired => println!("   Session has expired"),
        SessionValidationResult::Invalid => println!("   Session is invalid"),
    }
    println!();

    // 4. Get user sessions
    println!("4. Getting user sessions:");
    let user_sessions = auth_service.get_user_sessions(login_result.user_id).await?;

    println!("   Found {} active sessions", user_sessions.len());
    for session in user_sessions {
        println!(
            "   - Session: {} (expires in {}s)",
            session.session_id, session.expires_in
        );
    }
    println!();

    // 5. Logout
    println!("5. Logging out:");
    auth_service.logout(login_result.session_id).await?;

    // 6. Try to validate revoked session
    println!("6. Validating revoked session:");
    let validation_result = auth_service
        .validate_session(login_result.session_id)
        .await?;

    match validation_result {
        SessionValidationResult::Valid(_) => println!("   Session is still valid (unexpected!)"),
        SessionValidationResult::Expired => println!("   Session has expired"),
        SessionValidationResult::Invalid => println!("   Session is invalid (revoked)"),
    }

    println!("\n✅ All authentication flows completed successfully!");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_complete_auth_flow() {
        let auth_service = AuthService::new();

        // Test registration
        let registration_result = auth_service
            .register_user(
                "test_user".to_string(),
                "test@example.com".to_string(),
                "MySecureP@ssw0rd!".to_string(),
            )
            .await
            .expect("Registration should succeed");

        assert!(!registration_result.user_id.is_nil());

        // Test login
        let login_result = auth_service
            .login(
                "test@example.com".to_string(),
                "MySecureP@ssw0rd!".to_string(),
                false,
                Some("127.0.0.1".to_string()),
            )
            .await
            .expect("Login should succeed");

        assert!(!login_result.session_id.is_nil());

        // Test session validation
        let validation_result = auth_service
            .validate_session(login_result.session_id)
            .await
            .expect("Session validation should succeed");

        match validation_result {
            SessionValidationResult::Valid(_) => assert!(true),
            _ => panic!("Session should be valid"),
        }

        // Test logout
        auth_service
            .logout(login_result.session_id)
            .await
            .expect("Logout should succeed");

        // Test that session is now invalid
        let validation_result = auth_service
            .validate_session(login_result.session_id)
            .await
            .expect("Session validation should succeed");

        match validation_result {
            SessionValidationResult::Invalid => assert!(true),
            _ => panic!("Session should be invalid after logout"),
        }
    }

    #[tokio::test]
    async fn test_password_validation_integration() {
        let auth_service = AuthService::new();

        // Test weak password rejection
        let result = auth_service
            .register_user(
                "bad_user".to_string(),
                "bad@example.com".to_string(),
                "password".to_string(), // Weak password
            )
            .await;

        assert!(result.is_err());
        match result.unwrap_err() {
            AuthError::PasswordValidation(_) => assert!(true),
            _ => panic!("Should get password validation error"),
        }
    }
}
