use axum::extract::ws::Message;

use crate::structs::{
    game::{
        Game,
        protocol::{CURRENT_CARD, CURRENT_PLAYER},
    },
    player::{Player, PlayerId, Role, Tx},
};

impl Game {
    pub fn add_player(&mut self, tx: Tx) -> PlayerId {
        let id = Player::generate_id();
        let role = if self.host.is_none() {
            self.host = Some(id.clone());
            Role::Host
        } else {
            Role::Player
        };

        println!("Adding player {id} ({role:?}) to game {}", self.id);

        let player = Player::new(tx, role);
        self.players.insert(id.clone(), player);

        self.broadcast(Message::Text(format!("join:{}", id).into()));

        return id;
    }

    pub fn remove_player(&mut self, id: &PlayerId) {
        println!("Removing player {id} from game {}", self.id);
        self.players.remove(id);
    }

    pub fn update_player(&mut self, id: &PlayerId, tx: Tx) {
        if let Some(player) = self.players.get_mut(id) {
            println!("Updated player {id} in game {}", self.id);
            player.tx = tx;
        }
    }

    pub async fn sync_player(&self, id: &PlayerId) {
        let player = self.player(id);
        if let Some(player) = player {
            if let Some(current_card) = &self.state.current_card {
                player
                    .tx
                    .send(Message::Text(
                        format!("{}:{}", CURRENT_CARD, current_card).into(),
                    ))
                    .await
                    .ok();
            }

            if let Some(current_player) = &self.state.current_player {
                player
                    .tx
                    .send(Message::Text(
                        format!("{}:{}", CURRENT_PLAYER, current_player).into(),
                    ))
                    .await
                    .ok();
            }
        }
    }

    pub fn player(&self, id: &PlayerId) -> Option<&Player> {
        self.players.get(id)
    }
}
