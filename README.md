# Auth4Free

A modern, secure, and easy-to-use authentication library for Rust applications.

[![Crates.io](https://img.shields.io/crates/v/auth4free.svg)](https://crates.io/crates/auth4free)
[![Build Status](https://github.com/fcarvajalbrown/Auth4Free/workflows/CI/badge.svg)](https://github.com/fcarvajalbrown/Auth4Free/actions)
 [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust Version](https://img.shields.io/badge/rust-1.56%2B-blue.svg)](https://www.rust-lang.org)
[![Release](https://github.com/fcarvajalbrown/auth4free/actions/workflows/release.yml/badge.svg)](https://github.com/fcarvajalbrown/auth4free/releases)

## 🚀 Features

- **Password Validation** - Robust password strength checking and validation
- **JWT Authentication** - Secure token-based authentication
- **Password Hashing** - Industry-standard bcrypt password hashing
- **User Management** - Complete user lifecycle management
- **Session Management** - Secure session handling with refresh tokens
- **Rate Limiting** - Brute force protection with configurable limits ✨ **NEW in v0.2.0**
- **Multi-Factor Authentication** - Enhanced security *(planned)*
- **OAuth2 Integration** - Social login support *(planned)*

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
auth4free = "0.2.0"
```

## 🔧 Quick Start

### Password Validation

```rust
use auth4free::password_validation::*;

let config = PasswordValidationConfig::default();
let password = "MySecureP@ssw0rd!";

match validate_password(password, &config) {
    Ok(()) => println!("Password is valid!"),
    Err(e) => println!("Password invalid: {}", e),
}

// Check password strength
let score = password_strength_score(password);
let category = password_strength_category(score);
println!("Password strength: {} ({}/100)", category, score);
```

### User Authentication with Rate Limiting

```rust
use auth4free::auth::authenticate_user;
use auth4free::rate_limiter::RateLimiter;
use auth4free::rate_limiter::models::RateLimitConfig;
use auth4free::user::User;

async fn login_example() -> Result<String, Box<dyn std::error::Error>> {
    // Setup rate limiter
    let config = RateLimitConfig::default();
    let limiter = RateLimiter::new(config);
    let user_ip = "192.168.1.1";
    
    // Check rate limit
    match limiter.check_rate_limit(user_ip).await {
        auth4free::rate_limiter::RateLimitResult::Allowed { .. } => {
            // Proceed with authentication
            let user = User::new("john_doe".to_string(), "john@example.com".to_string());
            let token = authenticate_user(user).await?;
            
            // Record successful login
            limiter.record_success(user_ip).await?;
            
            Ok(token)
        }
        auth4free::rate_limiter::RateLimitResult::Denied { .. } => {
            // Record failed attempt
            limiter.record_failure(user_ip).await?;
            Err("Rate limit exceeded".into())
        }
    }
}

```

### Session Management

```rust
use auth4free::session::SessionManager;
use uuid::Uuid;
use std::time::Duration;

async fn session_example() -> Result<(), Box<dyn std::error::Error>> {
    let session_manager = SessionManager::new();
    let user_id = Uuid::new_v4();
    
    // Create session
    let session = session_manager
        .create_session(user_id, true, Some("127.0.0.1".to_string()), None)
        .await?;
    
    // Validate session
    let validation = session_manager.validate_session(session.id).await;
    match validation {
        auth4free::session::SessionValidation::Valid(session) => {
            println!("Session valid for user: {}", session.user_id);
        }
        _ => println!("Session invalid"),
    }
    
    Ok(())
}

```

### Rate Limiting

```rust
use auth4free::rate_limiter::{RateLimiter, RateLimitConfig};
use std::time::Duration;

// Configure rate limiting
let config = RateLimitConfig {
    max_attempts: 5,           // Max 5 attempts
    window_duration: Duration::from_secs(300),  // Per 5 minutes
    lockout_duration: Duration::from_secs(900), // 15 minute lockout
    reset_on_success: true,    // Reset on successful auth
};

let limiter = RateLimiter::new(config);

// Check if IP is allowed to attempt login
match limiter.check_rate_limit("192.168.1.1").await {
    RateLimitResult::Allowed { remaining_attempts, .. } => {
        println!("{} attempts remaining", remaining_attempts);
    }
    RateLimitResult::Denied { remaining_lockout_time, .. } => {
        println!("Locked out for {:?}", remaining_lockout_time);
    }
}
```

## 🛡️ Security Features

### Password Validation Rules
- Minimum length requirements
- Uppercase/lowercase letter requirements
- Number and special character requirements
- Consecutive character limits
- Common password detection

### Password Strength Analysis
```rust
use auth4free::password_validation::*;

let passwords = vec![
    "password",                    // Very Weak
    "Password123",                 // Weak  
    "MySecureP@ssw0rd!",          // Strong
    "correct horse battery staple" // Very Strong
];

for pwd in passwords {
    let score = password_strength_score(pwd);
    let category = password_strength_category(score);
    println!("{}: {} ({}/100)", pwd, category, score);
}
```

## 📚 Examples

Check out the [examples](examples/) directory for complete working examples:

- [`password_validation`](examples/password_validation.rs) - Basic password validation
- [`user_registration`](examples/user_registration.rs) - Complete user registration flow
- [`web_api_integration`](examples/web_api_integration.rs) - Web framework integration
- [`session_integration`] [examples\session_integration.rs] - Session management with auth

Run examples with:
```bash
cargo run --example password_validation
```

## 🧪 Testing

Run all tests:
```bash
cargo test
```

## 🔮 Future Features
### Coming Soon
- Multi-Factor Authentication (MFA)
- OAuth2 Provider Integration (Google, GitHub, etc.)
- Email Verification System
- Role-Based Access Control (RBAC)
### Planned Features
- Account Lockout Mechanisms
- Refresh Token System
- Rate Limiting and Brute Force Protection (✓ Added in v0.2.0)
- Audit Logging

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE]((https://mit-license.org/)) file for details.

## 🙏 Acknowledgments

- Built with security best practices in mind
- Inspired by industry standards for authentication systems
- Development assisted by AI pair programming tools

---

*Made with ❤️ for the Rust community*