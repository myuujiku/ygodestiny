use std::path::Path;

use redb::{Database, DatabaseError, ReadableTable, TableDefinition};
use serde::{de::DeserializeOwned, Serialize};

pub type RedbTable = TableDefinition<'static, u128, &'static str>;
pub const REDB_TABLE: RedbTable = RedbTable::new("data");

pub trait RedbStorage: Serialize + DeserializeOwned {
    fn db_path() -> &'static Path;

    fn save_data(&self, key: u128) -> anyhow::Result<()> {
        let db = Self::open_db()?;
        let write_txn = db.begin_write()?;
        {
            let mut table = write_txn.open_table(REDB_TABLE)?;
            table.insert(key, ron::to_string(self)?.as_str())?;
        }
        write_txn.commit()?;

        Ok(())
    }

    fn load_data(key: u128) -> anyhow::Result<Self> {
        Self::load_data_as(key)
    }

    fn get_all_keys() -> anyhow::Result<Vec<u128>> {
        let db = Self::open_db()?;
        let read_txn = db.begin_read()?;
        let table = read_txn.open_table(REDB_TABLE)?;
        let keys = Ok(table
            .iter()?
            .filter_map(|e| match e {
                Ok(v) => Some(v.0.value()),
                Err(_) => None,
            })
            .collect());

        keys
    }

    // There probably is a better solution, but this works eh
    #[allow(clippy::let_and_return)]
    fn load_data_as<T: DeserializeOwned>(key: u128) -> anyhow::Result<T> {
        let db = Self::open_db()?;
        let read_txn = db.begin_read()?;
        let table = read_txn.open_table(REDB_TABLE)?;

        let data = Ok(ron::from_str(table.get(key)?.unwrap().value())?);

        data
    }

    fn open_db() -> Result<Database, DatabaseError> {
        Database::create(Self::db_path())
    }
}
