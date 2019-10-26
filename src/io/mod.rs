use crate::model::Policy;
use std::fs::File;
use std::io;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;

#[derive(Debug)]
pub enum Error {
    ReadingFile(io::Error),
    WritingFile(io::Error),
    SerializingPolicy(String),
    DeserializingJson(String),
    InvalidPolicy,
}

pub fn read_from_file(path: &PathBuf) -> Result<Policy, Error> {
    match File::open(path) {
        Ok(f) => {
            let reader = BufReader::new(f);
            match serde_json::from_reader(reader) {
                Ok(policy) => Ok(policy),
                Err(e) => Err(Error::DeserializingJson(e.to_string())),
            }
        }
        Err(e) => Err(Error::ReadingFile(e)),
    }
}

pub fn write_to_file(path: &PathBuf, policy: &Policy) -> Result<(), Error> {
    match File::open(path) {
        Ok(f) => {
            let writer = BufWriter::new(f);
            match serde_json::to_writer_pretty(writer, policy) {
                Ok(policy) => Ok(policy),
                Err(e) => Err(Error::SerializingPolicy(e.to_string())),
            }
        }
        Err(e) => Err(Error::WritingFile(e)),
    }
}
