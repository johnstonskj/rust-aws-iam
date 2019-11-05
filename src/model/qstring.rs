/*!
Provides a namespace-qualified string.
*/

use regex::Regex;
use serde::export::fmt::Error;
use serde::export::Formatter;
use serde::{de, de::Visitor, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::fmt::Display;
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

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

const SEPARATOR: &str = ":";

impl QString {
    pub fn new(qualifier: String, value: String) -> Self {
        match (validate_part(&qualifier), validate_part(&value)) {
            (Ok(_), Ok(_)) => QString {
                qualifier: Some(qualifier),
                value,
            },
            _ => panic!("Invalid format for qualifier or value"),
        }
    }

    pub fn unqualified(value: String) -> Self {
        match validate_part(&value) {
            Ok(_) => QString {
                qualifier: None,
                value,
            },
            _ => panic!("Invalid format for value"),
        }
    }

    pub fn qualifier(&self) -> &Option<String> {
        &self.qualifier
    }

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

#[derive(Clone, Debug)]
pub enum QStringError {
    EmptyString,
    ComponentInvalid,
    TooManySeparators,
}

impl FromStr for QString {
    type Err = QStringError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Err(QStringError::EmptyString)
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
        QString::from_str(&value).map_err(de::Error::custom)
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
        static ref ID: Regex = Regex::new(r"^(\*|[a-zA-Z][a-zA-Z0-9\-_]*\*?)$").unwrap();
    }
    if ID.is_match(part) {
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
