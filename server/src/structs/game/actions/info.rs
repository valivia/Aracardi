use chrono::Utc;
use tracing::warn;

use crate::{
    structs::game::{Game, info::GameInfo},
    util::human_readable::HumanReadable,
};

impl Game {
    pub fn initialize_game(&mut self, game_info: GameInfo) {
        if self.info.is_some() {
            warn!(
                "[game] {} | Attempted to update game info after game start",
                self.join_code
            );
            return;
        }

        // TODO: validate
        self.info = Some(game_info.clone());

        let (country, ip, user_agent) = self
            .clients
            .get(&self.host_id)
            .map(|host| {
                (
                    host.connection.country_code.clone(),
                    host.connection.remote_addr.clone(),
                    host.connection.user_agent.clone(),
                )
            })
            .unwrap_or((None, None, None));

        let log_message = format!(
            r#"
    <====== Game started ======>
    Joincode:    {}
    Setup time:  {}
    Players:     {:?}
    Addons:      {:?}
    Country:     {:?}
    IP:          {:?}
    User Agent:  {:#?}
    Version:     {}
    Date:        {}
    <==========================>
    "#,
            self.join_code,
            game_info
                .started_at
                .signed_duration_since(game_info.initiated_at)
                .human_readable(),
            self.state
                .players
                .iter()
                .cloned()
                .map(|player| player.name)
                .collect::<Vec<String>>(),
            game_info.addons,
            country.unwrap_or("unknown".to_string()),
            ip.unwrap_or("unknown".to_string()),
            user_agent.unwrap_or("unknown".to_string()),
            game_info.version,
            Utc::now().format("%Y-%m-%d %H:%M:%S"),
        );

        println!("{}", log_message);
    }
}
