use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientInfo {
    joined_at: i64,
    left_at: i64,
    country: String,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameClientStats {
    pub host_reconnected: u32,

    pub clients_connected: u32,
    pub clients_reconnected: u32,
    pub clients_disconnected: u32,

    #[serde(skip)]
    clients: Vec<ClientInfo>,
}

impl Default for GameClientStats {
    fn default() -> Self {
        Self {
            host_reconnected: 0,
            clients_connected: 0,
            clients_reconnected: 0,
            clients_disconnected: 0,

            clients: vec![],
        }
    }
}
