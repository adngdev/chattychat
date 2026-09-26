use axum::Router;

use crate::state::AppState;

mod room;
mod user;

pub fn router() -> Router<AppState> {
    Router::new().merge(room::router()).merge(user::router())
}
