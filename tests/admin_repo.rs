#[cfg(test)]
mod tests {
    use ruserwation::admin::{
        models::Admin,
        repo::{AdminRepo, InMemoryAdminRepo},
    };

    #[test]
    fn test_in_memory_admin_repo() {
        // Create an instance of InMemoryAdminRepo
        let mut repo = InMemoryAdminRepo::init();

        // Create an Admin
        let admin = Admin::new(
            1,
            "admin".to_string(),
            "password123".to_string(),
            "admin@localhost".to_string(),
        );

        // Save the Admin
        let saved_id = repo.save(admin.clone());

        // Verify that the saved ID is correct
        assert_eq!(saved_id, admin.id);

        // Retrieve the Admin by username
        let found_admin = repo.find_by_username("admin").unwrap();
        assert_eq!(found_admin.username, admin.username);
        assert_eq!(found_admin.email, admin.email);
        assert!(found_admin.root);

        // Verify the password
        assert!(admin.verify_password("password123"));
        assert!(repo.verify("admin", "password123")); // Ensure you check against the hashed password

        // Create a session for the admin
        let session_id = repo.create_session("admin");

        // Retrieve the session
        let session = repo.get_session(&session_id).unwrap();
        assert_eq!(session.get_raw("user").unwrap(), admin.username);

        // Destroy the session
        repo.destroy_session(&session_id);
        assert!(repo.get_session(&session_id).is_none()); // Ensure session is removed
    }
}
