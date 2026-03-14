use axum::{Json, extract::State};
use std::sync::Arc;

use crate::{AppState, structs::app_state::CreatedGame};

pub async fn handler(State(state): State<Arc<AppState>>) -> Json<CreatedGame> {
    Json(state.create_game())
}
