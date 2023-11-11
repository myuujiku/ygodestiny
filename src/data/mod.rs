use once_cell::sync::Lazy;

pub mod deck;
pub mod game;
pub mod game_mode;
pub mod images;
pub mod pack;
pub mod set;

mod redb_storage;
pub use redb_storage::RedbStorage;

mod metadata;
pub use metadata::Metadata;

macro_rules! lazy_path {
    ( $i:ident, $e:expr ) => {
        pub static $i: Lazy<PathBuf> = Lazy::new(|| $e);
    };
}

pub mod dirs;
pub mod files;
