// src/tests/user_service_tests.rs

use super::user::models::*;
use serde_json;

#[tokio::test]
async fn test_create_user() {
    let user = User::new("john_doe".to_string(), "john@example.com".to_string());
    let result = create_user(user).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_user_by_id() {
    let id = Uuid::new_v4();
    let user = get_user_by_id(id).await.unwrap();
    assert_eq!(user.id, id);
}
