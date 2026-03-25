// src/password_hasher/hasher.rs

use bcrypt::{self, BcryptError};

pub async fn hash_password(password: &str) -> Result<String, BcryptError> {
    // Use proper cost instead of generating salt manually
    bcrypt::hash(password, 10)
}
