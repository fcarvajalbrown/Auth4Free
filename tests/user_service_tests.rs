use authlib::user::{create_user, get_user_by_id, User};
use uuid::Uuid;

#[tokio::test]
async fn test_create_user() {
    let user = User::new("john_doe".to_string(), "john@example.com".to_string());
    let result: String = create_user(user).await;
    assert!(!result.is_empty());
}

#[tokio::test]
async fn test_get_user_by_id() {
    let id = Uuid::new_v4();
    let user_option = get_user_by_id(id).await;
    assert!(user_option.is_some());
}
