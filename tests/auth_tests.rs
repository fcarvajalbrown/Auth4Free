use authlib::auth::authenticate_user;
use authlib::user::User;

#[tokio::test]
async fn test_authenticate_user() {
    let user = User::new("john_doe".to_string(), "john@example.com".to_string());
    let result: Result<String, String> = authenticate_user(user).await;
    assert!(result.is_ok());
}
