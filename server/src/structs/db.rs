use crate::structs::telemetry::active_card::TelemetryActiveCard;
use crate::structs::telemetry::event::{TelemetryEvent, TelemetryEventType};
use crate::structs::telemetry::{card::TelemetryCard, game::TelemetryGame};
use sqlx::PgPool;
use std::env;
use tracing::info;

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
            Self::insert_cards(&mut tx, cards).await?;
        }

        if !active_cards.is_empty() {
            Self::insert_active_cards(&mut tx, active_cards).await?;
        }

        tx.commit().await?;
        Ok(())
    }

    async fn insert_cards(
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        cards: Vec<TelemetryCard>,
    ) -> Result<(), sqlx::Error> {
        for card in cards {
            sqlx::query(
                "INSERT INTO card (game_id, card_id, duration_ms)
             VALUES ($1, $2, $3)",
            )
            .bind(card.game_id)
            .bind(card.card_id)
            .bind(card.duration_ms)
            .execute(&mut **tx)
            .await?;
        }
        Ok(())
    }

    async fn insert_active_cards(
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        active_cards: Vec<TelemetryActiveCard>,
    ) -> Result<(), sqlx::Error> {
        for card in active_cards {
            sqlx::query(
            "INSERT INTO active_card (game_id, card_id, duration_ms, expected_turns, actual_turns)
             VALUES ($1, $2, $3, $4, $5)",
        )
        .bind(card.game_id)
        .bind(card.card_id)
        .bind(card.duration_ms)
        .bind(card.expected_turns)
        .bind(card.actual_turns)
        .execute(&mut **tx)
        .await?;
        }
        Ok(())
    }

    async fn insert_games(&self, games: Vec<TelemetryGame>) -> Result<(), sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        for game in games {
            sqlx::query(
                "
            INSERT INTO game \
            (id, join_code,
            version, initiated_at, started_at, ended_at, addons,
            card_play_count, card_avg_duration_ms, card_median_duration_ms,
            players_initial, players_loaded, players_added, players_renamed, players_removed,
            used_avatars, exclusion_reasons, game_end_reason) \
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18)
            ",
            )
            .bind(game.id)
            .bind(game.join_code)
            // Info
            .bind(game.info.version)
            .bind(game.info.initiated_at)
            .bind(game.info.started_at)
            .bind(game.info.ended_at)
            .bind(game.info.addons)
            // card stats
            .bind(game.stats.card.play_count as i32)
            .bind(game.stats.card.average_duration_ms as i32)
            .bind(game.stats.card.median_duration_ms as i32)
            // player stats
            .bind(game.stats.player.initial as i32)
            .bind(game.stats.player.loaded as i32)
            .bind(game.stats.player.added as i32)
            .bind(game.stats.player.renamed as i32)
            .bind(game.stats.player.removed as i32)
            // Other
            .bind(game.stats.used_avatars.into_iter().collect::<Vec<_>>())
            .bind(
                game.exclusion_reasons
                    .iter()
                    .map(|r| r.to_string())
                    .collect::<Vec<_>>(),
            )
            .bind(game.game_end_reason.to_string())
            .execute(&mut *tx)
            .await?;

            for client in game.clients {
                let (theme, load_images, allow_nsfw) = match client.preferences {
                    Some(p) => (p.theme, Some(p.load_images), Some(p.allow_nsfw)),
                    None => (None, None, None),
                };

                sqlx::query(
                    "
            INSERT INTO client \
            (id, game_id, is_host, user_agent, country_code,
            connected_at, disconnected_at, reconnect_count, total_connected_ms,
            theme, load_images, allow_nsfw)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            ",
                )
                .bind(client.id)
                .bind(game.id)
                .bind(client.is_host)
                .bind(client.connection.user_agent)
                .bind(client.connection.country_code)
                .bind(client.session.original_connect_at)
                .bind(client.session.disconnected_at)
                .bind(client.session.reconnect_count as i32)
                .bind(client.session.total_time_connected_ms)
                .bind(theme)
                .bind(load_images)
                .bind(allow_nsfw)
                .execute(&mut *tx)
                .await?;
            }

            for player in game.players {
                sqlx::query(
                    "
            INSERT INTO player \
            (game_id, name, avatar, is_hand_picked, was_loaded)
            VALUES ($1, $2, $3, $4, $5)
            ",
                )
                .bind(game.id)
                .bind(player.name)
                .bind(player.avatar)
                .bind(player.is_hand_picked)
                .bind(player.was_loaded)
                .execute(&mut *tx)
                .await?;
            }
        }

        tx.commit().await?;
        Ok(())
    }
}
