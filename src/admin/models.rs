use std::str::from_utf8;

use argon2::{Argon2, PasswordHash, PasswordVerifier};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::utils::env_util::{var_as_str, var_as_str_or};

use super::helper::{hash_password, validate_email, validate_username};

#[derive(Clone, Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct Admin {
    pub id: u32,
    pub username: String,
    pub password: Vec<u8>,
    pub email: String,
    pub root: bool,
    #[sqlx(default)]
    pub last_login_time: Option<NaiveDateTime>,
}

impl Admin {
    pub fn new(id: u32, username: String, password: String, email: String) -> Self {
        let username = validate_username(&username);
        let password = hash_password(&password);
        let email = validate_email(&email);

        Self {
            id,
            username,
            password,
            email,
            root: id == 1,
            last_login_time: None,
        }
    }

    pub fn init() -> Self {
        let email = var_as_str_or("RW_ADMIN_EMAIL", "admin@localhost".to_string());
        let password = var_as_str("RW_ADMIN_PASSWORD");
        let username = var_as_str_or("RW_ADMIN_USERNAME", "admin".to_string());

        Self::new(1, username, password, email)
    }

    pub fn verify_password(&self, password: &str) -> bool {
        if let Ok(hash_str) = from_utf8(&self.password) {
            if let Ok(parsed_hash) = PasswordHash::new(hash_str) {
                return Argon2::default()
                    .verify_password(password.as_bytes(), &parsed_hash)
                    .is_ok();
            }
        }
        false
    }
}
