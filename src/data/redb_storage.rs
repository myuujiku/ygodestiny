use std::any::type_name;
use std::path::Path;

use log::warn;
use redb::{Database, ReadableTable, TableDefinition};
use serde::{de::DeserializeOwned, Serialize};
use uuid::Uuid;

pub type RedbTable = TableDefinition<'static, u128, &'static str>;
pub const REDB_TABLE: RedbTable = RedbTable::new("data");

pub trait RedbStorage: Serialize + DeserializeOwned {
    fn db_path() -> &'static Path;

    fn generate_uuid() -> anyhow::Result<Option<u128>> {
        let uuid = Uuid::new_v4().as_u128();

        let db = Self::open_db()?;
        let read_txn = db.begin_read()?;
        let table = read_txn.open_table(REDB_TABLE)?;
        let element = table.get(uuid)?;

        if element.is_none() {
            Ok(Some(uuid))
        } else {
            Ok(None)
        }
    }

    fn save(&self, key: u128) -> anyhow::Result<()> {
        let db = Self::open_db()?;
        let write_txn = db.begin_write()?;
        {
            let mut table = write_txn.open_table(REDB_TABLE)?;
            table.insert(key, ron::to_string(self)?.as_str())?;
        }
        write_txn.commit()?;

        Ok(())
    }

    fn get(key: u128) -> anyhow::Result<Self> {
        Self::get_as(key)
    }

    // There probably is a better solution, but this works eh
    #[allow(clippy::let_and_return)]
    fn get_as<T: DeserializeOwned>(key: u128) -> anyhow::Result<T> {
        let db = Self::open_db()?;
        let read_txn = db.begin_read()?;
        let table = read_txn.open_table(REDB_TABLE)?;

        let data = Ok(ron::from_str(table.get(key)?.unwrap().value())?);

        data
    }

    fn get_all() -> anyhow::Result<Vec<(u128, Self)>> {
        Self::get_all_as()
    }

    fn get_all_as<T: DeserializeOwned>() -> anyhow::Result<Vec<(u128, T)>> {
        let db = Self::open_db()?;
        let read_txn = db.begin_read()?;
        let table = read_txn.open_table(REDB_TABLE)?;

        let mut data = Vec::new();
        for element in table.iter()? {
            match element {
                Ok((k, v)) => {
                    let (k, v) = (k.value(), v.value());

                    match ron::from_str::<T>(v) {
                        Ok(value) => {
                            data.push((k, value));
                        }
                        Err(e) => warn!("Error while parsing {} {k:#X}: {e}", type_name::<T>()),
                    }
                }
                Err(e) => warn!("Error while reading {} database: {e}", type_name::<T>()),
            }
        }

        Ok(data)
    }

    fn open_db() -> anyhow::Result<Database> {
        let path = Self::db_path();
        let exists = path.is_file();

        let db = Database::create(path)?;

        // Make sure that the default table exists
        if !exists {
            let write_txn = db.begin_write()?;
            write_txn.open_table(REDB_TABLE)?;
            write_txn.commit()?;
        }

        Ok(db)
    }
}
