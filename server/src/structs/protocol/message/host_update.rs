use serde::{Deserialize, Serialize};

use crate::structs::game::{
    info::GameInfo,
    state::{Player, card::CardTurns},
};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HostCard {
    pub id: String,
    pub instance_id: String,
    pub players: Vec<String>,
    #[serde(flatten)]
    pub turns: Option<CardTurns>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HostUpdate {
    pub players: Option<Vec<Player>>,
    pub current_player_id: Option<String>,

    pub current_card: Option<HostCard>,
    pub active_cards: Option<Vec<HostCard>>,

    pub info: Option<GameInfo>,
}
