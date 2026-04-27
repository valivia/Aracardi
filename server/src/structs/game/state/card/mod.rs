pub mod active;
pub mod current;
pub mod inner;
pub mod turns;

use std::sync::Arc;

use crate::structs::app_state::AppState;
use crate::structs::game::state::GameState;
use crate::structs::protocol::message::host_update::HostCard;
use crate::util::card_loader::AddonCard;

pub use self::active::ActiveCard;
pub use self::current::CurrentCard;
pub use self::inner::InnerCard;
pub use self::turns::CardTurns;

pub enum CardParseError {
    NotFound,
    InvalidPlayers,
    InvalidTurns,
}

pub trait Card {
    async fn get_reference_card(id: &String, state: &Arc<AppState>) -> Option<AddonCard> {
        let cards = state.cards.read().await;
        let addon_card = cards.get(id)?;
        return Some(addon_card.to_owned());
    }

    async fn from_update(
        state: &Arc<AppState>,
        game_state: &GameState,
        host_card: &HostCard,
    ) -> Result<Self, CardParseError>
    where
        Self: Sized;
}
