use std::sync::Arc;

use tracing::{debug, info};

use crate::{
    ACTIVE_GAME_COUNTER,
    structs::{
        app_state::AppState,
        game::{
            Game, GameEndReason, MAX_HOST_ABSENCE,
            client::{Client, connection::ClientConnection, socket::ClientSocket},
            state::ClientId,
        },
        protocol::{
            connection::DisconnectReason,
            message::{game_update::GameUpdate, outgoing::OutgoingMessage},
        },
    },
};

impl Game {
    pub fn upsert_client(
        &mut self,
        id: Option<ClientId>,
        connection: ClientConnection,
        socket: ClientSocket,
    ) -> ClientId {
        let Some(id) = id else {
            return self.register_client(connection, socket);
        };

        let os = connection.get_os();

        let Some(client) = self.clients.get_mut(&id) else {
            return match id == self.host_id {
                // Initial host join
                true => self.register_host(connection, socket),
                // Client joined with invalid ID
                false => self.register_client(connection, socket),
            };
        };

        let is_host = client.is_host;

        // TODO: Handle user trying to connect to a non-dead connection
        // TODO: Maybe also compare ip/user agent
        client.reconnect(socket);

        if is_host {
            self.send_host_status();

            // Cancel deletion timer
            if let Some(old_task) = self.host_timeout_task.take() {
                old_task.abort();
            }
        } else {
            self.sync_client(&id);
        }

        info!(
            client = %id,
            os,
            "[game] {} | 🟡 {} reconnected",
            self.join_code,
            if is_host { "Host" } else { "Client" },
        );

        return id;
    }

    fn register_host(&mut self, connection: ClientConnection, socket: ClientSocket) -> ClientId {
        let os = connection.get_os();
        let mut client = Client::new(self.host_id, connection, socket);
        client.is_host = true;

        self.clients.insert(self.host_id, client);

        info!(
            client = %self.host_id,
            os,
            "[game] {} | 🔌 Host connected",
            self.join_code,
        );

        return self.host_id;
    }

    fn register_client(&mut self, connection: ClientConnection, socket: ClientSocket) -> ClientId {
        let os = connection.get_os();
        let id = Client::generate_id();
        let client = Client::new(id, connection, socket);
        self.clients.insert(id, client);
        self.sync_client(&id);

        info!(
            client = %id,
            os,
            "[game] {} | 🟢 Client connected",
            self.join_code,
        );

        return id;
    }

    pub fn disconnect_client(&mut self, client_id: &ClientId, reason: DisconnectReason) {
        let Some(client) = self.clients.get_mut(&client_id) else {
            return;
        };

        if client.is_disconnected() {
            debug!(
                client = %client_id,
                "Attempted to double disconnect"
            );
            return;
        }

        info!(
            client = %client_id,
            os = client.connection.get_os(),
            "[game] {} | 🔴 {} disconnected ({})",
            self.join_code,
            if client_id == &self.host_id {
                "Host"
            } else {
                "Client"
            },
            &reason
        );

        client.disconnect(reason.clone());

        if client.is_host {
            self.send_host_status();

            if let Some(old_task) = self.host_timeout_task.take() {
                old_task.abort();
            }

            if !reason.is_intentional() {
                self.host_timeout_task = Some(tokio::spawn(Self::host_timeout(
                    self.app_state.clone(),
                    self.join_code.clone(),
                )));
            }
        }
    }

    pub fn sync_client(&self, id: &ClientId) {
        let client = self.clients.get(id);
        let mut game_update = GameUpdate::from_game(self.state.clone());
        game_update.host_connected = Some(self.is_host_connected());
        if let Some(client) = client {
            client.send(OutgoingMessage::GameUpdate(game_update).to_message());
        }
    }

    async fn host_timeout(state: Arc<AppState>, join_code: String) {
        tokio::time::sleep(MAX_HOST_ABSENCE).await;

        let removed = state
            .games
            .remove_if(&join_code, |_, game| !game.is_host_connected());

        if let Some((_id, mut game)) = removed {
            game.close(GameEndReason::HostTimeout);
            ACTIVE_GAME_COUNTER.dec();
            info!(
                "[game] {join_code} | 🗑️  Deleted game ({})",
                GameEndReason::HostTimeout
            );
        }
    }
}
