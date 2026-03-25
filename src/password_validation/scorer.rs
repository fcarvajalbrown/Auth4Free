/// Calculates password strength score (0-100)
pub fn password_strength_score(password: &str) -> u32 {
    let mut score = 0u32;

    // Length bonus (more conservative)
    score += password.len() as u32 * 2; // Reduced from *4
    if password.len() >= 8 {
        score += 5; // Reduced from 10
    }
    if password.len() >= 12 {
        score += 5; // Reduced from 10
    }

    // Character variety bonuses
    let mut has_lower = false;
    let mut has_upper = false;
    let mut has_digit = false;
    let mut has_symbol = false;
    let mut digit_count = 0;
    let mut symbol_count = 0;

    for ch in password.chars() {
        if ch.is_lowercase() {
            has_lower = true;
        }
        if ch.is_uppercase() {
            has_upper = true;
        }
        if ch.is_numeric() {
            has_digit = true;
            digit_count += 1;
        }
        if !ch.is_alphanumeric() {
            has_symbol = true;
            symbol_count += 1;
        }
    }

    if has_lower {
        score += 10;
    }
    if has_upper {
        score += 10;
    }
    if has_digit {
        score += 10;
    }
    if has_symbol {
        score += 15;
    }

    // Bonus for digits and symbols in middle
    if digit_count > 1 {
        let bonus = (digit_count - 1) * 2;
        score += bonus as u32;
    }
    if symbol_count > 1 {
        let bonus = (symbol_count - 1) * 2;
        score += bonus as u32;
    }

    // Penalties
    let chars: Vec<char> = password.chars().collect();

    // Sequential letters penalty
    for window in chars.windows(3) {
        let first = window[0] as u32;
        let second = window[1] as u32;
        let third = window[2] as u32;

        if second == first + 1 && third == second + 1 {
            score = score.saturating_sub(15);
        }
    }

    // Sequential numbers penalty
    for window in chars.windows(3) {
        if window[0].is_ascii_digit() && window[1].is_ascii_digit() && window[2].is_ascii_digit() {
            let first = window[0] as u32;
            let second = window[1] as u32;
            let third = window[2] as u32;

            if second == first + 1 && third == second + 1 {
                score = score.saturating_sub(15);
            }
        }
    }

    // Major penalty for common passwords
    if crate::password_validation::common::is_common_password(password) {
        score = score.saturating_sub(50); // Heavy penalty for common passwords
    }

    score.clamp(0, 100)
}

/// Returns password strength category
pub fn password_strength_category(score: u32) -> &'static str {
    match score {
        0..=30 => "Very Weak",
        31..=50 => "Weak",
        51..=70 => "Medium",
        71..=85 => "Strong",
        86..=100 => "Very Strong",
        _ => "Unknown", // This handles scores > 100, though they shouldn't occur
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_strength() {
        // Test various password strengths
        let trivial = password_strength_score("a");
        let very_weak = password_strength_score("aaaaaa");
        let weak = password_strength_score("password123");
        let medium = password_strength_score("Password123");
        let strong = password_strength_score("MySecureP@ssw0rd!");

        println!("'a' score: {}", trivial);
        println!("'aaaaaa' score: {}", very_weak);
        println!("'password123' score: {}", weak);
        println!("'Password123' score: {}", medium);
        println!("'MySecureP@ssw0rd!' score: {}", strong);

        // Test that clearly weak passwords score lower than clearly strong ones
        assert!(
            trivial < strong,
            "Trivial password should be weaker than strong password"
        );
        assert!(
            weak < strong,
            "Weak password should be weaker than strong password"
        );

        // Test reasonable bounds
        assert!(
            trivial <= 20,
            "Trivial password should be very weak (got {})",
            trivial
        );
        assert!(
            strong >= 80,
            "Strong password should score high (got {})",
            strong
        );
    }

    #[test]
    fn test_password_categories() {
        assert_eq!(password_strength_category(15), "Very Weak");
        assert_eq!(password_strength_category(40), "Weak");
        assert_eq!(password_strength_category(60), "Medium");
        assert_eq!(password_strength_category(80), "Strong");
        assert_eq!(password_strength_category(95), "Very Strong");
    }
}
