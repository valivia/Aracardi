use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{structs::game::state::card::CardParseError, util::card_loader::AddonCard};

#[skip_serializing_none]
#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CardTurns {
    #[serde(skip)]
    pub original_turn_count: i32,
    #[serde(skip_serializing)]
    pub turns_passed: i32,

    pub turns_left: i32,
}

impl CardTurns {
    pub fn validate(&self, addon_card: &AddonCard) -> Result<Self, CardParseError> {
        let original_turn_count = addon_card.turns.ok_or(CardParseError::InvalidTurns)?;

        let invalid_turns_left = self.turns_left == 0 || self.turns_left < -1;
        let not_both_indefinite = original_turn_count == -1 && self.turns_left != -1;
        let negative_turns_passed = self.turns_passed < 0;

        if invalid_turns_left || not_both_indefinite || negative_turns_passed {
            return Err(CardParseError::InvalidTurns);
        }

        Ok(Self {
            original_turn_count,
            turns_passed: self.turns_passed + 1,
            turns_left: self.turns_left,
        })
    }
}
