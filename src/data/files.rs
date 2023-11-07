use std::path::PathBuf;

use super::{dirs, Lazy};

lazy_path!(DATA_DB, dirs::ROOT.join("data.db"));
lazy_path!(DATA_DB_VERSION, dirs::ROOT.join("version.txt"));

lazy_path!(DECK_REDB, dirs::USER.join("decks.redb"));
lazy_path!(GAME_REDB, dirs::USER.join("games.redb"));
lazy_path!(GAME_MODE_REDB, dirs::USER.join("game_modes.redb"));
lazy_path!(PACK_REDB, dirs::USER.join("packs.redb"));
lazy_path!(SET_REDB, dirs::USER.join("sets.redb"));
