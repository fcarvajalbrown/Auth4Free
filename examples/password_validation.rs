//! Example demonstrating password validation features
//!
//! This example shows how to use the password validation module
//! to validate user passwords and provide feedback on strength.

use auth4free::password_validation::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Password Validation Examples ===\n");

    // Example 1: Basic password validation with default settings
    println!("1. Basic Password Validation");
    let config = PasswordValidationConfig::default();
    let password = "MySecureP@ssw0rd!";

    match validate_password(password, &config) {
        Ok(()) => println!("✅ Password '{}' is valid", password),
        Err(e) => println!("❌ Password invalid: {}", e),
    }

    // Show password strength
    let score = password_strength_score(password);
    let category = password_strength_category(score);
    println!("   Strength: {} ({}/100)", category, score);
    println!();

    // Example 2: Validating weak passwords
    println!("2. Testing Weak Passwords");
    let weak_passwords = vec![
        "password",
        "123456",
        "short",
        "nouppercase123!",
        "NOLOWERCASE123!",
        "NoNumbers!",
        "NoSpecialChars123",
    ];

    for pwd in weak_passwords {
        match validate_password(pwd, &config) {
            Ok(()) => println!("✅ '{}' is valid", pwd),
            Err(e) => println!("❌ '{}': {}", pwd, e),
        }
        let score = password_strength_score(pwd);
        let category = password_strength_category(score);
        println!("   Strength: {} ({}/100)", category, score);
        println!();
    }

    // Example 3: Custom validation configuration
    println!("3. Custom Validation Rules");
    let custom_config = PasswordValidationConfig {
        min_length: 12,
        require_uppercase: true,
        require_lowercase: true,
        require_numbers: false, // Don't require numbers
        require_special_chars: true,
        max_consecutive_same_char: Some(2), // Max 2 consecutive same chars
    };

    let custom_password = "MyCustomPass!";
    match validate_password(custom_password, &custom_config) {
        Ok(()) => println!("✅ Custom password '{}' is valid", custom_password),
        Err(e) => println!("❌ Custom password invalid: {}", e),
    }
    println!();

    // Example 4: Password strength analysis
    println!("4. Password Strength Analysis");
    let test_passwords = vec![
        "password",
        "Password123",
        "MySecureP@ssw0rd!",
        "correct horse battery staple",
        "Tr0ub4dor&3",
    ];

    for pwd in test_passwords {
        let score = password_strength_score(pwd);
        let category = password_strength_category(score);
        println!("Password: '{}' -> {} ({}/100)", pwd, category, score);
    }

    Ok(())
}
