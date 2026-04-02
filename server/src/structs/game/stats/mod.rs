use serde::{Deserialize, Serialize};
use std::collections::HashSet;

pub use crate::structs::game::stats::card::GameCardStats;
use crate::structs::game::stats::client::GameClientStats;
pub use crate::structs::game::stats::player::GamePlayerStats;

pub mod card;
pub mod client;
pub mod player;

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameStats {
    pub card: GameCardStats,
    pub player: GamePlayerStats,
    pub client: GameClientStats,
    pub used_avatars: HashSet<String>,
}

impl Default for GameStats {
    fn default() -> Self {
        GameStats {
            player: GamePlayerStats::default(),
            card: GameCardStats::default(),
            client: GameClientStats::default(),
            used_avatars: HashSet::new(),
        }
    }
}
