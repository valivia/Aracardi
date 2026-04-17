use serde::{Deserialize, Serialize};
use std::collections::HashSet;

pub use crate::structs::game::stats::card::GameCardStats;
pub use crate::structs::game::stats::player::GamePlayerStats;

pub mod card;
pub mod player;

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameStats {
    pub card: GameCardStats,
    pub player: GamePlayerStats,
    pub used_avatars: HashSet<String>,
}

impl Default for GameStats {
    fn default() -> Self {
        GameStats {
            player: GamePlayerStats::default(),
            card: GameCardStats::default(),
            used_avatars: HashSet::new(),
        }
    }
}
