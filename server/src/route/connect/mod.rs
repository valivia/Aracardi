use axum::{
    extract::{Path, State, ws::WebSocketUpgrade},
    http::HeaderMap,
    response::Response,
};
use std::sync::Arc;

use crate::AppState;

use self::client::handle_socket;

mod client;
mod heartbeat;
mod receive;
mod send;

pub async fn handler(
    ws: WebSocketUpgrade,
    Path(join_code): Path<String>,
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, state, join_code.to_uppercase(), headers))
}
