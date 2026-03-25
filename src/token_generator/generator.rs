// src/token_generator/generator.rs

use crate::user::models::User;
use jsonwebtoken::{encode, EncodingKey, Header};

pub async fn generate_token(user: User) -> Result<String, String> {
    encode(
        &Header::default(),
        &user,
        &EncodingKey::from_secret("secret".as_ref()),
    )
    .map_err(|e| e.to_string())
}
