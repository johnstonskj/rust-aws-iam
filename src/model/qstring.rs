/*!
Provides a namespace-qualified string.
*/

use regex::Regex;
use serde::{de, de::Visitor, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::Display;
use std::fmt::{self, Error, Formatter};
use std::str::FromStr;
use std::string::ToString;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// A Qualified String, i.e. `prefix:value`
///
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct QString {
    pub(crate) qualifier: Option<String>,
    pub(crate) value: String,
}

///
/// Errors that may arise when parsing using `FromStr::from_str()`.
///
#[derive(Clone, Debug)]
pub enum QStringError {
    /// One part of the qualified string is invalid.
    ComponentInvalid,
    /// Only one ':' is allowed.
    TooManySeparators,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

const SEPARATOR: &str = ":";

impl QString {
    ///
    /// Create a new qualified string with both `qualifier` and `value`.
    ///
    pub fn new(qualifier: String, value: String) -> Self {
        match (validate_part(&qualifier), validate_part(&value)) {
            (Ok(_), Ok(_)) => QString {
                qualifier: Some(qualifier),
                value,
            },
            _ => panic!("Invalid format for qualifier or value"),
        }
    }

    ///
    /// Create a new qualified string with only a `value`.
    ///
    pub fn unqualified(value: String) -> Self {
        match validate_part(&value) {
            Ok(_) => QString {
                qualifier: None,
                value,
            },
            _ => panic!("Invalid format for value: '{}'", value),
        }
    }

    ///
    /// Construct an empty qualified string
    ///
    pub fn empty() -> Self {
        QString {
            qualifier: None,
            value: "".to_string(),
        }
    }

    ///
    /// Determines if the `value` part of this qualified string is empty.
    ///
    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }

    ///
    /// Return the `qualifier` part of this qualified string.
    ///
    pub fn qualifier(&self) -> &Option<String> {
        &self.qualifier
    }

    ///
    /// Return the `value` part of this qualified string.
    ///
    pub fn value(&self) -> &String {
        &self.value
    }
}

impl Display for QString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match &self.qualifier {
            Some(qualifier) => write!(f, "{}{}{}", qualifier, SEPARATOR, &self.value),
            None => write!(f, "{}", &self.value),
        }
    }
}

impl FromStr for QString {
    type Err = QStringError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Ok(QString::unqualified(s.to_string()))
        } else {
            let parts = s.split(SEPARATOR).collect::<Vec<&str>>();
            match parts.len() {
                1 => Ok(QString::unqualified(validate_part(parts.get(0).unwrap())?)),
                2 => Ok(QString::new(
                    validate_part(parts.get(0).unwrap())?,
                    validate_part(parts.get(1).unwrap())?,
                )),
                _ => Err(QStringError::TooManySeparators),
            }
        }
    }
}
impl Serialize for QString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for QString {
    fn deserialize<D>(deserializer: D) -> Result<QString, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(QStringVisitor)
    }
}

struct QStringVisitor;

impl<'de> Visitor<'de> for QStringVisitor {
    type Value = QString;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a qualified string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        QString::from_str(value).map_err(de::Error::custom)
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_str(&value)
    }
}

impl Display for QStringError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{:?}", self)
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn validate_part(part: &str) -> Result<String, QStringError> {
    lazy_static! {
        static ref ID: Regex = Regex::new(r"^(\*|[a-zA-Z\*][a-zA-Z0-9\-_\*/]*)$").unwrap();
    }
    if part.is_empty() || ID.is_match(part) {
        Ok(part.to_string())
    } else {
        Err(QStringError::ComponentInvalid)
    }
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_new() {
        let q_string = QString::new(String::from("foo"), String::from("bar"));
        assert_eq!(q_string.qualifier(), &Some(String::from("foo")));
        assert_eq!(q_string.value(), &String::from("bar"));
    }

    #[test]
    fn test_valid_unqualified() {
        let q_string = QString::unqualified(String::from("bar"));
        assert_eq!(q_string.qualifier(), &None);
        assert_eq!(q_string.value(), &String::from("bar"));
    }

    #[test]
    fn test_valid_from_str() {
        let q_string = QString::from_str("foo:bar").unwrap();
        assert_eq!(q_string.qualifier(), &Some(String::from("foo")));
        assert_eq!(q_string.value(), &String::from("bar"));

        let q_string = QString::from_str("bar").unwrap();
        assert_eq!(q_string.qualifier(), &None);
        assert_eq!(q_string.value(), &String::from("bar"));
    }
}
