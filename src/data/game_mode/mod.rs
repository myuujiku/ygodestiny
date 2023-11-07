use std::path::Path;
use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use crate::data::files;
use crate::data::RedbStorage;
use crate::settings::game_mode::Settings;

#[derive(Debug, Serialize, Deserialize)]
pub struct GameMode {
    pub metadata: GameModeMetadata,
    pub settings: Settings,
    pub games: Vec<u128>,
}

impl RedbStorage for GameMode {
    fn db_path() -> &'static Path {
        &files::GAME_MODE_REDB
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameModeMetadata {
    pub name: String,
    pub game_mode_type: String,
    pub last_played: SystemTime,
}
