/*!
Provides basic file read/writer or policies.
*/
use crate::model::Policy;
use std::fs::File;
use std::io;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::PathBuf;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Errors possible with file read/write.
///
#[derive(Debug)]
pub enum Error {
    /// Wrapper for any IO error that occurs during reading.
    ReadingFile(io::Error),
    /// Wrapper for any IO error that occurs duriug writing.
    WritingFile(io::Error),
    /// Wrapper for Serde error serializing object to JSON.
    SerializingPolicy(String),
    /// Wrapper for Serde error de-serializing JSON to object.
    DeserializingJson(String),
    /// The policy read from a file is not valid.
    InvalidPolicy,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// Read a `Policy` document from `path`.
///
pub fn read_from_file(path: &PathBuf) -> Result<Policy, Error> {
    match File::open(path) {
        Ok(f) => read_from_reader(f),
        Err(e) => Err(Error::ReadingFile(e)),
    }
}

///
/// Read a `Policy` document from any implementation of `std::io::Read`.
///
pub fn read_from_reader<R>(reader: R) -> Result<Policy, Error>
where
    R: Read + Sized,
{
    let reader = BufReader::new(reader);
    match serde_json::from_reader(reader) {
        Ok(policy) => Ok(policy),
        Err(e) => Err(Error::DeserializingJson(e.to_string())),
    }
}

///
/// Write `policy` object to `path`.
///
pub fn write_to_file(path: &PathBuf, policy: &Policy) -> Result<(), Error> {
    match File::open(path) {
        Ok(f) => write_to_writer(f, policy),
        Err(e) => Err(Error::WritingFile(e)),
    }
}

pub fn write_to_writer<W>(writer: W, policy: &Policy) -> Result<(), Error>
where
    W: Write + Sized,
{
    let writer = BufWriter::new(writer);
    match serde_json::to_writer_pretty(writer, policy) {
        Ok(policy) => Ok(policy),
        Err(e) => Err(Error::SerializingPolicy(e.to_string())),
    }
}
