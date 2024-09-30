mod restaurant;

use argon2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Argon2,
};
use rand::{distributions::Alphanumeric, Rng};
use restaurant::models::Restaurant;
use ruserwation::admin::models::Admin;
use std::str::from_utf8;

fn main() {
    let rand_pwd: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect();
    let admin = Admin::new(
        1,
        "admin".to_string(),
        rand_pwd.clone(),
        "admin@localhost".to_string(),
    );
    println!("Admin Random Password: {}", rand_pwd);
    println!("{:?}", admin);
    let parsed_hash = PasswordHash::new(from_utf8(&admin.password).unwrap()).unwrap();
    assert!(Argon2::default()
        .verify_password(rand_pwd.as_bytes(), &parsed_hash)
        .is_ok());

    let restaurant = Restaurant {
        id: 1,
        name: "Hello World".to_string(),
        max_capacity: 64,
        location: "0x0".to_string(),
        active: true,
    };
    println!("{:?}", restaurant);
}
