use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use log::{debug, warn};
use rand::{distributions::Alphanumeric, Rng};
use std::env;

use crate::utils::env_util::{remove_whitespace, truncate_string};

const PWD_DEF_LEN: usize = 16; // Default password length
const PWD_MAX_LEN: usize = 32; // Maximum allowed password length
const PWD_MIN_LEN: usize = 8; // Minimum allowed password length
const VAR_LEN: usize = 256; // Maximum allowed length for variables like username/email

/// Hashes a given password using Argon2.
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

/// Generates a random password with a given length.
/// Ensures the password meets the minimum length requirement.
pub fn generate_random_password(n: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(n.max(PWD_MIN_LEN))
        .map(char::from)
        .collect()
}

/// Validates and sanitizes an email address.
/// Returns a default email (`admin@localhost`) if invalid.
pub fn validate_email(email: &str) -> String {
    let sanitized_email = remove_whitespace(email);
    if is_valid_email(&sanitized_email) {
        truncate_string(&sanitized_email, VAR_LEN)
    } else {
        warn!("Invalid email provided. Using default email: admin@localhost");
        "admin@localhost".to_string()
    }
}

/// Validates and sanitizes a password.
/// Generates a random password if the provided password is invalid.
pub fn validate_password(password: &str) -> String {
    let sanitized_password = remove_whitespace(password);
    if sanitized_password.len() < PWD_MIN_LEN {
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
    let username = if is_valid_username(username) {
        &remove_whitespace(username)
    } else {
        ""
    };
    if username.is_empty() {
        "admin".to_string()
    } else {
        truncate_string(username, VAR_LEN)
    }
}

fn is_valid_email(email: &str) -> bool {
    if "admin@localhost" == email {
        return true;
    }
    let email_regex = regex::Regex::new(r"^[\w\.-]+@[\w\.-]+\.\w+$").unwrap();
    if email_regex.is_match(email) {
        true
    } else {
        warn!("Invalid email format.");
        false
    }
}

fn is_valid_username(username: &str) -> bool {
    let username_regex = regex::Regex::new(r"^[a-zA-Z0-9_-]{3,32}$").unwrap();
    if username_regex.is_match(username) {
        true
    } else {
        warn!("Invalid username: Must be 3-32 characters, alphanumeric, and contain only _ or -.");
        false
    }
}
