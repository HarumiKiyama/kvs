use crate::{KvsEngine, Result};
use sled::{self, Db};

pub struct SledKvsEngine {
    db: Db,
}

impl SledKvsEngine {
    pub fn new() -> Result<Self> {
        Ok(Self {
            db: sled::open("/tmp/sled_db")?,
        })
    }
}


impl KvsEngine for SledKvsEngine {
    fn get(&mut self, key: String) -> Result<Option<String>> {
        let rv = self.db.get(key)?;
        Ok(rv.map(|s| String::from_utf8(AsRef::<[u8]>::as_ref(&s).to_vec()).unwrap()))

    }
    fn set(&mut self, key: String, value: String) -> Result<()> {
        self.db.insert(key.as_str(), value.as_str())?;
        Ok(())
    }
    fn remove(&mut self, key: String) -> Result<()> {
        self.db.remove(key)?;
        Ok(())
    }
}

