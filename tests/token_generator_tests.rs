use authlib::token_generator::generate_token;
use authlib::user::User;

#[tokio::test]
async fn test_generate_token() {
    let user = User::new("john_doe".to_string(), "john@example.com".to_string());
    let result: Result<String, String> = generate_token(user).await;
    assert!(result.is_ok());
}
