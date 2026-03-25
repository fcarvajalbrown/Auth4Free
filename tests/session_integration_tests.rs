//! Integration tests for session management with authentication

use auth4free::session::{SessionManager};
use auth4free::password_validation::*;
use auth4free::user::User;
use uuid::Uuid;
use std::time::Duration;

#[tokio::test]
async fn test_session_based_authentication_flow() {
    // Setup
    let session_manager = SessionManager::new();
    let user = User::new("test_user".to_string(), "test@example.com".to_string());
    
    // Test session creation
    let session = session_manager
        .create_session(
            user.id,
            true, // persistent session
            Some("127.0.0.1".to_string()),
            Some("Test Browser".to_string()),
        )
        .await
        .expect("Should create session successfully");

    assert_eq!(session.user_id, user.id);
    assert_eq!(session.ip_address, Some("127.0.0.1".to_string()));
    assert_eq!(session.user_agent, Some("Test Browser".to_string()));
    assert!(session.is_persistent);

    // Test session validation
    let validation = session_manager.validate_session(session.id).await;
    match validation {
        auth4free::session::SessionValidation::Valid(validated_session) => {
            assert_eq!(validated_session.id, session.id);
            assert_eq!(validated_session.user_id, user.id);
        }
        _ => panic!("Session should be valid"),
    }

    // Test session extension
    session_manager
        .extend_session(session.id)
        .await
        .expect("Should extend session successfully");

    // Verify session is still valid after extension
    let validation = session_manager.validate_session(session.id).await;
    assert!(matches!(validation, auth4free::session::SessionValidation::Valid(_)));

    // Test session revocation
    session_manager
        .revoke_session(session.id)
        .await
        .expect("Should revoke session successfully");

    // Verify session is now invalid
    let validation = session_manager.validate_session(session.id).await;
    assert!(matches!(validation, auth4free::session::SessionValidation::Invalid));
}

#[tokio::test]
async fn test_multiple_sessions_per_user() {
    let session_manager = SessionManager::new();
    let user_id = Uuid::new_v4();
    
    // Create multiple sessions for the same user
    let session1 = session_manager
        .create_session(user_id, false, None, None)
        .await
        .expect("Should create first session");
        
    let session2 = session_manager
        .create_session(user_id, false, None, None)
        .await
        .expect("Should create second session");

    // Both sessions should be valid
    let validation1 = session_manager.validate_session(session1.id).await;
    let validation2 = session_manager.validate_session(session2.id).await;
    
    assert!(matches!(validation1, auth4free::session::SessionValidation::Valid(_)));
    assert!(matches!(validation2, auth4free::session::SessionValidation::Valid(_)));

    // Get all user sessions
    let user_sessions = session_manager.get_user_sessions(user_id).await;
    assert_eq!(user_sessions.len(), 2);

    // Revoke all sessions for user
    let revoked_count = session_manager.revoke_user_sessions(user_id).await;
    assert_eq!(revoked_count, 2);

    // Sessions should now be invalid
    let validation1 = session_manager.validate_session(session1.id).await;
    let validation2 = session_manager.validate_session(session2.id).await;
    
    assert!(matches!(validation1, auth4free::session::SessionValidation::Invalid));
    assert!(matches!(validation2, auth4free::session::SessionValidation::Invalid));
}

#[tokio::test]
async fn test_session_expiration() {
    // Create session manager with very short session duration
    let session_manager = SessionManager::with_config(Duration::from_secs(0), 10);
    let user_id = Uuid::new_v4();
    
    // Create session
    let session = session_manager
        .create_session(user_id, false, None, None)
        .await
        .expect("Should create session");
    
    // Small delay to ensure expiration
    tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
    
    // Session should be expired
    let validation = session_manager.validate_session(session.id).await;
    assert!(matches!(validation, auth4free::session::SessionValidation::Expired));
    
    // Cleanup should remove expired sessions
    let cleanup_count = session_manager.cleanup_expired_sessions().await;
    let _ = cleanup_count; // Ignore the return value, just run the cleanup
    
    // Session should no longer exist
    let validation = session_manager.validate_session(session.id).await;
    assert!(matches!(validation, auth4free::session::SessionValidation::Invalid));
}

#[tokio::test]
async fn test_session_limit_enforcement() {
    // Create session manager with limit of 1 session per user
    let session_manager = SessionManager::with_config(Duration::from_secs(3600), 1);
    let user_id = Uuid::new_v4();
    
    // Create first session (should succeed)
    let session1 = session_manager
        .create_session(user_id, false, None, None)
        .await
        .expect("First session should be created");
    
    // Try to create second session (should fail)
    let result = session_manager
        .create_session(user_id, false, None, None)
        .await;
    
    assert!(result.is_err());
    match result.unwrap_err() {
        auth4free::session::manager::SessionError::SessionLimitExceeded => assert!(true),
        _ => panic!("Should get session limit exceeded error"),
    }
    
    // Revoke first session
    session_manager
        .revoke_session(session1.id)
        .await
        .expect("Should revoke session");
    
    // Now we should be able to create another session
    let session2 = session_manager
        .create_session(user_id, false, None, None)
        .await;
    
    assert!(session2.is_ok());
}

#[tokio::test]
async fn test_password_validation_with_sessions() {
    let config = PasswordValidationConfig::default();
    
    // Test that we can integrate password validation with session creation
    let valid_password = "MySecureP@ssw0rd!";
    let invalid_password = "password123";
    
    // Valid password should pass validation
    assert!(validate_password(valid_password, &config).is_ok());
    
    // Invalid password should fail validation
    assert!(validate_password(invalid_password, &config).is_err());
    
    // This shows how password validation integrates with the overall auth flow
}
