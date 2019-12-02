/*!
Provides basic file read/write functions for policies.

This module wraps basic read and write operations and the relevant Serde serialization and
deserialization logic. Both read and write functions come in two forms, one which takes a
file name in the form of a `PathBuf` and one which either takes an implementation of
`std::io::Read` or `std::io::Write`.

# Example

The following reads a policy document from a JSON file and returns the parsed form.

```rust
use aws_iam::{io, model::*};
use std::path::PathBuf;

let policy = io::read_from_file(
        &PathBuf::from("tests/data/good/example-021.json")
    ).expect("Error reading file");
```
*/

use crate::model::Policy;
use std::fs::OpenOptions;
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
/// Read a `Policy` document from the file at `path`.
///
pub fn read_from_file(path: &PathBuf) -> Result<Policy, Error> {
    match OpenOptions::new().read(true).open(path) {
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
/// Read a `Policy` document from a string.
///
pub fn read_from_string(s: &str) -> Result<Policy, Error> {
    read_from_reader(s.as_bytes())
}

///
/// Write the `policy` object to a file at `path`, this will create a file if it does
/// not exist and overwrite any file if it exists.
///
pub fn write_to_file(path: &PathBuf, policy: &Policy) -> Result<(), Error> {
    match OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
    {
        Ok(f) => write_to_writer(f, policy),
        Err(e) => Err(Error::WritingFile(e)),
    }
}

///
/// Write the `policy` object to any implementation of `std::io::Write`.
///
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
