use crate::structs::state::AppState;
use axum::{
    Router,
    routing::{any, post},
};
use dashmap::DashMap;
use std::sync::Arc;

mod route;
mod structs;

#[tokio::main]
async fn main() {
    let shared_state = Arc::new(AppState {
        games: DashMap::new(),
    });

    shared_state.create_game();

    let app = Router::new()
        .route("/ws/{game_id}", any(route::lobby::handler))
        .route("/ws", post(route::create::handler))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
