use sqlx::{query, query_as, SqlitePool};

use super::{models::Admin, repo::AdminRepo};

pub struct SqliteAdminRepo {
    pool: SqlitePool, // SQLite connection pool
}

impl SqliteAdminRepo {
    // Create a new repository with a database connection pool
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    async fn insert(&mut self, admin: &mut Admin) -> u32 {
        let result = query(
            r#"
            INSERT INTO Admin (username, password, email, root, last_login_time)
            VALUES (?, ?, ?, ?, ?);
            "#,
        )
        .bind(&admin.username)
        .bind(&admin.password)
        .bind(&admin.email)
        .bind(admin.root)
        .bind(admin.last_login_time)
        .execute(&self.pool)
        .await;

        match result {
            Ok(r) => {
                let id = r.last_insert_rowid() as u32;
                admin.id = id;

                id
            } // Return the inserted ID
            _ => 0, // Return 0 if there's an error
        }
    }

    async fn update(&mut self, admin: &Admin) -> u32 {
        let result = query(
            r#"
            UPDATE Admin
                SET username = ?, 
                    password = ?, 
                    email = ?, 
                    last_login_time = ?
            WHERE id = ?;
            "#,
        )
        .bind(&admin.username)
        .bind(&admin.password)
        .bind(&admin.email)
        .bind(admin.last_login_time)
        .bind(admin.id)
        .execute(&self.pool)
        .await;

        match result {
            Ok(_) => admin.id,
            _ => 0,
        }
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
        .fetch_optional(&self.pool)
        .await;

        match result {
            Ok(admin) => admin, // Return found Admin
            _ => None,          // Return None if not found or if there's an error
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
        .fetch_optional(&self.pool)
        .await;

        match result {
            Ok(admin) => admin, // Return found Admin
            _ => None,          // Return None if not found or if there's an error
        }
    }

    // Save an Admin and return its ID
    async fn save(&mut self, admin: &mut Admin) -> u32 {
        if admin.id == 0 {
            return self.insert(admin).await;
        }

        let found = self.find_by_id(admin.id).await;
        match found {
            Some(_) => self.update(admin).await,
            _ => 0,
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
