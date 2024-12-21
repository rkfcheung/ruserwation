use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
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
    let validated_password = validate_password(password);

    argon2
        .hash_password(validated_password.as_bytes(), &salt)
        .expect("Failed to hash password")
        .to_string()
        .into_bytes()
}

/// Generates a random password with a given length.
/// Ensures the password meets the minimum length requirement.
pub fn generate_random_password(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length.clamp(PWD_MIN_LEN, PWD_MAX_LEN))
        .map(char::from)
        .collect()
}

/// Validates and sanitises an email address.
/// Returns a default email (`admin@localhost`) if invalid.
pub fn validate_email(email: &str) -> String {
    let sanitised_email = remove_whitespace(email);

    if is_valid_email(&sanitised_email) {
        truncate_string(&sanitised_email, VAR_LEN)
    } else {
        log::warn!("Invalid email provided. Using default email: admin@localhost");
        "admin@localhost".to_string()
    }
}

/// Validates and sanitises a password.
/// Generates a random password if the provided password is invalid.
pub fn validate_password(password: &str) -> String {
    let sanitised_password = remove_whitespace(password);

    if sanitised_password.len() < PWD_MIN_LEN {
        // Get custom password length from the environment variable, falling back to default.
        let admin_pwd_len = env::var("RW_ADMIN_PWD_LEN")
            .ok()
            .and_then(|value| value.parse::<usize>().ok())
            .unwrap_or_else(|| {
                log::debug!(
                    "RW_ADMIN_PWD_LEN not set or invalid. Using default value: {}",
                    PWD_DEF_LEN
                );
                PWD_DEF_LEN
            })
            .clamp(PWD_MIN_LEN, PWD_MAX_LEN);

        // Generate and log a random password
        let random_password = generate_random_password(admin_pwd_len);
        log::warn!(
            "Invalid or missing admin password. Generated random password: {}",
            random_password
        );
        random_password
    } else {
        truncate_string(&sanitised_password, PWD_MAX_LEN)
    }
}

/// Validates and sanitises a username.
/// Returns a default username (`admin`) if invalid.
pub fn validate_username(username: &str) -> String {
    let sanitised_username = remove_whitespace(username);

    if is_valid_username(&sanitised_username) {
        truncate_string(&sanitised_username, VAR_LEN)
    } else {
        log::warn!("Invalid username provided. Using default username: admin");
        "admin".to_string()
    }
}

/// Checks if an email address is valid.
fn is_valid_email(email: &str) -> bool {
    // Allow the special default email for convenience
    if email == "admin@localhost" {
        return true;
    }

    // Compile the regex safely
    let email_regex = regex::Regex::new(r"^[\w\.-]+@[\w\.-]+\.\w+$");
    match email_regex {
        Ok(regex) => {
            if regex.is_match(email) {
                true
            } else {
                log::warn!("Invalid email format: {}", email);
                false
            }
        }
        Err(e) => {
            // Log an error if regex compilation fails
            log::warn!("Failed to compile email regex: {}", e);
            false
        }
    }
}

/// Checks if a username is valid.
/// Username must be 3-32 characters long, alphanumeric, and may include `_`.
fn is_valid_username(username: &str) -> bool {
    let username_regex = regex::Regex::new(r"^[a-zA-Z0-9_]{3,32}$");
    match username_regex {
        Ok(regex) => {
            if regex.is_match(username) {
                true
            } else {
                log::warn!(
                    "Invalid username: '{}' (must be 3-32 alphanumeric characters, or `_`)",
                    username
                );
                false
            }
        }
        Err(e) => {
            // Log an error if regex compilation fails
            log::warn!("Failed to compile username regex: {}", e);
            false
        }
    }
}
