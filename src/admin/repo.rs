use std::future::Future;

use super::models::Admin;

pub trait AdminRepo {
    // Find an Admin by id
    fn find_by_id(&self, id: u32) -> impl Future<Output = Option<Admin>> + Send;

    // Find an Admin by username
    fn find_by_username(&self, username: &str) -> impl Future<Output = Option<Admin>> + Send;

    // Save an Admin and return its ID
    fn save(&self, admin: &mut Admin) -> impl Future<Output = u32> + Send;

    // Verify username and password
    fn verify(&self, username: &str, password: &str) -> impl Future<Output = bool> + Send;
}
