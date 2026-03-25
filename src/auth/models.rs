// src/auth/models.rs

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthClaims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
}
