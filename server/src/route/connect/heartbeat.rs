use crate::{
    AppState,
    structs::game::{client::Tx, state::ClientId},
};
use std::{sync::Arc, time::Duration};
use tokio::sync::oneshot;

const PING_INTERVAL: Duration = Duration::from_secs(2);
const PONG_TIMEOUT: Duration = Duration::from_secs(4);
const MAX_MISSED_PONGS: u32 = 3;

pub async fn ping_task(
    timeout_tx: oneshot::Sender<()>,
    state: Arc<AppState>,
    join_code: String,
    client_id: ClientId,
    tx: Tx,
) {
    let mut missed = 0u32;

    loop {
        tokio::time::sleep(PING_INTERVAL).await;

        // Send ping
        match state.games.get_mut(&join_code) {
            Some(mut game) => match game.clients.get_mut(&client_id) {
                Some(client) => {
                    if client.ping(&tx).is_err() {
                        break;
                    }
                }
                None => break,
            },
            None => break,
        };

        tokio::time::sleep(PONG_TIMEOUT).await;

        // Check pong
        let timed_out = match state.games.get(&join_code) {
            Some(game) => match game.clients.get(&client_id) {
                Some(client) => client.is_ping_timed_out(),
                // Client removed, exit cleanly
                None => break,
            },
            // Game removed, exit cleanly
            None => return,
        };

        if timed_out {
            missed += 1;
            if missed >= MAX_MISSED_PONGS {
                break;
            }
        } else {
            missed = 0;
        }
    }

    let _ = timeout_tx.send(());
}
