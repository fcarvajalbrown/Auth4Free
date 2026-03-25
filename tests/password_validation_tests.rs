use authlib::password_validation::*;

#[test]
fn test_valid_password() {
    let config = PasswordValidationConfig::default();
    let result = validate_password("MySecureP@ssw0rd!", &config);
    assert!(result.is_ok());
}

#[test]
fn test_too_short_password() {
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
fn test_missing_uppercase() {
    let config = PasswordValidationConfig::default();
    let result = validate_password("nocaps123!", &config);
    assert_eq!(result, Err(PasswordValidationError::MissingUppercase));
}

#[test]
fn test_missing_lowercase() {
    let config = PasswordValidationConfig::default();
    let result = validate_password("NOCAPS123!", &config);
    assert_eq!(result, Err(PasswordValidationError::MissingLowercase));
}

#[test]
fn test_missing_number() {
    let config = PasswordValidationConfig::default();
    let result = validate_password("NoNumbers!", &config);
    assert_eq!(result, Err(PasswordValidationError::MissingNumber));
}

#[test]
fn test_missing_special_char() {
    let config = PasswordValidationConfig::default();
    let result = validate_password("NoSpecial123", &config);
    assert_eq!(result, Err(PasswordValidationError::MissingSpecialChar));
}

#[test]
fn test_common_password() {
    let config = PasswordValidationConfig::default();
    let result = validate_password("password", &config);
    assert_eq!(result, Err(PasswordValidationError::TooCommon));
}

#[test]
fn test_empty_password() {
    let config = PasswordValidationConfig::default();
    let result = validate_password("", &config);
    assert_eq!(result, Err(PasswordValidationError::EmptyPassword));
}

#[test]
fn test_consecutive_characters() {
    let config = PasswordValidationConfig {
        max_consecutive_same_char: Some(2),
        ..Default::default()
    };
    let result = validate_password("aaa123AAA!!!", &config);
    assert_eq!(result, Err(PasswordValidationError::TooManyConsecutiveChars('a', 3)));
}

#[test]
fn test_password_strength_scoring() {
    // Test various password strengths
    let very_weak = password_strength_score("password");
    let strong = password_strength_score("MySecureP@ssw0rd!");
    
    println!("'password' score: {}", very_weak);
    println!("'MySecureP@ssw0rd!' score: {}", strong);
    
    // Test that strong passwords are stronger than weak ones
    assert!(very_weak < strong, 
            "Weak password ({}) should score less than strong password ({})", 
            very_weak, strong);
    
    // Test reasonable bounds
    assert!(very_weak <= 100);
    assert!(strong <= 100);
}
