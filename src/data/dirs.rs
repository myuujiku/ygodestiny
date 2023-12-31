use std::fs;
use std::path::PathBuf;

use super::Lazy;
use directories::ProjectDirs;

lazy_path!(ROOT, {
    ProjectDirs::from("xyz", "ygodestiny", "YGODestiny")
        .expect("Failed to get project dirs")
        .data_dir()
        .to_path_buf()
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
