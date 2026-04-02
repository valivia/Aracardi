use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::structs::game::state::Player;

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GamePlayerStats {
    pub initial: u32,
    pub loaded: u32,
    pub added: u32,
    pub renamed: u32,
    pub removed: u32,
}

impl Default for GamePlayerStats {
    fn default() -> Self {
        GamePlayerStats {
            initial: 0,
            loaded: 0,
            added: 0,
            renamed: 0,
            removed: 0,
        }
    }
}

impl std::ops::AddAssign for GamePlayerStats {
    fn add_assign(&mut self, other: Self) {
        self.initial += other.initial;
        self.loaded += other.loaded;
        self.added += other.added;
        self.renamed += other.renamed;
        self.removed += other.removed;
    }
}

impl GamePlayerStats {
    pub fn calculate_changes(players_old: &Vec<Player>, players_new: &Vec<Player>) -> Self {
        let mut stats = GamePlayerStats::default();
        let old_by_id: HashMap<&str, &Player> =
            players_old.iter().map(|p| (p.id.as_str(), p)).collect();

        let new_by_id: HashMap<&str, &Player> =
            players_new.iter().map(|p| (p.id.as_str(), p)).collect();

        // Renamed or removed
        for (id, old_player) in &old_by_id {
            match new_by_id.get(id) {
                Some(new_player) => {
                    if old_player.name != new_player.name {
                        stats.renamed += 1;
                    }
                }
                None => {
                    stats.removed += 1;
                }
            }
        }

        // Added or loaded
        for (id, new_player) in &new_by_id {
            if !old_by_id.contains_key(id) {
                if players_old.is_empty() {
                    if new_player
                        .was_loaded
                        .is_some_and(|was_loaded| was_loaded == true)
                    {
                        stats.loaded += 1
                    } else {
                        stats.initial += 1;
                    };
                } else {
                    stats.added += 1;
                }
            }
        }
        return stats;
    }
}
