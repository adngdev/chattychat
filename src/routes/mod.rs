use axum::Router;

use crate::state::AppState;

mod room;

pub fn router() -> Router<AppState> {
    Router::new().merge(room::router())
}
