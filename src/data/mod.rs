use once_cell::sync::Lazy;

macro_rules! lazy_path {
    ( $i:ident, $e:expr ) => {
        pub static $i: Lazy<PathBuf> = Lazy::new(|| $e);
    };
}

pub mod dirs;
pub mod files;
pub mod images;
