use std::{sync::Arc, time::Duration};
use tokio::sync::oneshot;

use crate::{AppState, structs::game::state::ClientId};

const PING_INTERVAL: Duration = Duration::from_secs(5);
const PONG_TIMEOUT: Duration = Duration::from_secs(3);

pub async fn ping_task(
    timeout_tx: oneshot::Sender<()>,
    state: Arc<AppState>,
    join_code: String,
    client_id: ClientId,
) {
    loop {
        tokio::time::sleep(PING_INTERVAL).await;

        match state.games.get_mut(&join_code) {
            Some(mut game) => match game.clients.get_mut(&client_id) {
                Some(client) => {
                    if client.ping().is_err() {
                        break;
                    }
                }
                None => break,
            },
            None => break,
        };

        tokio::time::sleep(PONG_TIMEOUT).await;

        let timed_out = match state.games.get(&join_code) {
            Some(game) => match game.clients.get(&client_id) {
                Some(client) => client.is_ping_timed_out(),
                None => true,
            },
            None => true,
        };

        if timed_out {
            break;
        }
    }

    let _ = timeout_tx.send(());
}
