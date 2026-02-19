use axum::extract::ws::Message;

use crate::structs::{
    game::{
        Game,
        protocol::{CURRENT_CARD, CURRENT_PLAYER},
    },
    player::PlayerId,
};

impl Game {
    pub fn on_message(&mut self, id: &PlayerId, msg: Message) {
        if let Some(player) = self.players.get(id) {
            match &msg {
                Message::Close(_) => {
                    println!("Player: disconnect {id}, game: {}", self.id);
                    return;
                }
                Message::Ping(_) => {
                    println!("Player: ping {id}, game: {}", self.id);
                    return;
                }
                Message::Pong(_) => {
                    println!("Player: pong {id}, game: {}", self.id);
                    return;
                }
                Message::Text(text) => {
                    let action = text.split(':').next().unwrap_or("");
                    let payload = text.split(':').nth(1).unwrap_or("");
                    match action {
                        CURRENT_CARD => {
                            self.state.current_card = Some(payload.to_string());
                            self.broadcast(Message::Text(
                                format!("{}:{}", CURRENT_CARD, payload).into(),
                            ));
                            return;
                        }
                        CURRENT_PLAYER => {
                            self.state.current_player = Some(payload.to_string());
                            self.broadcast(Message::Text(
                                format!("{}:{}", CURRENT_PLAYER, payload).into(),
                            ));
                            return;
                        }
                        _ => {}
                    }
                    return;
                }
                _ => {}
            }
        }
    }

    pub fn broadcast(&self, msg: Message) {
        for player in &self.players {
            let _ = player.1.tx().try_send(msg.clone());
        }
    }
}
