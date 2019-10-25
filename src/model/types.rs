use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Identifies the IAM version to validate.
///
#[derive(Debug, Serialize, Deserialize)]
pub enum Version {
    #[serde(rename = "2008-10-17")]
    /// IAM version 2008-10-17
    V2008,
    #[serde(rename = "2012-10-17")]
    /// IAM version 2012-10-17
    V2012,
}

///
/// An IAM policy resource.
///
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Policy {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The IAM version of the policy grammar used in this resource
    pub version: Option<Version>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The identifier of this policy, if any
    pub id: Option<String>,
    /// One or more policy statements
    pub statement: Statements,
}

///
/// The effect, or outcome, of a statement.
///
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Effect {
    Allow,
    Deny,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Qualified {
    #[serde(rename = "*")]
    Any,
    One(String),
    AnyOf(Vec<String>),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Action {
    Action(Qualified),
    NotAction(Qualified),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PrincipalType {
    AWS,
    Federated,
    Service,
    CanonicalUser,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Principal {
    Principal(HashMap<PrincipalType, Qualified>),
    NotPrincipal(HashMap<PrincipalType, Qualified>),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Resource {
    Resource(Qualified),
    NotResource(Qualified),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConditionTypeQuantifier {
    ForAllValues,
    ForAnyValue,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConditionType {
    pub quantifier: Option<ConditionTypeQuantifier>,
    pub base_type: BaseConditionType,
    pub only_if_exists: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BaseConditionType {
    StringEquals,
    StringNotEquals,
    StringEqualsIgnoreCase,
    StringNotEqualsIgnoreCase,
    StringLike,
    StringNotLike,
    NumericEquals,
    NumericNotEquals,
    NumericLessThan,
    NumericLessThanEquals,
    NumericGreaterThan,
    NumericGreaterThanEquals,
    DateEquals,
    DateNotEquals,
    DateLessThan,
    DateLessThanEquals,
    DateGreaterThan,
    DateGreaterThanEquals,
    Bool,
    BinaryEquals,
    IpAddress,
    NotIpAddress,
    ArnEquals,
    ArnLike,
    ArnNotEquals,
    ArnNotLike,
    Other(String),
    Null,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConditionValue {
    String(String),
    Integer(i64),
    Float(f64),
    Bool(bool),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConditionValues {
    One(ConditionValue),
    All(Vec<ConditionValue>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Statements {
    One(Statement), // TODO this makes a large enum, consider One(Box<Statement>)
    All(Vec<Statement>),
}

///
/// A statement within a policy.
///
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Statement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal: Option<Principal>,
    pub effect: Effect,
    pub action: Action,
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<HashMap<ConditionType, HashMap<String, ConditionValues>>>,
}
