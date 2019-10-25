// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------
use crate::model::types::*;
use serde::{de, de::Visitor, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::fmt::Display;
use std::str::FromStr;

impl Policy {
    /// Create a minimal `Policy` with only required fields.
    pub fn new(statement: Statements) -> Self {
        Policy {
            version: Some(Version::V2012),
            id: None,
            statement,
        }
    }
}

impl Statement {
    /// Create a minimal `Statement` with only required fields.
    ///
    /// # Example
    ///
    /// ```
    /// use aws_iam::model::*;
    /// use aws_iam::model::builder::*;
    ///
    /// let statement = Statement::new(
    ///     Effect::Allow,
    ///     Action::Action(this("s3:ListBucket")),
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
}

impl ConditionType {
    pub fn new(base: BaseConditionType) -> Self {
        match base {
            BaseConditionType::Other(other) => Self::new_other(other),
            base @ _ => ConditionType {
                quantifier: None,
                base_type: base,
                only_if_exists: false,
            },
        }
    }

    pub fn new_other(condition: String) -> Self {
        assert!(condition.chars().all(|c| c.is_ascii_alphabetic()));
        ConditionType {
            quantifier: None,
            base_type: BaseConditionType::Other(condition),
            only_if_exists: false,
        }
    }

    pub fn for_all(self) -> Self {
        ConditionType {
            quantifier: Some(ConditionTypeQuantifier::ForAllValues),
            ..self
        }
    }

    pub fn for_any(self) -> Self {
        ConditionType {
            quantifier: Some(ConditionTypeQuantifier::ForAnyValue),
            ..self
        }
    }

    pub fn if_exists(self) -> Self {
        ConditionType {
            only_if_exists: true,
            ..self
        }
    }
}

impl Display for ConditionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "{}{:?}{}",
            match &self.quantifier {
                Some(quantifier) => format!("{:?}:", quantifier),
                None => "".to_string(),
            },
            self.base_type,
            if self.only_if_exists { "IfExists" } else { "" }
        )
    }
}

#[derive(Debug, PartialEq)]
pub enum ConditionTypeError {
    EmptyString,
    InvalidQuantifier,
    InvalidBaseConditionType,
    InvalidFormat,
}

impl Display for ConditionTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self)
    }
}

impl FromStr for ConditionType {
    type Err = ConditionTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(ConditionTypeError::EmptyString);
        }
        let mut s = s.clone();
        let quantifier = if s.starts_with("ForAllValues:") {
            s = &s[13..];
            Some(ConditionTypeQuantifier::ForAllValues)
        } else if s.starts_with("ForAnyValue:") {
            s = &s[12..];
            Some(ConditionTypeQuantifier::ForAnyValue)
        } else {
            None
        };
        if s.contains(":") {
            return Err(ConditionTypeError::InvalidQuantifier);
        }
        let only_if_exists = if s.ends_with("IfExists") {
            let end = s.len() - 8;
            s = &s[0..end];
            true
        } else {
            false
        };
        if !s.chars().all(|c| c.is_ascii_alphabetic()) {
            return Err(ConditionTypeError::InvalidBaseConditionType);
        }
        let base_type = match s {
            "StringEquals" => BaseConditionType::StringEquals,
            "StringNotEquals" => BaseConditionType::StringNotEquals,
            "StringEqualsIgnoreCase" => BaseConditionType::StringEqualsIgnoreCase,
            "StringNotEqualsIgnoreCase" => BaseConditionType::StringNotEqualsIgnoreCase,
            "StringLike" => BaseConditionType::StringLike,
            "StringNotLike" => BaseConditionType::StringNotLike,
            "NumericEquals" => BaseConditionType::NumericEquals,
            "NumericNotEquals" => BaseConditionType::NumericNotEquals,
            "NumericLessThan" => BaseConditionType::NumericLessThan,
            "NumericLessThanEquals" => BaseConditionType::NumericLessThanEquals,
            "NumericGreaterThan" => BaseConditionType::NumericGreaterThan,
            "NumericGreaterThanEquals" => BaseConditionType::NumericGreaterThanEquals,
            "DateEquals" => BaseConditionType::DateEquals,
            "DateNotEquals" => BaseConditionType::DateNotEquals,
            "DateLessThan" => BaseConditionType::DateLessThan,
            "DateLessThanEquals" => BaseConditionType::DateLessThanEquals,
            "DateGreaterThan" => BaseConditionType::DateGreaterThan,
            "DateGreaterThanEquals" => BaseConditionType::DateGreaterThanEquals,
            "Bool" => BaseConditionType::Bool,
            "BinaryEquals" => BaseConditionType::BinaryEquals,
            "IpAddress" => BaseConditionType::IpAddress,
            "NotIpAddress" => BaseConditionType::NotIpAddress,
            "ArnEquals" => BaseConditionType::ArnEquals,
            "ArnLike" => BaseConditionType::ArnLike,
            "ArnNotEquals" => BaseConditionType::ArnNotEquals,
            "ArnNotLike" => BaseConditionType::ArnNotLike,
            "Null" => BaseConditionType::Null,
            other => BaseConditionType::Other(other.to_string()),
        };
        Ok(ConditionType {
            quantifier,
            base_type,
            only_if_exists,
        })
    }
}

impl Serialize for ConditionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for ConditionType {
    fn deserialize<D>(deserializer: D) -> Result<ConditionType, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(ConditionTypeVisitor)
    }
}

struct ConditionTypeVisitor;

impl<'de> Visitor<'de> for ConditionTypeVisitor {
    type Value = ConditionType;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a condition type string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        ConditionType::from_str(&value).map_err(de::Error::custom)
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_str(&value)
    }
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_condition_type_ok() {
        assert!(ConditionType::from_str("StringEquals").is_ok());
        assert!(ConditionType::from_str("Null").is_ok());
        assert!(ConditionType::from_str("FooTest").is_ok());
        assert!(ConditionType::from_str("ForAllValues:Null").is_ok());
        assert!(ConditionType::from_str("NullIfExists").is_ok());
        assert!(ConditionType::from_str("ForAllValues:NullIfExists").is_ok());
    }

    #[test]
    fn test_condition_type_bad() {
        assert_eq!(
            ConditionType::from_str(""),
            Err(ConditionTypeError::EmptyString)
        );
        assert_eq!(
            ConditionType::from_str("ForNone:StringEquals"),
            Err(ConditionTypeError::InvalidQuantifier)
        );
        assert_eq!(
            ConditionType::from_str("String="),
            Err(ConditionTypeError::InvalidBaseConditionType)
        );
    }
}
