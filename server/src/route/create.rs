use axum::extract::State;
use std::sync::Arc;

use crate::AppState;

pub async fn handler(State(state): State<Arc<AppState>>) -> String {
    state.create_game()
}
