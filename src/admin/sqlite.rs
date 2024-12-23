use sqlx::{query, query_as, query_scalar, SqlitePool};
use std::sync::Arc;

use super::{
    models::Admin,
    repo::{AdminRepo, VerifyUser},
};

enum OpType {
    Insert,
    Update,
    NoOp,
}

pub struct SqliteAdminRepo {
    pool: Arc<SqlitePool>, // SQLite connection pool
}

impl SqliteAdminRepo {
    // Create a new repository with a database connection pool
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }

    async fn count(&self) -> Result<u32, sqlx::Error> {
        let count: u32 = query_scalar("SELECT COUNT(1) FROM Admin")
            .fetch_one(self.pool.as_ref())
            .await?;
        Ok(count)
    }

    async fn insert(&self, admin: &mut Admin) -> Result<u32, sqlx::Error> {
        let result = query(
            r#"
            INSERT INTO Admin (username, password, email, root)
            VALUES (?, ?, ?, ?);
            "#,
        )
        .bind(&admin.username)
        .bind(&admin.password)
        .bind(&admin.email)
        .bind(admin.root)
        .execute(self.pool.as_ref())
        .await?;

        admin.id = result.last_insert_rowid() as u32;
        Ok(admin.id)
    }

    async fn update(&self, admin: &Admin) -> Result<u32, sqlx::Error> {
        query(
            r#"
            UPDATE Admin
                SET password = ?, 
                    email = ?, 
                    last_login_time = CURRENT_TIMESTAMP
            WHERE id = ?;
            "#,
        )
        .bind(&admin.password)
        .bind(&admin.email)
        .bind(admin.id)
        .execute(self.pool.as_ref())
        .await?;

        Ok(admin.id)
    }

    async fn update_login_time(&self, id: u32) -> Result<(), sqlx::Error> {
        query(
            r#"
            UPDATE Admin
                SET last_login_time = CURRENT_TIMESTAMP
            WHERE id = ?;
            "#,
        )
        .bind(id)
        .execute(self.pool.as_ref())
        .await?;

        Ok(())
    }
}

impl AdminRepo for SqliteAdminRepo {
    // Find an Admin by ID
    async fn find_by_id(&self, id: u32) -> Option<Admin> {
        let result = query_as(
            r#"
            SELECT id, username, password, email, root, last_login_time
            FROM Admin
            WHERE id = ?;
            "#,
        )
        .bind(id)
        .fetch_optional(self.pool.as_ref())
        .await;

        match result {
            Ok(admin) => admin,
            Err(e) => {
                log::warn!("Error finding admin by ID: {:?}", e);
                None
            }
        }
    }

    // Find an Admin by username
    async fn find_by_username(&self, username: &str) -> Option<Admin> {
        let result = query_as(
            r#"
            SELECT id, username, password, email, root, last_login_time
            FROM Admin
            WHERE username = ?;
            "#,
        )
        .bind(username)
        .fetch_optional(self.pool.as_ref())
        .await;

        match result {
            Ok(admin) => admin,
            Err(e) => {
                log::error!("Error finding admin by username {}: {:?}", username, e);
                None
            }
        }
    }

    // Save an Admin and return its ID
    async fn save(&self, admin: &mut Admin) -> u32 {
        let op_type = if self.count().await.unwrap_or_default() == 0 {
            OpType::Insert
        } else if admin.id == 0 {
            match self.find_by_username(&admin.username).await {
                Some(found) => {
                    admin.id = found.id;
                    OpType::Update
                }
                None => OpType::Insert,
            }
        } else {
            match self.find_by_id(admin.id).await {
                Some(_) => OpType::Update,
                None => OpType::NoOp,
            }
        };

        match op_type {
            OpType::Insert => match self.insert(admin).await {
                Ok(id) => id,
                Err(e) => {
                    log::error!(
                        "Failed to insert admin with username '{}': {:?}",
                        admin.username,
                        e
                    );
                    0
                }
            },
            OpType::Update => match self.update(admin).await {
                Ok(id) => id,
                Err(e) => {
                    log::error!(
                        "Failed to update admin with username '{}': {:?}",
                        admin.username,
                        e
                    );
                    0
                }
            },
            OpType::NoOp => {
                log::warn!(
                    "Admin with ID {} not found. Save operation skipped.",
                    admin.id
                );
                0
            }
        }
    }
}

impl VerifyUser for SqliteAdminRepo {
    async fn contains(&self, username: &str) -> bool {
        self.find_by_username(username).await.is_some()
    }

    // Verify username and password
    async fn verify(&self, username: &str, password: &str) -> bool {
        if let Some(admin) = self.find_by_username(username).await {
            // Compare the password (assuming stored passwords are hashed)
            if admin.verify_password(password) {
                return self.update_login_time(admin.id).await.is_ok();
            }
        }

        false
    }
}
