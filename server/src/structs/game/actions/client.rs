use tracing::{debug, info};

use crate::structs::{
    game::{
        Game,
        client::{Client, connection::ClientConnection, socket::ClientSocket},
        state::ClientId,
    },
    protocol::{
        connection::DisconnectReason,
        message::{game_update::GameUpdate, outgoing::OutgoingMessage},
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
            return self.new_client(connection, socket);
        };

        let os = connection.get_os();

        // Host connect
        if !self.clients.contains_key(&self.host_id) && id == self.host_id {
            let mut client = Client::new(id, connection, socket);
            client.is_host = true;
            self.clients.insert(id, client);
            info!(
                client = %id,
                os,
                "[game] {} | Host connected",
                self.join_code,
            );
            return id;
        }

        let Some(client) = self.clients.get_mut(&id) else {
            return self.new_client(connection, socket);
        };

        // TODO: Handle user trying to connect to a non-dead connection
        // TODO: Maybe also compare ip/user agent
        client.reconnect(socket);

        if client.is_host {
            self.send_host_status()
        } else {
            self.sync_client(&id);
        }

        info!(
            client = %id,
            os,
            "[game] {} | {} reconnected",
            self.join_code,
            if id == self.host_id { "Host" } else { "Client" },
        );

        return id;
    }

    fn new_client(&mut self, connection: ClientConnection, socket: ClientSocket) -> ClientId {
        let id = Client::generate_id();
        let os = connection.get_os();
        let client = Client::new(id, connection, socket);
        self.clients.insert(id, client);
        self.sync_client(&id);

        info!(
            client = %id,
            os,
            "[game] {} | Client connected",
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
            "[game] {} | {} disconnected ({})",
            self.join_code,
            if client_id == &self.host_id {
                "Host"
            } else {
                "Client"
            },
            &reason
        );

        client.disconnect(reason);

        if client.is_host {
            self.send_host_status()
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
