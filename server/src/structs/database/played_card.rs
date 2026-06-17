use crate::structs::{database::Database, telemetry::active_card::TelemetryActiveCard};

impl Database {
    pub async fn insert_played_active_cards(
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        active_cards: Vec<TelemetryActiveCard>,
    ) -> Result<(), sqlx::Error> {
        for card in active_cards {
            sqlx::query(
            "INSERT INTO played_active_card (game_id, card_id, duration_ms, expected_turns, actual_turns)
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
}
