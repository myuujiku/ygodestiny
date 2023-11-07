use std::path::PathBuf;

use redb::{RedbKey, RedbValue, TableDefinition};

use crate::data::files;

pub trait RedbStorage {
    fn db_path() -> PathBuf {
        files::USER_DB.clone()
    }

    fn table<'a, K: RedbKey + 'static, V: RedbValue + 'static>() -> TableDefinition<'a, K, V>;
}
