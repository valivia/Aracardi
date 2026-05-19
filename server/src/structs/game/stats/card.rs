use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::structs::game::state::CurrentCard;

const RUNNING_AVERAGE_SIZE: usize = 128;

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameCardStats {
    pub play_count: u32,

    pub average_duration_ms: u32,
    pub median_duration_ms: u32,

    #[serde(skip)]
    pub duration_running_average_ms: Vec<u32>,
}

impl Default for GameCardStats {
    fn default() -> Self {
        Self {
            play_count: 0,
            duration_running_average_ms: vec![0; RUNNING_AVERAGE_SIZE],
            average_duration_ms: 0,
            median_duration_ms: 0,
        }
    }
}

impl GameCardStats {
    fn get_average_card_duration_ms(&self) -> u32 {
        let (sum, count) = self
            .duration_running_average_ms
            .iter()
            .filter(|&&v| v != 0)
            .fold((0u64, 0u32), |(sum, count), &v| (sum + v as u64, count + 1));

        if count == 0 {
            0
        } else {
            (sum / count as u64) as u32
        }
    }

    fn get_median_card_duration_ms(&self) -> u32 {
        let mut values: Vec<u32> = self
            .duration_running_average_ms
            .iter()
            .copied()
            .filter(|&v| v != 0)
            .collect();

        if values.is_empty() {
            return 0;
        }

        values.sort_unstable();
        values[values.len() / 2]
    }

    pub fn register_card(&mut self, card: &CurrentCard) {
        let index = self.play_count as usize % RUNNING_AVERAGE_SIZE;
        self.duration_running_average_ms[index] =
            card.inner
                .get_duration()
                .num_milliseconds()
                .clamp(0, Duration::from_mins(60).as_millis() as i64) as u32;
        self.play_count += 1;
        self.average_duration_ms = self.get_average_card_duration_ms();
        self.median_duration_ms = self.get_median_card_duration_ms();
    }
}
