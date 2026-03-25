# Auth4Free

A modern, secure, and easy-to-use authentication library for Rust applications.

[![Build Status](https://github.com/fcarvajalbrown/Auth4Free/workflows/CI/badge.svg)](https://github.com/fcarvajalbrown/Auth4Free/actions)
[![License](https://img.shields.io/crates/l/authlib.svg)](LICENSE)
[![Rust Version](https://img.shields.io/badge/rust-1.56%2B-blue.svg)](https://www.rust-lang.org)

## 🚀 Features

- **Password Validation** - Robust password strength checking and validation
- **JWT Authentication** - Secure token-based authentication
- **Password Hashing** - Industry-standard bcrypt password hashing
- **User Management** - Complete user lifecycle management
- **Session Management** - Secure session handling *(coming soon)*
- **Rate Limiting** - Protection against brute force attacks *(planned)*
- **Multi-Factor Authentication** - Enhanced security *(planned)*

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
authlib = "0.1.0"
```

## 🔧 Quick Start

### Password Validation

```rust
use authlib::password_validation::*;

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

### User Authentication

```rust
use authlib::auth::authenticate_user;
use authlib::user::User;

async fn login_example() -> Result<String, String> {
    let user = User::new("john_doe".to_string(), "john@example.com".to_string());
    let token = authenticate_user(user).await?;
    Ok(token)
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
use authlib::password_validation::*;

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

Run examples with:
```bash
cargo run --example password_validation
```

## 🧪 Testing

Run all tests:
```bash
cargo test
```

## 📖 Documentation

API documentation is available at [docs.rs](https://docs.rs/authlib).

## 🔮 Future Features

### Coming Soon
- Session Management
- Refresh Token System
- Account Lockout Mechanisms

### Planned Features
- OAuth2 Provider Integration (Google, GitHub, etc.)
- Multi-Factor Authentication (TOTP, SMS, Email)
- Rate Limiting and Brute Force Protection
- Email Verification System
- Role-Based Access Control (RBAC)
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