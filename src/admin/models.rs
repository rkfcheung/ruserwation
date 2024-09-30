use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use chrono::NaiveDateTime;
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
}
