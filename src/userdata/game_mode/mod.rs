use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::data::files;
use crate::userdata::RedbStorage;

#[derive(Serialize, Deserialize)]
pub struct GameMode;

impl RedbStorage for GameMode {
    fn db_path() -> &'static Path {
        &files::GAME_MODE_REDB
    }
}
