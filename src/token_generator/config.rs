// src/token_generator/config.rs

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TokenConfig {
    pub secret: String,
}
