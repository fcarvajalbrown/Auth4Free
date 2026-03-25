use auth4free::auth::authenticate_user;
use auth4free::user::User;

#[tokio::test]
async fn test_authenticate_user() {
    let user = User::new("john_doe".to_string(), "john@example.com".to_string());
    let result: Result<String, String> = authenticate_user(user).await;
    assert!(result.is_ok());
}
