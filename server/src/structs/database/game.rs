use sqlx::PgConnection;

use crate::structs::{
    database::Database,
    game::{GameId, client::Client},
    telemetry::{game::TelemetryGame, player::TelemetryPlayer},
};

impl Database {
    pub async fn insert_games(&self, games: Vec<TelemetryGame>) -> Result<(), sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        for game in games {
            sqlx::query(
                "
            INSERT INTO game \
            (id, join_code,
            initiated_at, started_at, ended_at, addons,
            card_play_count, card_avg_duration_ms, card_median_duration_ms,
            players_initial, players_loaded, players_added, players_renamed, players_removed,
            used_avatars, exclusion_reasons, game_end_reason) \
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)
            ",
            )
            .bind(game.id)
            .bind(game.join_code)
            // Info
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
                Self::insert_client(&mut tx, &game.id, client).await?;
            }

            for player in game.players {
                Self::insert_player(&mut tx, &game.id, player).await?;
            }
        }

        tx.commit().await?;
        Ok(())
    }

    async fn insert_client(
        tx: &mut PgConnection,
        game_id: &GameId,
        client: Client,
    ) -> Result<(), sqlx::Error> {
        let (theme, load_images, allow_nsfw) = match client.preferences {
            Some(p) => (p.theme, Some(p.load_images), Some(p.allow_nsfw)),
            None => (None, None, None),
        };

        sqlx::query(
            "
            INSERT INTO client \
            (id, game_id, is_host, version, user_agent, country_code,
            connected_at, disconnected_at, reconnect_count, total_connected_ms,
            theme, load_images, allow_nsfw)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            ",
        )
        .bind(client.id)
        .bind(game_id)
        .bind(client.is_host)
        .bind(client.connection.version.to_string())
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

        Ok(())
    }

    async fn insert_player(
        tx: &mut PgConnection,
        game_id: &GameId,
        player: TelemetryPlayer,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "
            INSERT INTO player \
            (game_id, name, avatar, is_hand_picked, was_loaded)
            VALUES ($1, $2, $3, $4, $5)
            ",
        )
        .bind(game_id)
        .bind(player.name)
        .bind(player.avatar)
        .bind(player.is_hand_picked)
        .bind(player.was_loaded)
        .execute(&mut *tx)
        .await?;

        Ok(())
    }
}
