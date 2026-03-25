//! Example showing integration with web frameworks (Axum/Tide style)
//!
//! This demonstrates how to integrate password validation into
//! a web API handler.

use auth4free::password_validation::*;

// Mock web framework types
type HttpResponse = String;
type _HttpRequest = String;

struct AuthHandler {
    password_config: PasswordValidationConfig,
}

impl AuthHandler {
    fn new() -> Self {
        Self {
            password_config: PasswordValidationConfig {
                min_length: 8,
                require_uppercase: true,
                require_lowercase: true,
                require_numbers: true,
                require_special_chars: false, // More lenient for web apps
                max_consecutive_same_char: Some(3),
            },
        }
    }

    fn register_user_handler(&self, json_body: &str) -> HttpResponse {
        // Parse JSON (simplified)
        let request_data = parse_json(json_body);
        let username = request_data
            .get("username")
            .unwrap_or(&String::new())
            .clone();
        let email = request_data.get("email").unwrap_or(&String::new()).clone();
        let password = request_data
            .get("password")
            .unwrap_or(&String::new())
            .clone();

        if username.is_empty() || email.is_empty() || password.is_empty() {
            return r#"{"error": "Missing required fields"}"#.to_string();
        }

        // Validate password
        match validate_password(&password, &self.password_config) {
            Ok(()) => {
                // Proceed with registration (simplified)
                format!(
                    r#"{{"success": true, "message": "User {} registered successfully"}}"#,
                    username
                )
            }
            Err(e) => {
                // Return validation error
                format!(r#"{{"error": "{}"}}"#, e)
            }
        }
    }

    fn change_password_handler(&self, json_body: &str) -> HttpResponse {
        let request_data = parse_json(json_body);
        let _old_password = request_data
            .get("old_password")
            .unwrap_or(&String::new())
            .clone();
        let new_password = request_data
            .get("new_password")
            .unwrap_or(&String::new())
            .clone();

        if new_password.is_empty() {
            return r#"{"error": "New password is required"}"#.to_string();
        }

        // Validate new password
        match validate_password(&new_password, &self.password_config) {
            Ok(()) => {
                let score = password_strength_score(&new_password);
                let category = password_strength_category(score);

                format!(
                    r#"{{"success": true, "strength": "{}", "score": {}}}"#,
                    category, score
                )
            }
            Err(e) => {
                format!(r#"{{"error": "{}"}}"#, e)
            }
        }
    }
}

// Simple JSON parser mock
fn parse_json(json_str: &str) -> std::collections::HashMap<String, String> {
    // This is a very simplified mock - real implementation would use serde_json
    let mut map = std::collections::HashMap::new();

    // Extract key-value pairs (very basic parsing)
    if json_str.contains("\"username\"") {
        map.insert("username".to_string(), "testuser".to_string());
    }
    if json_str.contains("\"email\"") {
        map.insert("email".to_string(), "test@example.com".to_string());
    }
    if json_str.contains("\"password\"") {
        map.insert("password".to_string(), "MySecureP@ssw0rd!".to_string());
    }
    if json_str.contains("\"old_password\"") {
        map.insert("old_password".to_string(), "oldpass123".to_string());
    }
    if json_str.contains("\"new_password\"") {
        map.insert("new_password".to_string(), "NewSecureP@ssw0rd!".to_string());
    }

    map
}

fn main() {
    println!("=== Web API Integration Examples ===\n");

    let auth_handler = AuthHandler::new();

    // Registration example
    println!("1. User Registration API Call:");
    let register_request =
        r#"{"username": "testuser", "email": "test@example.com", "password": "MySecureP@ssw0rd!"}"#;
    let response = auth_handler.register_user_handler(register_request);
    println!("   Request: {}", register_request);
    println!("   Response: {}", response);
    println!();

    // Change password example
    println!("2. Change Password API Call:");
    let change_pass_request =
        r#"{"old_password": "oldpass123", "new_password": "NewSecureP@ssw0rd!"}"#;
    let response = auth_handler.change_password_handler(change_pass_request);
    println!("   Request: {}", change_pass_request);
    println!("   Response: {}", response);
    println!();

    // Failed registration example
    println!("3. Failed Registration (Weak Password):");
    let bad_register_request =
        r#"{"username": "baduser", "email": "bad@example.com", "password": "password"}"#;
    let response = auth_handler.register_user_handler(bad_register_request);
    println!("   Request: {}", bad_register_request);
    println!("   Response: {}", response);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_web_api_integration() {
        let handler = AuthHandler::new();

        // Test successful registration
        let request =
            r#"{"username": "test", "email": "test@test.com", "password": "MySecureP@ssw0rd!"}"#;
        let response = handler.register_user_handler(request);
        assert!(response.contains("success"));

        // Test failed registration
        let bad_request =
            r#"{"username": "test", "email": "test@test.com", "password": "password"}"#;
        let response = handler.register_user_handler(bad_request);
        assert!(response.contains("error"));
    }
}
