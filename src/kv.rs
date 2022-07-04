use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::{KvsError, Result};

#[derive(Debug)]
pub struct KvStore {
    path: PathBuf,
    index: HashMap<String, ValueLocation>,
    writer: BufWriter<File>,
    reader: BufReader<File>,
}

#[derive(Debug)]
pub struct BufWriterWithPos {
    writer: BufWriter<File>,
}

#[derive(Debug)]
struct ValueLocation {
    pos: u64,
    len: u64,
}

#[derive(Debug)]
pub struct BufReaderWithPos {
    reader: BufReader<File>,
}

impl BufReaderWithPos {
    fn new() -> Result<Self> {
        todo!()
    }
}

impl Read for BufReaderWithPos {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.reader.read(buf)
    }
}

impl Seek for BufReaderWithPos {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.reader.seek(pos)
    }
}

impl Write for BufWriterWithPos {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.write(buf)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.flush()
    }
}

impl Seek for BufWriterWithPos {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.writer.seek(pos)
    }
}

#[derive(Serialize, Deserialize)]
enum Operation {
    Set {
        key: String,
        value: String,
    },
    Rm {
        key: String
    },
}


impl KvStore {
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
        self.index.insert(key.clone(), value.clone());
        let row = Operation::Set { key, value };
        serde_json::to_writer(&mut self.writer, &row)?;
        self.writer.write("\n".as_bytes())?;
        self.writer.flush()?;
        Ok(())
    }
    pub fn remove(&mut self, key: String) -> Result<()> {
        match self.index.remove(&key) {
            None => {
                return Err(KvsError::KeyNotFound());
            }
            _ => (),
        };
        let row = Operation::Rm { key };
        serde_json::to_writer(&mut self.writer, &row)?;
        self.writer.flush()?;
        Ok(())
    }
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        let mut path: PathBuf = path.into();
        path.push("db");
        let mut kvs = KvStore {
            writer: BufWriter::new(path),
            reader: BufReader::new(path),
        };
        kvs.load();
        Ok(kvs)
    }
}
