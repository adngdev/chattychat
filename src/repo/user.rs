use crate::model::User;
use chrono::Utc;
use sqlx::{Error, SqlitePool};
use uuid::Uuid;

#[derive(Clone)]
pub struct UserRepo {
    pool: SqlitePool,
}

impl UserRepo {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, username: &str, name: &str, password: &str) -> Result<User, Error> {
        let id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query(
            "INSERT INTO users (id, username, name, password, created_at) VALUES (?, ?, ?, ?, ?)",
        )
        .bind(id)
        .bind(username)
        .bind(name)
        .bind(password)
        .bind(now)
        .execute(&self.pool)
        .await?;

        Ok(User {
            id,
            username: username.to_string(),
            name: name.to_string(),
            password: password.to_string(),
            created_at: now,
        })
    }
}
