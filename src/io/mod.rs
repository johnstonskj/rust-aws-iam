/*!
Provides basic file read/writer or policies.
*/
use crate::model::Policy;
use std::fs::File;
use std::io;
use std::io::{BufReader, BufWriter};
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

///
/// Write `policy` object to `path`.
///
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
