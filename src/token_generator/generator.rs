// src/token_generator/generator.rs

use super::models::*;
use jsonwebtoken::{decode, EncodingKey, Header};
use serde_json;

pub async fn generate_token(user: User) -> String {
    // Simulate token generation logic
    encode(
        &Header::default(),
        &user,
        &EncodingKey::from_secret("secret".as_ref()),
    )
    .unwrap()
}
