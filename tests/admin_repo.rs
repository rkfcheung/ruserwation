mod common;

#[cfg(test)]
mod tests {
    use ruserwation::admin::{
        models::Admin,
        repo::{AdminRepo, EnableSession},
        sqlite::SqliteAdminRepo,
    };
    use std::sync::Arc;

    use crate::common::db_utils;

    #[tokio::test]
    async fn test_sqlite_admin_repo() {
        // Create an instance
        let pool = db_utils::init_test_db()
            .await
            .expect("Failed to create test DB!");
        let repo = SqliteAdminRepo::new(Arc::new(pool));

        // Create an Admin
        let mut admin = Admin::builder()
            .id(1)
            .username("admin")
            .password("password123")
            .email("admin@localhost")
            .build();

        // Save the Admin
        let saved_id = repo.save(&mut admin).await;

        // Verify that the saved ID is correct
        assert_eq!(saved_id, admin.id);
        assert_eq!(saved_id, 1);

        // Retrieve the Admin by ID
        let found_admin = repo.find_by_id(1).await.unwrap();
        assert_eq!(found_admin.id, 1);

        // Retrieve the Admin by username
        let mut found_admin = repo.find_by_username("admin").await.unwrap();
        assert_eq!(found_admin.username, admin.username);
        assert_eq!(found_admin.email, admin.email);
        assert!(found_admin.root);

        // Verify the password
        assert!(admin.verify_password("password123"));
        assert!(repo.verify("admin", "password123").await); // Ensure you check against the hashed password

        // Create a session for the admin
        let session_id = repo.create_session("admin").await.unwrap();

        // Update the email for the admin
        found_admin.email = "admin-new@localhost".to_string();

        // Save the updated admin back to the repository
        repo.save(&mut found_admin).await;
        let updated = repo.find_by_username("admin").await.unwrap();
        assert_eq!(updated.email, found_admin.email);
        assert!(updated.last_login_time.is_some());

        // Retrieve the session
        let session = repo.get_session(&session_id).await.unwrap();
        assert!(!session.is_expired());
        assert_eq!(session.get_raw("user").unwrap(), admin.username);

        // Destroy the session
        repo.destroy_session(&session_id).await;
        assert!(repo.get_session(&session_id).await.is_err()); // Ensure session is removed
    }
}
