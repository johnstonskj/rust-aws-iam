use crate::model::containers::OneOrAll;
use crate::model::qstring::QString;
use crate::model::types::*;
use serde::{de, de::Visitor, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::fmt::Display;
use std::str::FromStr;
use std::string::ToString;
use uuid::Uuid;

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// Policy -----------------------------------------------------------------------------------------

impl Policy {
    /// Create a minimal `Policy` with only required fields.
    pub fn new(statement: OneOrAll<Statement>) -> Self {
        Policy {
            version: Some(Self::default_version()),
            id: None,
            statement,
        }
    }

    pub fn default_version() -> Version {
        Version::V2008
    }

    pub fn new_id() -> String {
        random_id("pid_")
    }
}

impl Display for Policy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match serde_json::to_string(self) {
            Ok(json) => write!(f, "{}", json),
            Err(_) => Err(fmt::Error),
        }
    }
}

impl FromStr for Policy {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match serde_json::from_str(s) {
            Ok(policy) => Ok(policy),
            Err(e) => Err(e.to_string()),
        }
    }
}

// Statement --------------------------------------------------------------------------------------

impl Statement {
    /// Create a minimal `Statement` with only required fields.
    ///
    /// # Example
    ///
    /// ```
    /// use aws_iam::model::*;
    /// use aws_iam::model::builder::*;
    /// use std::str::FromStr;
    ///
    /// let statement = Statement::new(
    ///     Effect::Allow,
    ///     Action::Action(OneOrAny::One("s3:ListBucket".parse().unwrap())),
    ///     Resource::Resource(this("arn:aws:s3:::example_bucket")),
    /// );
    /// ```
    ///
    pub fn new(effect: Effect, action: Action, resource: Resource) -> Self {
        Statement {
            sid: None,
            principal: None,
            effect,
            action,
            resource,
            condition: None,
        }
    }

    pub fn new_sid() -> String {
        random_id("sid_")
    }
}

// ConditionOperator ----------------------------------------------------------------------------------

impl ConditionOperator {
    pub fn new(base: GlobalConditionOperator) -> Self {
        match base {
            GlobalConditionOperator::Other(other) => Self::new_other(other),
            base @ _ => ConditionOperator {
                quantifier: None,
                operator: base,
                only_if_exists: false,
            },
        }
    }

    pub fn new_other(condition: QString) -> Self {
        ConditionOperator {
            quantifier: None,
            operator: GlobalConditionOperator::Other(condition),
            only_if_exists: false,
        }
    }

    pub fn for_all(self) -> Self {
        ConditionOperator {
            quantifier: Some(ConditionOperatorQuantifier::ForAllValues),
            ..self
        }
    }

    pub fn for_any(self) -> Self {
        ConditionOperator {
            quantifier: Some(ConditionOperatorQuantifier::ForAnyValue),
            ..self
        }
    }

    pub fn if_exists(self) -> Self {
        ConditionOperator {
            only_if_exists: true,
            ..self
        }
    }
}

impl Display for ConditionOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "{}{:?}{}",
            match &self.quantifier {
                Some(quantifier) => format!("{:?}:", quantifier),
                None => "".to_string(),
            },
            self.operator,
            if self.only_if_exists { "IfExists" } else { "" }
        )
    }
}

#[derive(Debug, PartialEq)]
pub enum ConditionOperatorError {
    EmptyString,
    InvalidQuantifier,
    InvalidGlobalConditionOperator,
    InvalidFormat,
}

impl Display for ConditionOperatorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self)
    }
}

impl FromStr for ConditionOperator {
    type Err = ConditionOperatorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(ConditionOperatorError::EmptyString);
        }
        let mut s = s.clone();
        let quantifier = if s.starts_with("ForAllValues:") {
            s = &s[13..];
            Some(ConditionOperatorQuantifier::ForAllValues)
        } else if s.starts_with("ForAnyValue:") {
            s = &s[12..];
            Some(ConditionOperatorQuantifier::ForAnyValue)
        } else {
            None
        };
        if s.contains(":") {
            return Err(ConditionOperatorError::InvalidQuantifier);
        }
        let only_if_exists = if s.ends_with("IfExists") {
            let end = s.len() - 8;
            s = &s[0..end];
            true
        } else {
            false
        };
        if !s.chars().all(|c| c.is_ascii_alphabetic()) {
            return Err(ConditionOperatorError::InvalidGlobalConditionOperator);
        }
        let operator = match s {
            "StringEquals" => GlobalConditionOperator::StringEquals,
            "StringNotEquals" => GlobalConditionOperator::StringNotEquals,
            "StringEqualsIgnoreCase" => GlobalConditionOperator::StringEqualsIgnoreCase,
            "StringNotEqualsIgnoreCase" => GlobalConditionOperator::StringNotEqualsIgnoreCase,
            "StringLike" => GlobalConditionOperator::StringLike,
            "StringNotLike" => GlobalConditionOperator::StringNotLike,
            "NumericEquals" => GlobalConditionOperator::NumericEquals,
            "NumericNotEquals" => GlobalConditionOperator::NumericNotEquals,
            "NumericLessThan" => GlobalConditionOperator::NumericLessThan,
            "NumericLessThanEquals" => GlobalConditionOperator::NumericLessThanEquals,
            "NumericGreaterThan" => GlobalConditionOperator::NumericGreaterThan,
            "NumericGreaterThanEquals" => GlobalConditionOperator::NumericGreaterThanEquals,
            "DateEquals" => GlobalConditionOperator::DateEquals,
            "DateNotEquals" => GlobalConditionOperator::DateNotEquals,
            "DateLessThan" => GlobalConditionOperator::DateLessThan,
            "DateLessThanEquals" => GlobalConditionOperator::DateLessThanEquals,
            "DateGreaterThan" => GlobalConditionOperator::DateGreaterThan,
            "DateGreaterThanEquals" => GlobalConditionOperator::DateGreaterThanEquals,
            "Bool" => GlobalConditionOperator::Bool,
            "BinaryEquals" => GlobalConditionOperator::BinaryEquals,
            "IpAddress" => GlobalConditionOperator::IpAddress,
            "NotIpAddress" => GlobalConditionOperator::NotIpAddress,
            "ArnEquals" => GlobalConditionOperator::ArnEquals,
            "ArnLike" => GlobalConditionOperator::ArnLike,
            "ArnNotEquals" => GlobalConditionOperator::ArnNotEquals,
            "ArnNotLike" => GlobalConditionOperator::ArnNotLike,
            "Null" => GlobalConditionOperator::Null,
            other => GlobalConditionOperator::Other(other.parse().unwrap()),
        };
        Ok(ConditionOperator {
            quantifier,
            operator,
            only_if_exists,
        })
    }
}

impl Serialize for ConditionOperator {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for ConditionOperator {
    fn deserialize<D>(deserializer: D) -> Result<ConditionOperator, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(ConditionOperatorVisitor)
    }
}

struct ConditionOperatorVisitor;

impl<'de> Visitor<'de> for ConditionOperatorVisitor {
    type Value = ConditionOperator;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a condition type string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ConditionOperator::from_str(&value).map_err(de::Error::custom)
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_str(&value)
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn random_id(prefix: &str) -> String {
    format!(
        "{}{}",
        prefix,
        Uuid::new_v4()
            .to_hyphenated()
            .encode_lower(&mut Uuid::encode_buffer())
    )
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

    #[test]
    fn test_condition_type_ok() {
        assert!(ConditionOperator::from_str("StringEquals").is_ok());
        assert!(ConditionOperator::from_str("Null").is_ok());
        assert!(ConditionOperator::from_str("FooTest").is_ok());
        assert!(ConditionOperator::from_str("ForAllValues:Null").is_ok());
        assert!(ConditionOperator::from_str("NullIfExists").is_ok());
        assert!(ConditionOperator::from_str("ForAllValues:NullIfExists").is_ok());
    }

    #[test]
    fn test_condition_type_parts_ok() {
        let c_type = ConditionOperator::from_str("ForAllValues:NullIfExists").unwrap();
        assert_eq!(
            c_type.quantifier,
            Some(ConditionOperatorQuantifier::ForAllValues)
        );
        assert_eq!(c_type.operator, GlobalConditionOperator::Null);
        assert_eq!(c_type.only_if_exists, true);
    }

    #[test]
    fn test_condition_type_bad() {
        assert_eq!(
            ConditionOperator::from_str(""),
            Err(ConditionOperatorError::EmptyString)
        );
        assert_eq!(
            ConditionOperator::from_str("ForNone:StringEquals"),
            Err(ConditionOperatorError::InvalidQuantifier)
        );
        assert_eq!(
            ConditionOperator::from_str("String="),
            Err(ConditionOperatorError::InvalidGlobalConditionOperator)
        );
    }
}
