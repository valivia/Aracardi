use crate::structs::state::AppState;
use axum::{
    Router,
    routing::{any, post},
};
use dashmap::DashMap;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

mod route;
mod structs;

#[tokio::main]
async fn main() {
    let shared_state = Arc::new(AppState {
        games: DashMap::new(),
    });

    shared_state.create_game();

    // Allow requests from anywhere

    let mut app = Router::new()
        .route("/ws/{game_id}", any(route::lobby::handler))
        .route("/ws", post(route::create::handler))
        .with_state(shared_state);

    if cfg!(debug_assertions) {
        let cors = CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any);

        app = app.layer(cors);
    }

    let address = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    println!("Server running on {}", address);
    axum::serve(listener, app).await.unwrap();
}
