use crate::{model::Room, state::AppState};
use axum::{
    Json, Router,
    extract::{Query, State},
    routing::post,
};
use serde::{Deserialize, Serialize};

pub fn router() -> Router<AppState> {
    Router::new().route("/rooms", post(create_room).get(list_rooms))
}

fn default_max_members() -> u32 {
    2
}

impl Default for ListRoomsParams {
    fn default() -> Self {
        Self { limit: 10, page: 1 }
    }
}

#[derive(Deserialize)]
struct CreateRoomRequest {
    name: String,
    #[serde(default = "default_max_members")]
    max_members: u32,
}

#[derive(Deserialize)]
#[serde(default)]
struct ListRoomsParams {
    limit: u32,
    page: u32,
}

#[derive(Serialize)]
struct PaginatedResponse<T> {
    rooms: Vec<T>,
    page: u32,
    limit: u32,
    total: u32,
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

async fn list_rooms(
    State(state): State<AppState>,
    Query(params): Query<ListRoomsParams>,
) -> Json<PaginatedResponse<Room>> {
    let rooms = state.rooms.list(params.limit, params.page).await.unwrap();
    let total = state.rooms.count().await.unwrap();

    Json(PaginatedResponse {
        rooms,
        page: params.page,
        limit: params.limit,
        total,
    })
}
