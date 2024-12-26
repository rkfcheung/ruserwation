mod common;

#[cfg(test)]
mod tests {
    use chrono::NaiveDateTime;
    use ruserwation::admin::models::Admin;
    use ruserwation::admin::{
        repo::{AdminRepo, VerifyUser},
        sessions::{EnableSession, SessionManager},
        sqlite::SqliteAdminRepo,
    };
    use ruserwation::common::Repo;
    use std::sync::Arc;

    use crate::common::db_utils;

    #[tokio::test]
    async fn test_sqlite_admin_repo() {
        // Create an instance
        let pool = db_utils::init_test_db()
            .await
            .expect("Failed to create test DB!");
        let admin_repo = Arc::new(SqliteAdminRepo::new(pool.into()));
        let session_manager = SessionManager::new(admin_repo.clone());
        let repo = &admin_repo.as_ref();

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
        let session_id = session_manager.create_session("admin").await.unwrap();

        // Update the email for the admin
        found_admin.email = "admin-new@localhost".to_string();

        // Save the updated admin back to the repository
        repo.save(&mut found_admin).await;
        let updated = repo.find_by_username("admin").await.unwrap();
        assert_eq!(updated.email, found_admin.email);
        assert!(updated.last_login_time.is_some());

        // Retrieve the session
        let session = session_manager.get_session(&session_id).await.unwrap();
        assert!(!session.is_expired());
        assert_eq!(session.get::<String>("user").unwrap(), admin.username);

        // Destroy the session
        session_manager.destroy_session(&session_id).await;
        assert!(session_manager.get_session(&session_id).await.is_err()); // Ensure session is removed
    }

    #[tokio::test]
    async fn test_verify_with_update_login_time() {
        // Initialise an in-memory SQLite database
        let pool = db_utils::init_test_db()
            .await
            .expect("Failed to create test DB!");

        // Create the repository
        let repo = SqliteAdminRepo::new(pool.into());

        // Insert a test admin
        let mut admin = Admin::builder()
            .username("testuser")
            .password("hashed_password")
            .email("test@example.com")
            .build();
        let id = repo.save(&mut admin).await;

        // Verify with correct username and password
        let result = repo.verify("testuser", "hashed_password").await; // Use the correct hashed password
        assert!(result);

        // Check if `last_login_time` was updated
        let last_login_time: Option<NaiveDateTime> = repo
            .find_by_id(id)
            .await
            .and_then(|admin| admin.last_login_time);
        assert!(
            last_login_time.is_some(),
            "last_login_time should be updated after successful login"
        );

        // Verify with incorrect password
        let result = repo.verify("testuser", "wrong_password").await;
        assert!(!result, "Verification should fail with incorrect password");

        // Check if `last_login_time` was not updated
        let login_time_after_failed_login: Option<NaiveDateTime> = repo
            .find_by_username(&admin.username)
            .await
            .and_then(|admin| admin.last_login_time);
        assert_eq!(
            login_time_after_failed_login, last_login_time,
            "last_login_time should not be updated after failed login"
        );

        // Verify with non-existent username
        let result = repo.verify("nonexistent", "hashed_password").await;
        assert!(
            !result,
            "Verification should fail with non-existent username"
        );
    }
}
