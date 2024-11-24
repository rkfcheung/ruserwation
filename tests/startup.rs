#[cfg(test)]
mod tests {
    use std::env;

    use ruserwation::{
        admin::{repo::AdminRepo, sqlite::SqliteAdminRepo},
        db,
        setup::startup,
    };

    #[tokio::test]
    async fn test_init() {
        // Set environment variables for testing
        env::set_var("RW_ADMIN_PASSWORD", "local_test");
        env::set_var("RW_ADMIN_USERNAME", "startup");
        env::set_var("RW_SQLITE_URL", "sqlite://local-test-init.db");

        let result: Result<(), startup::SetupError> = startup::init().await;
        assert!(result.is_ok(), "Failed to init");

        let pool = db::sqlite::init_conn().await.unwrap();
        let admin_repo = SqliteAdminRepo::new(pool);
        let root_user = admin_repo.find_by_id(1).await;
        assert!(root_user.is_some());

        let verified = admin_repo.verify("startup", "local_test").await;
        assert!(verified, "Failed to verify the startup password");

        // Clean up environment variables after the test
        env::remove_var("RW_ADMIN_PASSWORD");
        env::remove_var("RW_ADMIN_USERNAME");
        env::remove_var("RW_SQLITE_URL");
        std::fs::remove_file("local-test-init.db").expect("Failed to remove test database file");
    }
}
