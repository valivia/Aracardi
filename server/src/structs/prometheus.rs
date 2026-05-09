use lazy_static::lazy_static;
use prometheus::{IntCounter, IntGauge, register_int_counter, register_int_gauge};

lazy_static! {
    pub static ref GAMES_CREATED_SUM: IntCounter =
        register_int_counter!("games_created_sum", "Total number of games created").unwrap();
}

lazy_static! {
    pub static ref CARDS_PLAYED_SUM: IntCounter =
        register_int_counter!("cards_played_sum", "Total number of cards played").unwrap();
}

lazy_static! {
    pub static ref ACTIVE_GAME_COUNTER: IntGauge =
        register_int_gauge!("active_games", "Number of active games").unwrap();
}

lazy_static! {
    pub static ref CONNECTED_CLIENTS: IntGauge =
        register_int_gauge!("connected_clients", "Number of connected clients").unwrap();
}

pub fn init_metrics() {
    GAMES_CREATED_SUM.reset();
    CARDS_PLAYED_SUM.reset();
    ACTIVE_GAME_COUNTER.set(0);
    CONNECTED_CLIENTS.set(0);
}
