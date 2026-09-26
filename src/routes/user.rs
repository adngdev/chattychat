use crate::{model::User, state::AppState};
use serde::Deserialize;

use axum::{Json, Router, extract::State, routing::post};

pub fn router() -> Router<AppState> {
    Router::new().route("/users", post(create_user))
}

#[derive(Deserialize)]
struct CreateUserRequest {
    username: String,
    name: String,
    password: String,
}

async fn create_user(
    State(state): State<AppState>,
    Json(body): Json<CreateUserRequest>,
) -> Json<User> {
    let user = state
        .users
        .create(&body.username, &body.name, &body.password)
        .await
        .unwrap();

    Json(user)
}
