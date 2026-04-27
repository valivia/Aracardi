use std::time::Duration;
use tokio::{sync::mpsc, time::interval};
use tracing::{debug, error};

use crate::structs::{db::Database, telemetry::event::TelemetryEvent};

pub mod active_card;
pub mod card;
pub mod event;
pub mod game;
pub mod player;

const FLUSH_TIMEOUT: Duration = Duration::from_secs(2);

#[derive(Clone)]
pub struct Telemetry {
    tx: mpsc::Sender<TelemetryEvent>,
}

impl Telemetry {
    pub fn new(db: Database) -> Self {
        let (tx, rx) = mpsc::channel(1024); // back-pressure buffer
        tokio::spawn(drain_task(rx, db));
        Self { tx }
    }

    pub fn push(&self, event: TelemetryEvent) {
        if let Err(_) = self.tx.try_send(event) {
            tracing::warn!("telemetry queue full or closed, dropping event");
        }
    }
}

async fn drain_task(mut rx: mpsc::Receiver<TelemetryEvent>, db: Database) {
    let mut batch: Vec<TelemetryEvent> = Vec::with_capacity(50);
    let mut flush_ticker = interval(FLUSH_TIMEOUT);

    loop {
        tokio::select! {
            // accumulate incoming events
            Some(event) = rx.recv() => {
                batch.push(event);
                if batch.len() >= 50 {
                    flush_ticker.reset();
                    flush(&db, &mut batch).await;
                }
            }
            // periodic flush for low-traffic periods
            _ = flush_ticker.tick() => {
                if !batch.is_empty() {
                    flush(&db, &mut batch).await;
                }
            }
            // channel closed = server shutting down
            else => {
                flush(&db, &mut batch).await;
                break;
            }
        }
    }
}

async fn flush(db: &Database, batch: &mut Vec<TelemetryEvent>) {
    let events = std::mem::take(batch);
    let event_count = events.len();

    let result = db.persist_events(events).await;

    match result {
        Ok(_) => debug!("[telemetry] written {event_count} entries to db"),
        Err(e) => error!("[telemetry] failed to flush {event_count} entries to db: {e}"),
    }
}
