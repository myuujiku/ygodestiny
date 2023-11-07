use once_cell::sync::Lazy;

macro_rules! lazy_path {
    ( $i:ident, $e:expr ) => {
        pub static $i: Lazy<PathBuf> = Lazy::new(|| $e);
    };
}

pub mod dirs {
    use std::fs;
    use std::path::PathBuf;

    use super::Lazy;
    use directories::ProjectDirs;

    lazy_path!(ROOT, {
        let mut root = ProjectDirs::from("xyz", "ygodestiny", "YGODestiny")
            .data_dir()
            .to_path_buf();

        #[cfg(all(unix, not(target_os = "macos")))]
        {
            root.pop();
            root.push("xyz.ygodestiny.YGODestiny");
        }

        root
    });

    lazy_path!(USER, ROOT.join("user"));
    lazy_path!(IMAGES, ROOT.join("images"));
    lazy_path!(THEMES, ROOT.join("themes"));

    macro_rules! create_dirs_lazy {
        ( $( $i:ident ),* ) => {
            $( fs::create_dir_all($i.as_path())?; )*
        }
    }

    pub fn init() -> anyhow::Result<()> {
        create_dirs_lazy!(ROOT, USER, IMAGES, THEMES);
        Ok(())
    }
}

pub mod files {
    use std::path::PathBuf;

    use super::{dirs, Lazy};

    lazy_path!(DATA_DB, dirs::ROOT.join("data.db"));
    lazy_path!(DATA_DB_VERSION, dirs::ROOT.join("version.txt"));

    lazy_path!(USER_DB, dirs::USER.join("user.redb"));
}

pub mod images {
    use relm4::gtk::Image;

    use super::dirs::IMAGES;

    pub fn load_card(id: u32) -> Image {
        let filename = IMAGES.join(format!("{}.jpg", id));

        Image::from_file(filename)
    }
}

pub mod redb_tables {
    use redb::TableDefinition;

    macro_rules! create_tables {
        ( $( $n:ident ),* ) => { $(
            pub const $n: TableDefinition<u128, &str> = TableDefinition::new(stringify!($n));
        )* };
    }

    create_tables!(DECKS, GAMES, GAME_MODES, PACKS, SETS);
}
