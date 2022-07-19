use crate::{KvsEngine, Result};
use std::path::PathBuf;
use sled::{self, Db};

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
    fn set(&mut self, key: String, value: String) -> Result<()> {
        self.db.insert(key.as_str(), value.as_str())?;
        Ok(())
    }
    fn get(&mut self, key: String) -> Result<Option<String>> {
        let rv = self.db.get(key)?;
        Ok(rv.map(|s| String::from_utf8(AsRef::<[u8]>::as_ref(&s).to_vec()).unwrap()))
    }
    fn remove(&mut self, key: String) -> Result<()> {
        self.db.remove(key)?;
        Ok(())
    }
}

