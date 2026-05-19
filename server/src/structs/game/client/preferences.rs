use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientPreferences {
    pub theme: Option<String>,
    pub load_images: bool,
    pub allow_nsfw: bool,
}
