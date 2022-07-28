use crate::{KvsEngine, KvsError, Result};
use sled::{self, Db};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct SledKvsEngine {
    db: Db,
}

impl SledKvsEngine {
    pub fn open(path: impl Into<PathBuf>) -> Result<Self> {
        Ok(Self {
            db: sled::open(path.into())?,
        })
    }
}

impl KvsEngine for SledKvsEngine {
    fn set(&self, key: String, value: String) -> Result<()> {
        self.db.insert(key.as_str(), value.as_str())?;
        self.db.flush()?;
        Ok(())
    }
    fn get(&self, key: String) -> Result<Option<String>> {
        let rv = self.db.get(key)?;
        Ok(rv.map(|s| String::from_utf8(AsRef::<[u8]>::as_ref(&s).to_vec()).unwrap()))
    }
    fn remove(&self, key: String) -> Result<()> {
        self.db.remove(key)?.ok_or(KvsError::KeyNotFound)?;
        self.db.flush()?;
        Ok(())
    }
}
