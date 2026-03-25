//! Password strength scoring utilities

/// Calculates password strength score (0-100)
pub fn password_strength_score(password: &str) -> u32 {
    let mut score = 0u32;

    // Length bonus
    score += (password.len() * 4) as u32;
    if password.len() >= 8 {
        score += 10;
    }
    if password.len() >= 12 {
        score += 10;
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
        let weak_score = password_strength_score("password");
        let strong_score = password_strength_score("MySecureP@ssw0rd!");
        
        assert!(weak_score <= 30);
        assert!(strong_score >= 80);
    }

    #[test]
    fn test_password_categories() {
        assert_eq!(password_strength_category(25), "Very Weak");
        assert_eq!(password_strength_category(45), "Weak");
        assert_eq!(password_strength_category(65), "Medium");
        assert_eq!(password_strength_category(80), "Strong");
        assert_eq!(password_strength_category(95), "Very Strong");
        
        // Test edge cases
        assert_eq!(password_strength_category(0), "Very Weak");
        assert_eq!(password_strength_category(30), "Very Weak");
        assert_eq!(password_strength_category(31), "Weak");
        assert_eq!(password_strength_category(100), "Very Strong");
    }
}
