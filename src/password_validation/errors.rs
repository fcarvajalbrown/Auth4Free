//! Password validation error types

/// Password validation errors
#[derive(Debug, PartialEq)]
pub enum PasswordValidationError {
    TooShort(usize, usize), // (actual, minimum)
    MissingUppercase,
    MissingLowercase,
    MissingNumber,
    MissingSpecialChar,
    TooManyConsecutiveChars(char, usize),
    TooCommon,
    EmptyPassword,
}

impl std::fmt::Display for PasswordValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PasswordValidationError::TooShort(actual, min) => {
                write!(f, "Password too short: {} characters, minimum required: {}", actual, min)
            }
            PasswordValidationError::MissingUppercase => {
                write!(f, "Password must contain at least one uppercase letter")
            }
            PasswordValidationError::MissingLowercase => {
                write!(f, "Password must contain at least one lowercase letter")
            }
            PasswordValidationError::MissingNumber => {
                write!(f, "Password must contain at least one number")
            }
            PasswordValidationError::MissingSpecialChar => {
                write!(f, "Password must contain at least one special character")
            }
            PasswordValidationError::TooManyConsecutiveChars(ch, count) => {
                write!(f, "Too many consecutive '{}' characters: {}", ch, count)
            }
            PasswordValidationError::TooCommon => {
                write!(f, "Password is too common and easily guessable")
            }
            PasswordValidationError::EmptyPassword => {
                write!(f, "Password cannot be empty")
            }
        }
    }
}

impl std::error::Error for PasswordValidationError {}
