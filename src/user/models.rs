// src/user/models.rs

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
}

impl User {
    pub fn new(username: String, email: String) -> Self {
        User {
            id: Uuid::new_v4(),
            username,
            email,
        }
    }

    pub fn validate_email(&self) -> bool {
        self.email.contains('@')
    }
}
