use crate::structs::telemetry::event::{TelemetryEvent, TelemetryEventType};
use crate::structs::telemetry::{card::TelemetryCard, game::TelemetryGame};
use mongodb::{Client, Collection, error::Error as MongoError};
use std::env;
use tracing::info;

const DATABASE_NAME: &str = "aracardi";

pub struct Database {
    client: Client,
}

impl Database {
    pub async fn new() -> Self {
        let uri = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let client = Client::with_uri_str(&uri)
            .await
            .expect("Failed to connect to database");

        info!("[db] Database connected");
        Database { client }
    }

    fn collection<T: Send + Sync>(&self, name: &str) -> Collection<T> {
        self.client.database(DATABASE_NAME).collection::<T>(name)
    }

    pub async fn persist_events(&self, events: Vec<TelemetryEvent>) -> Result<(), MongoError> {
        let mut cards: Vec<TelemetryCard> = Vec::new();
        let mut games: Vec<TelemetryGame> = Vec::new();

        for event in events {
            match event.event {
                TelemetryEventType::CardViewed(card) => cards.push(card),
                TelemetryEventType::GameEnded(game) => games.push(game),
                _ => {}
            }
        }

        if !cards.is_empty() {
            self.collection::<TelemetryCard>("card")
                .insert_many(cards)
                .await?;
        }

        if !games.is_empty() {
            self.collection::<TelemetryGame>("game")
                .insert_many(games)
                .await?;
        }

        Ok(())
    }
}
