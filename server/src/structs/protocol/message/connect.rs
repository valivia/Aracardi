use semver::Version;
use serde::Deserialize;

use crate::structs::game::state::ClientId;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Connect {
    pub client_id: Option<ClientId>,
    pub version: Version,
}
