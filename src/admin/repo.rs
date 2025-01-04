use std::future::Future;

use super::models::Admin;
use crate::common::Repo;

pub trait AdminRepo: Repo<u32, Admin> {
    // Find an Admin by username
    fn find_by_username(&self, username: &str) -> impl Future<Output = Option<Admin>> + Send;
}

pub trait VerifyUser {
    // Check if user exists
    fn contains(&self, username: &str) -> impl Future<Output = bool> + Send;

    // Verify username and password
    fn verify(&self, username: &str, password: &str) -> impl Future<Output = bool> + Send;
}
