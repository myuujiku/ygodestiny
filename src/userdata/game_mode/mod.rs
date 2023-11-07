use std::path::Path;
use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use crate::data::files;
use crate::userdata::RedbStorage;

#[derive(Serialize, Deserialize)]
pub struct GameMode {
    metadata: GameModeMetadata,
    games: Vec<u128>,
    settings: (),
}

impl RedbStorage for GameMode {
    fn db_path() -> &'static Path {
        &files::GAME_MODE_REDB
    }
}

#[derive(Serialize, Deserialize)]
pub struct GameModeMetadata {
    pub name: String,
    pub description: String,
    pub last_played: SystemTime,
}
