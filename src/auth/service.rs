// src/auth/service.rs

use super::models::*;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json;

pub async fn authenticate_user(user: User) -> Result<String, String> {
    // Simulate authentication logic
    let token = encode(
        &Header::default(),
        &user,
        &EncodingKey::from_secret("secret".as_ref()),
    )
    .map_err(|_| "Failed to generate token")?;
    Ok(token)
}
