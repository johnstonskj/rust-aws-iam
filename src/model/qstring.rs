use regex::Regex;
use std::fmt::{Display, Error, Formatter};
use std::str::FromStr;
use std::string::ToString;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct QString {
    qualifier: Option<String>,
    value: String,
}

#[derive(Clone, Debug)]
pub enum QStringError {
    EmptyString,
    ComponentInvalid,
    TooManySeparators,
}

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
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
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

fn validate_part(part: &str) -> Result<String, QStringError> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"[a-zA-Z][a-zA-Z0-9\-_]").unwrap();
    }
    if RE.is_match(part) {
        Ok(part.to_string())
    } else {
        Err(QStringError::ComponentInvalid)
    }
}

#[cfg(test)]
mod tests {
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
