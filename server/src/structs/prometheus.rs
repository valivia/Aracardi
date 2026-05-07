use lazy_static::lazy_static;
use prometheus::{IntCounter, IntGauge, register_int_counter, register_int_gauge};

lazy_static! {
    pub static ref TOTAL_GAMES: IntCounter =
        register_int_counter!("total_games", "Total number of games").unwrap();
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
    TOTAL_GAMES.reset();
    ACTIVE_GAME_COUNTER.set(0);
    CONNECTED_CLIENTS.set(0);
}
