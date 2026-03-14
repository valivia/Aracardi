use crate::{structs::app_state::AppState, util::card_loader::load_cards};
use axum::{
    Router,
    routing::{any, get, post},
};
use dashmap::DashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::{Level, event};

mod route;
mod structs;
mod util;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_max_level(Level::DEBUG)
        // .with_env_filter(
        //     EnvFilter::try_from_default_env()
        //         .or_else(|_| EnvFilter::try_new("server=error,tower_http=warn"))
        //         .unwrap(),
        // )
        .init();

    let shared_state = Arc::new(AppState {
        games: DashMap::new(),
        cards: Arc::new(RwLock::new(load_cards())),
    });

    let mut app = Router::new()
        .route("/lobby/{game_id}/ws", any(route::connect::handler))
        .route("/lobby", post(route::create::handler))
        .route("/lobby", get(route::list::handler))
        .with_state(shared_state)
        .layer(TraceLayer::new_for_http());

    if cfg!(debug_assertions) {
        let cors = CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any);

        app = app.layer(cors);
    }

    let address = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    event!(Level::INFO, "Server running on {}", address);
    axum::serve(listener, app).await.unwrap();
}
