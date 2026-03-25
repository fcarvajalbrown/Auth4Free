// src/tests/auth_tests.rs

use super::auth::service::*;
use super::user::models::*;

#[tokio::test]
async fn test_authenticate_user() {
    let user = User::new("john_doe".to_string(), "john@example.com".to_string());
    let result = authenticate_user(user).await;
    assert!(result.is_ok());
}
