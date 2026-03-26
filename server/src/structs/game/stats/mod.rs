use serde::{Deserialize, Serialize};
use std::collections::HashSet;

pub use crate::structs::game::stats::player::GamePlayerStats;

pub mod player;

#[derive(Clone, Deserialize, Serialize)]
pub struct GameStats {
    pub players: GamePlayerStats,
    pub used_avatars: HashSet<String>,
    pub duration_ms: i64,
}

impl Default for GameStats {
    fn default() -> Self {
        GameStats {
            players: GamePlayerStats::default(),
            used_avatars: HashSet::new(),
            duration_ms: 0,
        }
    }
}
