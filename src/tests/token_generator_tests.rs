// src/tests/token_generator_tests.rs

use super::token_generator::generator::*;
use super::user::models::*;

#[tokio::test]
async fn test_generate_token() {
    let user = User::new("john_doe".to_string(), "john@example.com".to_string());
    let result = generate_token(user).await;
    assert!(result.is_ok());
}
