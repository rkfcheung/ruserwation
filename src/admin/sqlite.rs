use log::{error, warn};
use sqlx::{query, query_as, SqlitePool};

use super::{
    models::Admin,
    repo::{AdminRepo, Sessions},
};

pub struct SqliteAdminRepo<'conn> {
    pool: &'conn SqlitePool, // SQLite connection pool
    sessions: Sessions,      // Session ID to Session mapping
}

impl<'conn> SqliteAdminRepo<'conn> {
    // Create a new repository with a database connection pool
    pub fn new(pool: &'conn SqlitePool) -> Self {
        Self {
            pool,
            sessions: Sessions::new(),
        }
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
        .execute(self.pool)
        .await?;

        admin.id = result.last_insert_rowid() as u32;
        Ok(admin.id)
    }

    async fn update(&self, admin: &Admin) -> Result<u32, sqlx::Error> {
        let result = query(
            r#"
            UPDATE Admin
                SET password = ?, 
                    email = ?, 
                    last_login_time = ?
            WHERE id = ?;
            "#,
        )
        .bind(&admin.password)
        .bind(&admin.email)
        .bind(admin.last_login_time)
        .bind(admin.id)
        .execute(self.pool)
        .await?;

        Ok(admin.id)
    }
}

impl<'conn> AdminRepo for SqliteAdminRepo<'conn> {
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
        .fetch_optional(self.pool)
        .await;

        match result {
            Ok(admin) => admin,
            Err(e) => {
                warn!("Error finding admin by ID: {:?}", e);
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
        .fetch_optional(self.pool)
        .await;

        match result {
            Ok(admin) => admin,
            Err(e) => {
                error!(
                    "Database error while finding admin by username {}: {:?}",
                    username, e
                );
                None
            }
        }
    }

    // Save an Admin and return its ID
    async fn save(&mut self, admin: &mut Admin) -> u32 {
        if admin.id == 0 {
            match self.find_by_username(&admin.username).await {
                Some(found) => {
                    admin.id = found.id;
                    self.update(admin).await.unwrap_or(0)
                }
                None => self.insert(admin).await.unwrap_or(0),
            }
        } else {
            match self.find_by_id(admin.id).await {
                Some(_) => self.update(admin).await.unwrap_or(0),
                None => 0,
            }
        }
    }

    // Verify username and password
    async fn verify(&self, username: &str, password: &str) -> bool {
        if let Some(admin) = self.find_by_username(username).await {
            // Compare the password (assuming stored passwords are hashed)
            admin.verify_password(password)
        } else {
            false // Return false if no admin was found
        }
    }
}
