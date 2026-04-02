use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use std::sync::Arc;

use crate::AppState;

pub async fn handler(
    Path(join_code): Path<String>,
    State(state): State<Arc<AppState>>,
) -> StatusCode {
    if state.games.contains_key(&join_code) {
        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    }
}
