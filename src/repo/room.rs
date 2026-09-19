use crate::model::Room;
use sqlx::Error;
use sqlx::SqlitePool;
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

        sqlx::query("INSERT INTO rooms (id, name, max_members) VALUES (?, ?, ?)")
            .bind(id)
            .bind(name)
            .bind(max_members)
            .execute(&self.pool)
            .await?;

        Ok(Room {
            id,
            name: name.to_string(),
            max_members,
        })
    }
}
