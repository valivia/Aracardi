use crate::structs::{database::Database, telemetry::card::TelemetryCard};

impl Database {
    pub async fn insert_played_cards(
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        cards: Vec<TelemetryCard>,
    ) -> Result<(), sqlx::Error> {
        for card in cards {
            sqlx::query(
                "INSERT INTO played_card (game_id, card_id, duration_ms)
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
}
