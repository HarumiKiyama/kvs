use crate::{KvsError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::PathBuf;

#[derive(Debug)]
pub struct KvStore {
    map: HashMap<String, String>,
    writer: BufWriter<File>,
    reader: BufRead<File>,
}

#[derive(Serialize, Deserialize)]
struct Row(String, String, String);

impl KvStore {
    pub fn new(writer_handler: File, path: PathBuf) -> KvStore {
        KvStore {
            map: HashMap::new(),
            writer: BufWriter::new(writer_handler),
            db_path: path,
        }
    }
    fn load(&mut self) {
        let f = File::open(&self.db_path).expect("Unable to open file");
        let reader = BufReader::new(f);
        for line in reader.lines().map(|x| x.unwrap()) {
            let s = line.trim();
            let (op, key, value) = match serde_json::from_str(s) {
                Ok(v) => v,
                Err(e) => {
                    println!("{:?}", e);
                    continue;
                }
            };
            match op {
                "set" => {
                    self.map.insert(key, value);
                }
                "rm" => {
                    self.map.remove(&key);
                }
                _ => println!("not valid operation"),
            }
        }
    }
    pub fn get(&self, key: String) -> Result<Option<String>> {
        Ok(self.map.get(&key).cloned())
    }
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        self.map.insert(key.clone(), value.clone());
        let row = Row("set".to_string(), key, value);
        serde_json::to_writer(&mut self.writer, &row)?;
        self.writer.write("\n".as_bytes())?;
        self.writer.flush()?;
        Ok(())
    }
    pub fn remove(&mut self, key: String) -> Result<()> {
        match self.map.remove(&key) {
            None => {
                return Err(KvsError::KeyNotFound());
            }
            _ => (),
        };
        let row = Row("rm".to_string(), key, "nothing".to_string());
        serde_json::to_writer(&mut self.writer, &row)?;
        self.writer.flush()?;
        Ok(())
    }
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        let mut path: PathBuf = path.into();
        path.push("db");
        let writer_handler = OpenOptions::new().append(true).create(true).open(&path)?;
        let mut kvs = KvStore::new(writer_handler, path);
        kvs.load();
        Ok(kvs)
    }
}
