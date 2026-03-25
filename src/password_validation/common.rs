//! Common password detection utilities

use std::collections::HashSet;

/// Checks if a password is in the list of commonly used passwords
pub fn is_common_password(password: &str) -> bool {
    let common_passwords: HashSet<&str> = [
        "password",
        "123456",
        "qwerty",
        "admin",
        "welcome",
        "password123",
        "abc123",
        "letmein",
        "monkey",
        "dragon",
        "master",
        "mustang",
        "shadow",
        "baseball",
        "donald",
        "superman",
        "harley",
        "12345678",
        "qazwsx",
        "princess",
    ]
    .iter()
    .cloned()
    .collect();

    common_passwords.contains(&password.to_lowercase().as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_common_passwords() {
        assert!(is_common_password("password"));
        assert!(is_common_password("123456"));
        assert!(!is_common_password("MySecureP@ssw0rd!"));
    }
}
