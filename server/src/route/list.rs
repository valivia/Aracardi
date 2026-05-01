use axum::extract::State;
use std::sync::Arc;

use crate::AppState;

pub async fn handler(State(state): State<Arc<AppState>>) -> String {
    let games = state
        .games
        .iter()
        .map(|entry| entry.id.clone().to_string())
        .collect::<Vec<String>>();
    serde_json::to_string(&games).unwrap()
}
