use crate::{model::Room, state::AppState};
use axum::{Json, Router, extract::State, routing::post};
use serde::Deserialize;

pub fn router() -> Router<AppState> {
    Router::new().route("/rooms", post(create_room))
}

#[derive(Deserialize)]
struct CreateRoomRequest {
    name: String,
    max_members: u32,
}

async fn create_room(
    State(state): State<AppState>,
    Json(body): Json<CreateRoomRequest>,
) -> Json<Room> {
    let room = state
        .rooms
        .create(&body.name, body.max_members)
        .await
        .unwrap();

    Json(room)
}
