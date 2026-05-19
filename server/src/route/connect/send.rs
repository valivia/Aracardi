use axum::extract::ws::{Message, WebSocket};
use futures_util::{sink::SinkExt, stream::SplitSink};
use tokio::sync::mpsc::Receiver;

pub async fn send_task(mut rx: Receiver<Message>, mut sender: SplitSink<WebSocket, Message>) {
    while let Some(msg) = rx.recv().await {
        if sender.send(msg).await.is_err() {
            break;
        }
    }
}
