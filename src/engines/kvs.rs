use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::path::PathBuf;
use std::time::SystemTime;

use serde::{Deserialize, Serialize};
use serde_json::Deserializer;

use crate::{KvsEngine, KvsError, Result};

const COMPACTION_THRESHOLD: u64 = 1024 * 1024;

/// KvStore struct
#[derive(Debug, Clone)]
pub struct KvStore {
    path: PathBuf,
    index: HashMap<String, ValueLocation>,
    writer: BufWriterWithPos,
    reader: BufReaderWithPos,
    // uncompacted data bytes
    uncompacted: u64,
}

#[derive(Debug)]
pub struct BufWriterWithPos {
    writer: BufWriter<File>,
    pos: u64,
}

impl Clone for BufWriterWithPos {
    fn clone(&self) -> Self {
        todo!()
    }
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
impl Clone for BufReaderWithPos {
    fn clone(&self) -> Self {
        todo!()
    }
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
        todo!()
        // if let Some(value_location) = self.index.get(&key) {
        //     self.reader.seek(SeekFrom::Start(value_location.pos))?;
        //     let buf_reader = self.reader.get_mut().take(value_location.len);
        //     match serde_json::from_reader(buf_reader)? {
        //         Operation::Set { value, .. } => Ok(Some(value)),
        //         _ => Err(KvsError::UnsupportedOperation),
        //     }
        // } else {
        //     return Ok(None);
        // }
    }
    fn remove(&self, key: String) -> Result<()> {
        todo!()
        // if self.index.remove(&key).is_none() {
        //     return Err(KvsError::KeyNotFound);
        // };
        // let row = Operation::Rm { key };
        // serde_json::to_writer(&self.writer, &row)?;
        // self.writer.flush()?;
        // Ok(())
    }
    fn set(&self, key: String, value: String) -> Result<()> {
        todo!()
        //     let row = Operation::Set {
        //         key: key.clone(),
        //         value,
        //     };
        //     let pos = self.writer.pos;
        //     serde_json::to_writer(&mut self.writer, &row)?;
        //     self.writer.flush()?;
        //     if let Some(v) = self.index.insert(
        //         key,
        //         ValueLocation {
        //             pos,
        //             len: self.writer.pos - pos,
        //         },
        //     ) {
        //         self.uncompacted += v.len;
        //     };
        //     if self.uncompacted >= COMPACTION_THRESHOLD {
        //         self.compact()?;
        //     }
        //     Ok(())
    }
}

impl KvStore {
    fn load(&mut self) {
        let mut stream = Deserializer::from_reader(&mut self.reader).into_iter::<Operation>();
        let mut pos: u64 = 0;
        while let Some(op) = stream.next() {
            let new_pos = stream.byte_offset() as u64;
            match op.unwrap() {
                Operation::Set { key, .. } => {
                    if let Some(v) = self.index.insert(
                        key,
                        ValueLocation {
                            pos,
                            len: new_pos - pos,
                        },
                    ) {
                        self.uncompacted += v.len;
                    };
                }
                Operation::Rm { key } => {
                    let v = self.index.remove(&key).unwrap();
                    self.uncompacted += v.len;
                }
            }
            pos = new_pos;
        }
    }
    fn compact(&mut self) -> Result<()> {
        let mut archive_path = self.path.clone();
        archive_path.push(format!("db.archive.{:?}", SystemTime::now()));
        let mut current_path = self.path.clone();
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
        for v in self.index.values_mut() {
            let cur_reader = reader.get_mut();
            cur_reader.seek(SeekFrom::Start(v.pos))?;
            let mut data_reader = cur_reader.take(v.len);
            let len = io::copy(&mut data_reader, &mut writer)?;
            *v = ValueLocation { pos: new_pos, len };
            new_pos += len;
        }
        writer.flush()?;
        self.writer = writer;
        self.uncompacted = 0;
        fs::remove_file(&archive_path)?;
        Ok(())
    }

    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        let dir = path.into();
        let mut db_path: PathBuf = dir.clone();
        db_path.push("db");
        let mut kvs = KvStore {
            path: dir,
            index: HashMap::new(),
            writer: BufWriterWithPos::new(
                OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&db_path)?,
            )?,
            reader: BufReaderWithPos::new(OpenOptions::new().read(true).open(&db_path)?)?,
            uncompacted: 0,
        };
        kvs.load();
        Ok(kvs)
    }
}
