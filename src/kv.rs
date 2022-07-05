use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use serde_json::Deserializer;

use crate::{KvsError, Result};

/// KvStore struct
#[derive(Debug)]
pub struct KvStore {
    path: PathBuf,
    index: HashMap<String, ValueLocation>,
    writer: BufWriterWithPos,
    reader: BufReaderWithPos,
}

#[derive(Debug)]
pub struct BufWriterWithPos {
    writer: BufWriter<File>,
    pos: u64,
}

#[derive(Debug)]
struct ValueLocation {
    pos: u64,
    len: u64,
}

#[derive(Debug)]
pub struct BufReaderWithPos {
    reader: BufReader<File>,
    pos: u64,
}

impl BufReaderWithPos {
    fn new(mut inner: File) -> Result<Self> {
        let pos = inner.seek(SeekFrom::Current(0))?;
        Ok(Self {
            reader: BufReader::new(inner),
            pos,
        })
    }
    pub fn get_mut(&mut self) -> &mut File {
        self.reader.get_mut()
    }
}

impl Read for BufReaderWithPos {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let len = self.reader.read(buf)?;
        self.pos += len as u64;
        Ok(len)
    }
}

impl Seek for BufReaderWithPos {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.pos = self.reader.seek(pos)?;
        Ok(self.pos)
    }
}

impl BufWriterWithPos {
    fn new(mut inner: File) -> Result<Self> {
        let pos = inner.seek(SeekFrom::End(0))?;
        let writer = BufWriter::new(inner);
        Ok(Self { writer, pos })
    }
}

impl Write for BufWriterWithPos {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let len = self.writer.write(buf)?;
        self.pos += len as u64;
        Ok(len)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

impl Seek for BufWriterWithPos {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.pos = self.writer.seek(pos)?;
        Ok(self.pos)
    }
}

#[derive(Debug, Serialize, Deserialize)]
enum Operation {
    Set { key: String, value: String },
    Rm { key: String },
}

impl KvStore {
    fn load(&mut self) {
        let mut stream = Deserializer::from_reader(&mut self.reader).into_iter::<Operation>();
        let mut pos: u64 = 0;
        while let Some(op) = stream.next() {
            let new_pos = stream.byte_offset() as u64;
            match op.unwrap() {
                Operation::Set { key, .. } => {
                    self.index.insert(
                        key,
                        ValueLocation {
                            pos,
                            len: new_pos - pos,
                        },
                    );
                }
                Operation::Rm { key } => {
                    self.index.remove(&key);
                }
            }
            pos = new_pos;
        }
    }
    pub fn get(&mut self, key: String) -> Result<Option<String>> {
        if let Some(value_location) = self.index.get(&key) {
            self.reader.seek(SeekFrom::Start(value_location.pos))?;
            let buf_reader = self.reader.get_mut().take(value_location.len);
            match serde_json::from_reader(buf_reader)? {
                Operation::Set { value, .. } => Ok(Some(value)),
                _ => Err(KvsError::UnsupportedOperation),
            }
        } else {
            return Ok(None);
        }
    }
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        let row = Operation::Set { key: key.clone(), value };
        let pos = self.writer.pos;
        serde_json::to_writer(&mut self.writer, &row)?;
        self.writer.flush()?;
        self.index.insert(
            key,
            ValueLocation {
                pos,
                len: self.writer.pos - pos,
            },
        );
        Ok(())
    }
    pub fn remove(&mut self, key: String) -> Result<()> {
        if self.index.remove(&key).is_none() {
            return Err(KvsError::KeyNotFound);
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
            path: path.clone(),
            index: HashMap::new(),
            writer: BufWriterWithPos::new(
                OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(path.clone())?,
            )?,
            reader: BufReaderWithPos::new(OpenOptions::new().read(true).open(path.clone())?)?,
        };
        kvs.load();
        Ok(kvs)
    }
}
