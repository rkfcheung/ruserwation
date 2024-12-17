use argon2::{Argon2, PasswordHash, PasswordVerifier};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::str::from_utf8;

use super::helper::{hash_password, validate_email, validate_username};
use crate::utils::env_util::{var_as_str, var_as_str_or};

const ROOT_ADMIN_ID: u32 = 1;

/// Represents an admin user in the system.
#[derive(Clone, Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct Admin {
    /// The ID of the admin.
    pub id: u32,
    /// The username of the admin.
    pub username: String,
    /// The hashed password of the admin.
    pub password: Vec<u8>,
    /// The email of the admin.
    pub email: String,
    /// Whether the admin has root privileges.
    pub root: bool,
    /// The last login time of the admin.
    #[sqlx(default)]
    pub last_login_time: Option<NaiveDateTime>,
}

/// A builder for creating `Admin` instances.
#[derive(Default)]
pub struct AdminBuilder {
    id: u32,
    username: String,
    password: Vec<u8>,
    email: String,
}

impl Admin {
    pub fn builder() -> AdminBuilder {
        AdminBuilder::default()
    }

    /// Creates a new `Admin` instance with validated and processed inputs.
    pub fn new(id: u32, username: String, password: String, email: String) -> Self {
        Self::builder()
            .id(id)
            .username(&username)
            .password(&password)
            .email(&email)
            .build()
    }

    /// Initialises the root admin from environment variables.
    pub fn init() -> Self {
        let email = var_as_str_or("RW_ADMIN_EMAIL", "admin@localhost".to_string());
        let password = var_as_str("RW_ADMIN_PASSWORD");
        let username = var_as_str_or("RW_ADMIN_USERNAME", "admin".to_string());

        Self::new(ROOT_ADMIN_ID, username, password, email)
    }

    /// Verifies a plaintext password against the stored hashed password.
    pub fn verify_password(&self, password: &str) -> bool {
        if let Ok(hash_str) = from_utf8(&self.password) {
            if let Ok(parsed_hash) = PasswordHash::new(hash_str) {
                return Argon2::default()
                    .verify_password(password.as_bytes(), &parsed_hash)
                    .is_ok();
            } else {
                log::warn!("Invalid password hash format for user {}", self.username);
            }
        } else {
            log::warn!(
                "Invalid UTF-8 in stored password for user {}",
                self.username
            );
        }
        false
    }
}

impl AdminBuilder {
    /// Builds and returns the `Admin` instance.
    pub fn build(self) -> Admin {
        Admin {
            id: self.id,
            username: self.username,
            password: self.password,
            email: self.email,
            root: self.id == ROOT_ADMIN_ID,
            last_login_time: None,
        }
    }

    /// Sets the email for the admin after validating it.
    pub fn email(mut self, email: &str) -> Self {
        self.email = validate_email(email);
        self
    }

    /// Sets the ID for the admin.
    pub fn id(mut self, id: u32) -> Self {
        self.id = id;
        self
    }

    /// Sets the password for the admin after hashing it.
    pub fn password(mut self, password: &str) -> Self {
        self.password = hash_password(password);
        self
    }

    /// Sets the username for the admin after validating it.
    pub fn username(mut self, username: &str) -> Self {
        self.username = validate_username(username);
        self
    }
}

#[derive(Deserialize, Serialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}
