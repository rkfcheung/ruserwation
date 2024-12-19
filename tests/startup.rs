mod common;

#[cfg(test)]
mod tests {
    use ruserwation::{
        admin::{
            repo::{AdminRepo, VerifyUser},
            sqlite::SqliteAdminRepo,
        },
        setup::{errors, startup},
    };
    use std::{env, sync::Arc};

    use crate::common::db_utils::init_conn;

    #[tokio::test]
    async fn test_init() {
        // Set environment variables for testing
        env::set_var("RW_ADMIN_PASSWORD", "local_test");
        env::set_var("RW_ADMIN_USERNAME", "startup");
        env::set_var("RW_SQLITE_URL", "sqlite://local_test_init.db");

        let result: Result<_, errors::SetupError> = startup::init().await;
        assert!(result.is_ok(), "Failed to init");

        // Verify table exists
        let pool = init_conn().await.unwrap();
        let result: (i64,) = sqlx::query_as(
            "SELECT Count(1) FROM sqlite_master WHERE type='table' AND name='Admin'",
        )
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(result.0, 1, "Admin table should exist");

        let admin_repo = SqliteAdminRepo::new(Arc::new(pool));
        let root_user: Option<ruserwation::admin::models::Admin> = admin_repo.find_by_id(1).await;
        assert!(root_user.is_some());

        let verified = admin_repo.verify("startup", "local_test").await;
        assert!(verified, "Failed to verify the startup password");

        // Clean up environment variables after the test
        env::remove_var("RW_ADMIN_PASSWORD");
        env::remove_var("RW_ADMIN_USERNAME");
        env::remove_var("RW_SQLITE_URL");
        std::fs::remove_file("local_test_init.db").expect("Failed to remove test database file");
    }
}
