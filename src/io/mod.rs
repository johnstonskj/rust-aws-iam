/*!
Provides basic file read/write functions for policies.

This module wraps basic read and write operations and the relevant Serde serialization and
deserialization logic. Both read and write functions come in two forms, one which takes a
file name in the form of a `PathBuf` and one which either takes an implementation of
`std::io::Read` or `std::io::Write`.

# Example

The following reads a policy document from a JSON file and returns the parsed form.

```rust,ignore
use aws_iam::{io, model::*};
use std::path::PathBuf;

let policy = io::read_from_file(
        &PathBuf::from("tests/data/good/example-021.json")
    ).expect("Error reading file");
```
*/

use crate::error::IamError;
use crate::model::Policy;
use crate::syntax::IamValue;
use serde_json::Value;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::PathBuf;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// Read a `Policy` document from the file at `path`.
///
pub fn read_from_file(path: &PathBuf) -> Result<Policy, IamError> {
    match OpenOptions::new().read(true).open(path) {
        Ok(f) => read_from_reader(f),
        Err(e) => Err(IamError::from(e)),
    }
}

///
/// Read a `Policy` document from any implementation of `std::io::Read`.
///
pub fn read_from_reader<R>(reader: R) -> Result<Policy, IamError>
where
    R: Read + Sized,
{
    let mut reader = reader;
    let mut buffer = String::new();
    let _ = reader.read_to_string(&mut buffer)?;
    read_from_string(&buffer)
}

///
/// Read a `Policy` document from a string.
///
pub fn read_from_string(s: &str) -> Result<Policy, IamError> {
    let v: Value = serde_json::from_str(s)?;
    let policy = Policy::from_json(&v).map_err(IamError::from)?;
    Ok(policy)
}

///
/// Write the `policy` object to a file at `path`, this will create a file if it does
/// not exist and overwrite any file if it exists.
///
pub fn write_to_file(path: &PathBuf, policy: &Policy, pretty: bool) -> Result<(), IamError> {
    match OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
    {
        Ok(f) => write_to_writer(f, policy, pretty),
        Err(e) => Err(IamError::from(e)),
    }
}

///
/// Write the `policy` object to any implementation of `std::io::Write`.
///
pub fn write_to_writer<W>(writer: W, policy: &Policy, pretty: bool) -> Result<(), IamError>
where
    W: Write + Sized,
{
    let mut writer = writer;
    let _ = writer.write(to_string(policy, pretty)?.as_bytes())?;
    Ok(())
}

pub fn to_string(policy: &Policy, pretty: bool) -> Result<String, IamError> {
    let json = policy.to_json().unwrap();
    let json = if pretty {
        serde_json::to_string_pretty(&json)?
    } else {
        serde_json::to_string(&json)?
    };
    Ok(json)
}
