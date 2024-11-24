use std::env;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use log::{debug, warn};
use rand::{distributions::Alphanumeric, Rng};

use crate::utils::env_util::truncate_string;

const PWD_DEF_LEN: usize = 16;
const PWD_MAX_LEN: usize = 32;
const PWD_MIN_LEN: usize = 8;
const VAR_LEN: usize = 256;

pub fn hash_password(password: &str) -> Vec<u8> {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    let password = validate_password(&password);

    argon2
        .hash_password(password.as_bytes(), &salt)
        .expect("Failed to hash password")
        .to_string()
        .into_bytes()
}

pub fn generate_random_password(n: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(n.max(PWD_MIN_LEN))
        .map(char::from)
        .collect()
}

pub fn validate_email(email: &str) -> String {
    if email.is_empty() {
        "admin@localhost".to_string()
    } else {
        truncate_string(email, VAR_LEN)
    }
}

pub fn validate_password(password: &str) -> String {
    if password.len() < PWD_MIN_LEN {
        let admin_pwd_len = match env::var("RW_ADMIN_PWD_LEN") {
            Ok(value) => match value.parse::<usize>() {
                Ok(parsed_len) => parsed_len,
                Err(_) => {
                    debug!(
                        "Invalid RW_ADMIN_PWD_LEN, using default value of {}.",
                        PWD_DEF_LEN
                    );
                    PWD_DEF_LEN
                }
            },
            Err(_) => {
                debug!(
                    "RW_ADMIN_PWD_LEN not set, using default value of {}.",
                    PWD_DEF_LEN
                );
                PWD_DEF_LEN
            }
        }
        .min(PWD_MAX_LEN);

        let random_password = generate_random_password(admin_pwd_len);
        warn!(
            "Admin password not set. Generated random password: {}",
            random_password
        );
        random_password
    } else {
        truncate_string(password, PWD_MAX_LEN)
    }
}

pub fn validate_username(username: &str) -> String {
    if username.is_empty() {
        "admin".to_string()
    } else {
        truncate_string(username, VAR_LEN)
    }
}
