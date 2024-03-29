use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::ops::DerefMut;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;

use serde::{Deserialize, Serialize};
use serde_json::Deserializer;

use crate::{KvsEngine, KvsError, Result};

const COMPACTION_THRESHOLD: u64 = 1024 * 1024;

/// KvStore struct
#[derive(Debug, Clone)]
pub struct KvStore {
    path: Arc<PathBuf>,
    index: Arc<Mutex<HashMap<String, ValueLocation>>>,
    writer: Arc<Mutex<BufWriterWithPos>>,
    reader: Arc<Mutex<BufReaderWithPos>>,
    // uncompacted data bytes
    uncompacted: Arc<Mutex<u64>>,
}

#[derive(Debug)]
pub struct BufWriterWithPos {
    writer: BufWriter<File>,
    pos: u64,
}

#[derive(Debug, Clone)]
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

impl KvsEngine for KvStore {
    fn get(&self, key: String) -> Result<Option<String>> {
        let index_guard = self.index.lock().unwrap();
        let mut reader_guard = self.reader.lock().unwrap();
        if let Some(value_location) = index_guard.get(&key) {
            reader_guard.seek(SeekFrom::Start(value_location.pos))?;
            let mut buf: Vec<u8> = vec![0; value_location.len as usize];
            reader_guard.read_exact(&mut buf)?;
            match serde_json::from_slice(&buf)? {
                Operation::Set { value, .. } => Ok(Some(value)),
                _ => Err(KvsError::UnsupportedOperation),
            }
        } else {
            return Ok(None);
        }
    }
    fn remove(&self, key: String) -> Result<()> {
        let mut writer_guard = self.writer.lock().unwrap();
        let mut index_guard = self.index.lock().unwrap();
        if index_guard.remove(&key).is_none() {
            return Err(KvsError::KeyNotFound);
        };
        let row = Operation::Rm { key };
        serde_json::to_writer(writer_guard.deref_mut(), &row)?;
        writer_guard.flush()?;
        Ok(())
    }
    fn set(&self, key: String, value: String) -> Result<()> {
        let row = Operation::Set {
            key: key.clone(),
            value,
        };
        let mut writer_guard = self.writer.lock().unwrap();
        let mut uncompacted_guard = self.uncompacted.lock().unwrap();
        let mut index_guard = self.index.lock().unwrap();
        let pos = writer_guard.pos;
        serde_json::to_writer(writer_guard.deref_mut(), &row)?;
        writer_guard.flush()?;
        if let Some(v) = index_guard.insert(
            key,
            ValueLocation {
                pos,
                len: writer_guard.pos - pos,
            },
        ) {
            *uncompacted_guard += v.len;
        };
        if *uncompacted_guard >= COMPACTION_THRESHOLD {
            self.compact()?;
        }
        Ok(())
    }
}

impl KvStore {
    fn load(&self) {
        let mut reader_guard = self.reader.lock().unwrap();
        let mut index_guard = self.index.lock().unwrap();
        let mut uncompacted_guard = self.uncompacted.lock().unwrap();
        let mut stream =
            Deserializer::from_reader(reader_guard.deref_mut()).into_iter::<Operation>();
        let mut pos: u64 = 0;
        while let Some(op) = stream.next() {
            let new_pos = stream.byte_offset() as u64;
            match op.unwrap() {
                Operation::Set { key, .. } => {
                    if let Some(v) = index_guard.insert(
                        key,
                        ValueLocation {
                            pos,
                            len: new_pos - pos,
                        },
                    ) {
                        *uncompacted_guard += v.len;
                    };
                }
                Operation::Rm { key } => {
                    let v = index_guard.remove(&key).unwrap();
                    *uncompacted_guard += v.len;
                }
            }
            pos = new_pos;
        }
    }
    fn compact(&self) -> Result<()> {
        let mut archive_path: PathBuf = self.path.to_path_buf();
        archive_path.push(format!("db.archive.{:?}", SystemTime::now()));
        let mut current_path = self.path.to_path_buf();
        current_path.push("db");
        fs::copy(&current_path, &archive_path)?;
        fs::remove_file(&current_path)?;
        let mut writer = BufWriterWithPos::new(
            OpenOptions::new()
                .create(true)
                .append(true)
                .open(&current_path)?,
        )?;
        let mut reader = BufReaderWithPos::new(OpenOptions::new().read(true).open(&archive_path)?)?;
        let mut new_pos = 0;
        let mut index_guard = self.index.lock().unwrap();
        for v in index_guard.values_mut() {
            let cur_reader = reader.get_mut();
            cur_reader.seek(SeekFrom::Start(v.pos))?;
            let mut data_reader = cur_reader.take(v.len);
            let len = io::copy(&mut data_reader, &mut writer)?;
            *v = ValueLocation { pos: new_pos, len };
            new_pos += len;
        }
        writer.flush()?;
        *self.writer.lock().unwrap() = writer;
        *self.uncompacted.lock().unwrap() = 0;
        fs::remove_file(&archive_path)?;
        Ok(())
    }

    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        let dir = path.into();
        let mut db_path: PathBuf = dir.clone();
        db_path.push("db");
        let kvs = KvStore {
            path: Arc::new(dir),
            index: Arc::new(Mutex::new(HashMap::new())),
            writer: Arc::new(Mutex::new(BufWriterWithPos::new(
                OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&db_path)?,
            )?)),
            reader: Arc::new(Mutex::new(BufReaderWithPos::new(
                OpenOptions::new().read(true).open(&db_path)?,
            )?)),
            uncompacted: Arc::new(Mutex::new(0)),
        };
        kvs.load();
        Ok(kvs)
    }
}
