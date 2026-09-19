use axum::{Router, routing::get};

mod database;
mod model;
mod state;

use state::AppState;

#[tokio::main]
async fn main() {
    let db = database::connect().await;
    let state = AppState { db };

    let app = Router::new()
        .route("/", get(|| async { "hello from the server" }))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
