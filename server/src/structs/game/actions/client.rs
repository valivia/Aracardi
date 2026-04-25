use tracing::{debug, info};

use crate::structs::{
    game::{
        Game,
        client::{Client, connection::ClientConnection, socket::ClientSocket},
        state::ClientId,
    },
    protocol::message::{game_update::GameUpdate, outgoing::OutgoingMessage},
};

impl Game {
    pub fn upsert_client(
        &mut self,
        id: Option<ClientId>,
        connection: ClientConnection,
        socket: ClientSocket,
    ) -> ClientId {
        let Some(id) = id else {
            return self.new_client(connection, socket);
        };

        // Host connect
        if !self.is_host_connected() && id == self.host_id {
            let mut client = Client::new(id, connection, socket);
            client.is_host = true;
            self.clients.insert(id, client);
            info!(
                game = self.join_code,
                client = id.to_string(),
                "Host connected",
            );
            return id;
        }

        let Some(client) = self.clients.get_mut(&id) else {
            return self.new_client(connection, socket);
        };

        // TODO: Handle user trying to connect to a non-dead connection
        // TODO: Maybe also compare ip/user agent
        client.reconnect(socket);
        self.sync_client(&id);

        info!(
            game = self.join_code,
            client = id.to_string(),
            "{} reconnected",
            if id == self.host_id { "Host" } else { "Client" },
        );

        return id;
    }

    fn new_client(&mut self, connection: ClientConnection, socket: ClientSocket) -> ClientId {
        let id = Client::generate_id();
        let client = Client::new(id, connection, socket);
        self.clients.insert(id, client);
        self.sync_client(&id);

        info!(
            game = self.join_code,
            client = id.to_string(),
            "Client connected",
        );

        return id;
    }

    pub fn remove_client(&mut self, client_id: &ClientId) {
        if let Some(client) = self.clients.get_mut(&client_id) {
            if client.is_disconnected() {
                debug!(
                    client_id = client_id.to_string(),
                    "Attempted to double disconnect"
                );
                return;
            }

            client.disconnect(self.game_end_reason.is_some());

            info!(
                game = self.join_code,
                client = client_id.to_string(),
                "{} disconnected",
                if client_id == &self.host_id {
                    "Host"
                } else {
                    "Client"
                }
            );
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
}
