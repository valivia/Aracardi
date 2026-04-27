use core::panic;
use std::{
    collections::HashMap,
    error::Error,
    fs::{DirEntry, File, read_dir},
    io::BufReader,
};

use serde::Deserialize;
use tracing::{debug, error, info};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Addon {
    cards: Vec<AddonCard>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AddonCard {
    pub id: String,
    pub title: Option<String>,
    pub text: String,
    pub turns: Option<i32>,
    pub time_limit: Option<u32>,

    #[serde(default)]
    pub image: bool,
}

pub fn load_cards() -> HashMap<String, AddonCard> {
    let mut cards = HashMap::new();
    if let Ok(entries) = read_dir("addons") {
        entries.for_each(|entry| {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    match read_addon(&entry) {
                        Ok(addon) => {
                            cards.extend(addon);
                        }
                        Err(e) => error!("Failed to load addon: {}", e),
                    }
                }
            }
        });
    } else {
        panic!("Failed to read cards directory");
    }

    if cards.is_empty() {
        panic!("No cards loaded");
    }

    info!("[app] Loaded {} cards", cards.len());

    return cards;
}

fn read_addon(dir_entry: &DirEntry) -> Result<HashMap<String, AddonCard>, Box<dyn Error>> {
    let path = dir_entry.path();
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let addon: Addon = serde_json::from_reader(reader)?;
    let cards: HashMap<String, AddonCard> = addon
        .cards
        .into_iter()
        .map(|card| (card.id.clone(), card))
        .collect();

    debug!(
        "[app] Loaded {} cards from {}",
        cards.len(),
        dir_entry.file_name().to_string_lossy()
    );
    Ok(cards)
}
