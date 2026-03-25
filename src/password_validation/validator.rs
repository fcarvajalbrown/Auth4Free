//! Password validation logic

use std::collections::HashSet;
use crate::password_validation::errors::PasswordValidationError;

/// Configuration for password validation rules
#[derive(Debug, Clone)]
pub struct PasswordValidationConfig {
    pub min_length: usize,
    pub require_uppercase: bool,
    pub require_lowercase: bool,
    pub require_numbers: bool,
    pub require_special_chars: bool,
    pub max_consecutive_same_char: Option<usize>,
}

impl Default for PasswordValidationConfig {
    fn default() -> Self {
        Self {
            min_length: 8,
            require_uppercase: true,
            require_lowercase: true,
            require_numbers: true,
            require_special_chars: true,
            max_consecutive_same_char: Some(3),
        }
    }
}

/// Validates a password against the given configuration
pub fn validate_password(
    password: &str,
    config: &PasswordValidationConfig,
) -> Result<(), PasswordValidationError> {
    if password.is_empty() {
        return Err(PasswordValidationError::EmptyPassword);
    }

    // Check length
    if password.len() < config.min_length {
        return Err(PasswordValidationError::TooShort(
            password.len(),
            config.min_length,
        ));
    }

    // Check character requirements
    let mut has_uppercase = false;
    let mut has_lowercase = false;
    let mut has_number = false;
    let mut has_special = false;

    for ch in password.chars() {
        if ch.is_uppercase() {
            has_uppercase = true;
        }
        if ch.is_lowercase() {
            has_lowercase = true;
        }
        if ch.is_numeric() {
            has_number = true;
        }
        if !ch.is_alphanumeric() {
            has_special = true;
        }
    }

    if config.require_uppercase && !has_uppercase {
        return Err(PasswordValidationError::MissingUppercase);
    }

    if config.require_lowercase && !has_lowercase {
        return Err(PasswordValidationError::MissingLowercase);
    }

    if config.require_numbers && !has_number {
        return Err(PasswordValidationError::MissingNumber);
    }

    if config.require_special_chars && !has_special {
        return Err(PasswordValidationError::MissingSpecialChar);
    }

    // Check consecutive characters
    if let Some(max_consecutive) = config.max_consecutive_same_char {
        let mut current_char = '\0';
        let mut current_count = 0;

        for ch in password.chars() {
            if ch == current_char {
                current_count += 1;
                if current_count > max_consecutive {
                    return Err(PasswordValidationError::TooManyConsecutiveChars(
                        ch,
                        current_count,
                    ));
                }
            } else {
                current_char = ch;
                current_count = 1;
            }
        }
    }

    // Check against common passwords
    if is_common_password(password) {
        return Err(PasswordValidationError::TooCommon);
    }

    Ok(())
}

/// Checks if a password is in the list
fn is_common_password(password: &str) -> bool {
    // In a real implementation, this would check against a comprehensive list of common passwords
    let common_passwords: HashSet<&str> = [
        "password", "123456", "123456789", "qwerty", "abc123", "letmein", "monkey", "dragon",
    ]
    .iter()
    .cloned()
    .collect();

    common_passwords.contains(password)
}   

#[cfg(test)]
mod tests {
    use super::*;
    use crate::password_validation::errors::PasswordValidationError;

    #[test]
    fn test_valid_password_internal() {
        let config = PasswordValidationConfig::default();
        let result = validate_password("MySecureP@ssw0rd!", &config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_too_short_password_internal() {
        let config = PasswordValidationConfig {
            min_length: 12,
            ..Default::default()
        };
        let result = validate_password("short", &config);
        assert_eq!(
            result,
            Err(PasswordValidationError::TooShort(5, 12))
        );
    }

    #[test]
    fn test_missing_uppercase_internal() {
        let config = PasswordValidationConfig::default();
        let result = validate_password("nocaps123!", &config);
        assert_eq!(result, Err(PasswordValidationError::MissingUppercase));
    }

    #[test]
    fn test_common_password_internal() {
        let config = PasswordValidationConfig::default();
        let result = validate_password("password", &config);
        assert_eq!(result, Err(PasswordValidationError::TooCommon));
    }
}
