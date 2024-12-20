#[cfg(test)]
mod tests {
    use ruserwation::admin::sessions::Sessions;
    use std::thread;
    use std::time::Duration;

    // Helper function to create a mock session for testing
    fn setup_sessions() -> Sessions {
        Sessions::default()
    }

    #[tokio::test]
    async fn test_create_session() {
        let sessions = setup_sessions();
        let username = "test_user";

        // Test session creation
        let session_id = sessions.create(username).await;
        assert!(session_id.is_ok(), "Session ID should be returned");

        let session_id = session_id.unwrap();
        let retrieved_session = sessions.get(&session_id).await;
        assert!(retrieved_session.is_ok(), "Session should be retrievable");

        let session = retrieved_session.unwrap();
        let value: String = session.get::<String>("user").unwrap();
        assert_eq!(value, username, "Session should store the correct username");
    }

    #[tokio::test]
    async fn test_create_with_expire_in() {
        let sessions = setup_sessions();
        let username = "test_user";
        let expire_in = Duration::from_secs(2); // 2 seconds

        // Test session creation with custom expiration
        let session_id = sessions.create_with_expire_in(username, expire_in).await;
        assert!(session_id.is_ok(), "Session ID should be returned");

        let session_id = session_id.unwrap();
        let session = sessions.get(&session_id).await;
        assert!(session.is_ok(), "Session should be retrievable immediately");

        // Wait for the session to expire
        thread::sleep(Duration::from_secs(3));
        let expired_session = sessions.get(&session_id).await;
        assert!(
            expired_session.is_err(),
            "Session should be expired and no longer retrievable"
        );
    }

    #[tokio::test]
    async fn test_destroy_session() {
        let sessions = setup_sessions();
        let username = "test_user";

        // Create a session
        let session_id = sessions.create(username).await;
        assert!(session_id.is_ok(), "Session ID should be returned");

        let session_id = session_id.unwrap();
        let session = sessions.get(&session_id).await;
        assert!(session.is_ok(), "Session should exist before destroy");

        // Destroy the session
        let result = sessions.destroy(&session_id).await;
        let session_after_destroy = sessions.get(&session_id).await;
        assert!(result, "Session should be destroyed");
        assert!(
            session_after_destroy.is_err(),
            "Session should not exist after being destroyed"
        );
    }

    #[tokio::test]
    async fn test_get_nonexistent_session() {
        let sessions = setup_sessions();

        // Attempt to get a nonexistent session
        let session_id = "invalid_id";
        let session = sessions.get(session_id).await;
        assert!(
            session.is_err(),
            "Nonexistent session should not be retrievable"
        );
    }

    #[tokio::test]
    async fn test_session_expiry_removal() {
        let sessions = setup_sessions();
        let username = "test_user";

        // Create a session that expires quickly
        let session_id = sessions
            .create_with_expire_in(username, Duration::from_secs(1))
            .await;
        assert!(session_id.is_ok(), "Session ID should be returned");

        let session_id = session_id.unwrap();
        thread::sleep(Duration::from_secs(2));

        // Access the session after expiry
        let session = sessions.get(&session_id).await;
        assert!(
            session.is_err(),
            "Expired session should be removed and not retrievable"
        );
    }
}
