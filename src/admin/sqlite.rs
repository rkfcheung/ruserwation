use sqlx::{Pool, Sqlite};
use std::error::Error;

use super::models::Admin;

pub struct SqliteAdminRepo {}

pub async fn add_admin(pool: &Pool<Sqlite>, admin: Admin) -> Result<(), Box<dyn Error>> {
    sqlx::query(
        r#"
        INSERT INTO Admin (username, password, email, root, last_login_time)
        VALUES (?1, ?2, ?3, ?4, ?5)
        "#,
    )
    .bind(admin.username)
    .bind(admin.password)
    .bind(admin.email)
    .bind(admin.root)
    .bind(admin.last_login_time.map(|dt| dt.to_string()))
    .execute(pool)
    .await?;

    Ok(())
}
