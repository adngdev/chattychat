use sqlx::SqlitePool;
use sqlx::sqlite::SqlitePoolOptions;

pub async fn connect() -> SqlitePool {
    let db = SqlitePoolOptions::new()
        .connect("sqlite://chat.db?mode=rwc")
        .await
        .unwrap();

    sqlx::migrate!().run(&db).await.unwrap();

    db
}
