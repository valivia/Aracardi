use axum::extract::State;
use std::sync::Arc;

use crate::AppState;

pub async fn handler(State(state): State<Arc<AppState>>) -> String {
    let games = state.games.iter().map(|entry| entry.key().clone()).collect::<Vec<String>>();
    serde_json::to_string(&games).unwrap()
}
