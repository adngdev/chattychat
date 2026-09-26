mod database;
mod model;
mod repo;
mod routes;
mod state;

use crate::repo::{room::RoomRepo, user::UserRepo};
use state::AppState;

#[tokio::main]
async fn main() {
    let pool = database::connect().await;
    let state = AppState {
        rooms: RoomRepo::new(pool.clone()),
        users: UserRepo::new(pool),
    };

    let app = routes::router().with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
