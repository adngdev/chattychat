use crate::model::Room;
use chrono::Utc;
use sqlx::{Error, SqlitePool};
use uuid::Uuid;

#[derive(Clone)]
pub struct RoomRepo {
    pool: SqlitePool,
}

impl RoomRepo {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, name: &str, max_members: u32) -> Result<Room, Error> {
        let id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query("INSERT INTO rooms (id, name, max_members, created_at, updated_at) VALUES (?, ?, ?, ?, ?)")
            .bind(id)
            .bind(name)
            .bind(max_members)
            .bind(now)
            .bind(now)
            .execute(&self.pool)
            .await?;

        Ok(Room {
            id,
            name: name.to_string(),
            max_members,
            created_at: now,
            updated_at: now,
        })
    }

    pub async fn list(&self, limit: u32, page: u32) -> Result<Vec<Room>, Error> {
        let offset = (page - 1) * limit;

        sqlx::query_as::<_, Room>(
            "SELECT id, name, max_members, created_at, updated_at
            FROM rooms
            ORDER BY created_at DESC
            LIMIT ? OFFSET ?",
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await
    }

    pub async fn count(&self) -> Result<u32, Error> {
        let (total,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM rooms")
            .fetch_one(&self.pool)
            .await?;

        Ok(total as u32)
    }
}
