#[cfg(test)]
mod tests {
    use ruserwation::db::sqlite::init_db;
    use std::env;

    #[tokio::test]
    async fn test_max_connections_with_tasks() {
        env::set_var("RW_SQLITE_MAX_CONN", "3");
        env::set_var("RW_SQLITE_URL", "sqlite::memory:");

        let pool = init_db().await.expect("Failed to create pool");

        let mut tasks = Vec::new();

        for _ in 0..5 {
            let pool_clone = pool.clone();
            tasks.push(tokio::spawn(async move {
                let conn = pool_clone
                    .acquire()
                    .await
                    .expect("Failed to acquire connection");

                assert!(pool_clone.size() > 0);

                tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
                drop(conn); // Release connection
            }));
        }

        for task in tasks {
            task.await.expect("Task panicked");
        }

        // Cleanup
        env::remove_var("RW_SQLITE_MAX_CONN");
        env::remove_var("RW_SQLITE_URL");
    }

    #[tokio::test]
    async fn test_with_in_memory_sqlite() {
        env::set_var("RW_SQLITE_URL", "sqlite::memory:");

        let pool = init_db().await.expect("Failed to initialize the database");

        // Test query
        let result: (i32,) = sqlx::query_as("SELECT 1")
            .fetch_one(&pool)
            .await
            .expect("Failed to execute test query");

        assert_eq!(result.0, 1);

        // Cleanup
        env::remove_var("RW_SQLITE_URL");
    }
}
