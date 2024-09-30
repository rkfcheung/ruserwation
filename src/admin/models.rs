use std::{env, str::from_utf8};

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};
use chrono::NaiveDateTime;
use log::{debug, warn};
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Admin {
    pub id: u32,
    pub username: String,
    pub password: Vec<u8>,
    pub email: String,
    pub root: bool,
    pub last_login_time: Option<NaiveDateTime>,
}

impl Admin {
    pub fn new(id: u32, username: String, password: String, email: String) -> Self {
        let argon2 = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .expect("Failed to hash password")
            .to_string();

        Admin {
            id,
            username,
            password: password_hash.into_bytes(),
            email,
            root: id == 1,
            last_login_time: None,
        }
    }

    pub fn init() -> Self {
        let admin_email = env::var("RW_ADMIN_EMAIL").unwrap_or("admin@localhost".to_string());
        let admin_password = env::var("RW_ADMIN_PASSWORD").unwrap_or_default();
        let admin_username = env::var("RW_ADMIN_USERNAME").unwrap_or("admin".to_string());

        let password = if admin_password.len() < 4 {
            let admin_pwd_len = match env::var("RW_ADMIN_PWD_LEN") {
                Ok(val) => match val.parse::<usize>() {
                    Ok(parsed_len) => parsed_len,
                    Err(_) => {
                        debug!("Invalid RW_ADMIN_PWD_LEN, using default value of 16.");
                        16
                    }
                },
                Err(_) => {
                    debug!("RW_ADMIN_PWD_LEN not set, using default value of 16.");
                    16
                }
            };

            let random_password = Self::generate_random_password(admin_pwd_len);
            warn!(
                "Admin password not set. Generated random password: {}",
                random_password
            );
            random_password
        } else {
            admin_password
        };

        Admin::new(1, admin_username, password.clone(), admin_email)
    }

    pub fn generate_random_password(n: usize) -> String {
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(n.max(8))
            .map(char::from)
            .collect()
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
