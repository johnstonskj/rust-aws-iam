/*!
Provides implementations for the types in `crate::model::types`.
*/

use crate::model::containers::OneOrAll;
use crate::model::qstring::QString;
use crate::model::types::*;
use crate::model::OneOrAny;
use serde::{de, de::Visitor, Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;
use std::str::FromStr;
use std::string::ToString;
use uuid::Uuid;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Error conditions which may arise from `FromStr::from_str()`.
///
#[derive(Debug, PartialEq)]
pub enum ConditionOperatorError {
    /// Empty strings are not valid.
    EmptyString,
    /// The condition quantifier (preceeding ':') is invalid.
    InvalidQuantifier,
    /// Invalid unqualified condition operator.
    InvalidGlobalConditionOperator,
    /// Gobbledegook.
    InvalidFormat,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// Policy -----------------------------------------------------------------------------------------

impl Policy {
    ///
    /// Create a minimal `Policy` with only required fields.
    ///
    pub fn new(statement: OneOrAll<Statement>) -> Self {
        Policy {
            version: Some(Self::default_version()),
            id: None,
            statement,
        }
    }

    ///
    /// The default version for a policy. Specifically according to the IAM documentation
    /// if no version is specified in a document it is assumed to be the 2008 version.
    ///
    pub fn default_version() -> Version {
        Version::V2008
    }

    ///
    /// Construct a new, random, unique, ID for a Policy.
    ///
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
    ///     Resource::this("arn:aws:s3:::example_bucket".to_string()),
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

    ///
    /// Construct a new, random, unique, ID for a Statement.
    ///
    pub fn new_sid() -> String {
        random_id("sid_")
    }
}

// Action, Principal, Resource --------------------------------------------------------------------

impl Action {
    /// Construct a wildcard Action.
    pub fn any() -> Self {
        Action::Action(OneOrAny::Any)
    }

    /// Construct an Action with one value.
    pub fn this(one: QString) -> Self {
        Action::Action(OneOrAny::One(one))
    }

    /// Construct an Action with a list of values.
    pub fn these(any_of: &mut Vec<QString>) -> Self {
        Action::Action(OneOrAny::AnyOf(any_of.drain(0..).collect()))
    }

    /// Construct a negative wildcard Action.
    pub fn none() -> Self {
        Action::NotAction(OneOrAny::Any)
    }

    /// Construct an Action with one negative value.
    pub fn not_this(one: QString) -> Self {
        Action::NotAction(OneOrAny::One(one))
    }

    /// Construct an Action with a list of negative values.
    pub fn not_these(any_of: &mut Vec<QString>) -> Self {
        Action::NotAction(OneOrAny::AnyOf(any_of.drain(0..).collect()))
    }
}

impl Principal {
    /// Construct a wildcard Principal.
    pub fn any(p_type: PrincipalType) -> Self {
        let mut map: HashMap<PrincipalType, OneOrAny> = Default::default();
        map.insert(p_type, OneOrAny::Any);
        Principal::Principal(map)
    }

    /// Construct a Principal with one value.
    pub fn this(p_type: PrincipalType, one: String) -> Self {
        let mut map: HashMap<PrincipalType, OneOrAny> = Default::default();
        map.insert(p_type, OneOrAny::One(one));
        Principal::Principal(map)
    }

    /// Construct a Principal with a list of values.
    pub fn these(p_type: PrincipalType, any_of: &mut Vec<String>) -> Self {
        let mut map: HashMap<PrincipalType, OneOrAny> = Default::default();
        map.insert(p_type, OneOrAny::AnyOf(any_of.drain(0..).collect()));
        Principal::Principal(map)
    }

    /// Construct a negative wildcard Principal.
    pub fn none(p_type: PrincipalType) -> Self {
        let mut map: HashMap<PrincipalType, OneOrAny> = Default::default();
        map.insert(p_type, OneOrAny::Any);
        Principal::NotPrincipal(map)
    }

    /// Construct a Principal with one negative value.
    pub fn not_this(p_type: PrincipalType, one: String) -> Self {
        let mut map: HashMap<PrincipalType, OneOrAny> = Default::default();
        map.insert(p_type, OneOrAny::One(one));
        Principal::NotPrincipal(map)
    }

    /// Construct a Principal with a list of negative values.
    pub fn not_these(p_type: PrincipalType, any_of: &mut Vec<String>) -> Self {
        let mut map: HashMap<PrincipalType, OneOrAny> = Default::default();
        map.insert(p_type, OneOrAny::AnyOf(any_of.drain(0..).collect()));
        Principal::NotPrincipal(map)
    }
}

impl Resource {
    /// Construct a wildcard Resource.
    pub fn any() -> Self {
        Resource::Resource(OneOrAny::Any)
    }

    /// Construct a Resource with one value.
    pub fn this(one: String) -> Self {
        Resource::Resource(OneOrAny::One(one))
    }

    /// Construct a Resource with a list of values.
    pub fn these(any_of: &mut Vec<String>) -> Self {
        Resource::Resource(OneOrAny::AnyOf(any_of.drain(0..).collect()))
    }

    /// Construct a negative wildcard Resource.
    pub fn none() -> Self {
        Resource::NotResource(OneOrAny::Any)
    }

    /// Construct a Resource with one negative value.
    pub fn not_this(one: String) -> Self {
        Resource::NotResource(OneOrAny::One(one))
    }

    /// Construct a Resource with a list of negative values.
    pub fn not_these(any_of: &mut Vec<String>) -> Self {
        Resource::NotResource(OneOrAny::AnyOf(any_of.drain(0..).collect()))
    }
}

// ConditionOperator ------------------------------------------------------------------------------

impl ConditionOperator {
    ///
    /// Construct a new operator using one of the global operators.
    ///
    pub fn new(base: GlobalConditionOperator) -> Self {
        match base {
            GlobalConditionOperator::Other(other) => Self::new_other(other),
            base @ _ => ConditionOperator {
                quantifier: None,
                operator: base,
                if_exists: false,
            },
        }
    }

    ///
    /// Construct a new operator which isn't one of the global ones.
    ///
    pub fn new_other(condition: QString) -> Self {
        ConditionOperator {
            quantifier: None,
            operator: GlobalConditionOperator::Other(condition),
            if_exists: false,
        }
    }

    /// Set the quantifier to _for-all-values_.
    pub fn for_all(self) -> Self {
        ConditionOperator {
            quantifier: Some(ConditionOperatorQuantifier::ForAllValues),
            ..self
        }
    }

    /// Set the quantifier to _for-any-value_.
    pub fn for_any(self) -> Self {
        ConditionOperator {
            quantifier: Some(ConditionOperatorQuantifier::ForAnyValue),
            ..self
        }
    }

    /// Set the value of the constraint to `true`.
    pub fn if_exists(self) -> Self {
        ConditionOperator {
            if_exists: true,
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
            if self.if_exists { "IfExists" } else { "" }
        )
    }
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
        // TODO: regex this.
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
            if_exists: only_if_exists,
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
        assert_eq!(c_type.if_exists, true);
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
