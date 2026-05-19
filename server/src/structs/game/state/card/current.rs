use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::structs::{
    app_state::AppState,
    game::state::{GameState, card::CardParseError},
    protocol::message::host_update::HostCard,
};

use super::{Card, CardTurns, InnerCard};

#[skip_serializing_none]
#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CurrentCard {
    #[serde(flatten)]
    pub inner: InnerCard,
    #[serde(flatten)]
    pub turns: Option<CardTurns>,
}

impl Card for CurrentCard {
    async fn from_update(
        state: &Arc<AppState>,
        game_state: &GameState,
        host_card: &HostCard,
    ) -> Result<Self, CardParseError> {
        let addon_card = Self::get_reference_card(&host_card.id, &state)
            .await
            .ok_or(CardParseError::NotFound)?;

        let inner = InnerCard::from_addon_card(
            addon_card.clone(),
            host_card.instance_id.clone(),
            host_card.players.clone(),
        );

        if !inner.has_valid_players(&game_state.players) {
            return Err(CardParseError::InvalidPlayers);
        }

        let turns = host_card
            .turns
            .clone()
            .and_then(|turns| turns.validate(&addon_card).ok());

        Ok(Self { inner, turns })
    }
}
