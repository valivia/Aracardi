use crate::structs::telemetry::active_card::TelemetryActiveCard;
use crate::structs::telemetry::event::{TelemetryEvent, TelemetryEventType};
use crate::structs::telemetry::{card::TelemetryCard, game::TelemetryGame};
use sqlx::PgPool;
use std::env;
use tracing::info;

mod game;
mod played_active_card;
mod played_card;

pub struct Database {
    pool: PgPool,
}

impl Database {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let connection_string = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let pool = PgPool::connect(&connection_string).await?;
        info!("[db] Database connected");

        sqlx::migrate!("./migrations").run(&pool).await?;
        info!("[db] Migrations applied");

        Ok(Database { pool })
    }

    pub async fn persist_events(&self, events: Vec<TelemetryEvent>) -> Result<(), sqlx::Error> {
        let mut cards: Vec<TelemetryCard> = Vec::new();
        let mut active_cards: Vec<TelemetryActiveCard> = Vec::new();
        let mut games: Vec<TelemetryGame> = Vec::new();

        for event in events {
            match event.0 {
                TelemetryEventType::CardViewed(card) => cards.push(card),
                TelemetryEventType::ActiveCardDismissed(card) => active_cards.push(card),
                TelemetryEventType::GameEnded(game) => games.push(game),
            }
        }

        if !games.is_empty() {
            self.insert_games(games).await?;
        }

        let mut tx = self.pool.begin().await?;

        if !cards.is_empty() {
            Self::insert_played_cards(&mut tx, cards).await?;
        }

        if !active_cards.is_empty() {
            Self::insert_played_active_cards(&mut tx, active_cards).await?;
        }

        tx.commit().await?;
        Ok(())
    }
}
