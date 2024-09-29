use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::app::enums::ReservationStatus;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Admin {
    pub id: u32,
    pub username: String,
    pub password: Vec<u8>,
    pub email: String,
    pub root: bool,
    pub last_login_time: Option<NaiveDateTime>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Customer {
    pub id: u32,
    pub email: String,
    pub name: String,
    pub phone: String,
    pub last_reservation_id: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Reservation {
    pub id: u32,
    pub restaurant_id: u32,
    pub customer_id: u32,
    pub table_size: u8,
    pub reservation_time: NaiveDateTime,
    pub notes: Option<String>,
    pub status: ReservationStatus,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Restaurant {
    pub id: u32,
    pub name: String,
    pub max_capacity: u32,
    pub location: String,
    pub active: bool,
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
