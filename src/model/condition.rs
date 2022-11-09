/*!
One-line description.

```abnf
<condition_block> = "Condition" : { <condition_map> }
<condition_map> = {
  <condition_type_string> : { <condition_key_string> : <condition_value_list> },
  <condition_type_string> : { <condition_key_string> : <condition_value_list> }, ...
}
<condition_value_list> = [<condition_value>, <condition_value>, ...]
<condition_value> = ("string" | "number" | "Boolean")
```

## condition_type_string

Identifies the type of condition being tested,
such as StringEquals, StringLike, NumericLessThan, DateGreaterThanEquals,
Bool, BinaryEquals, IpAddress, ArnEquals, etc. For a complete list of
condition types, see IAM JSON policy elements: Condition operators.

```json
"Condition": {
  "NumericLessThanEquals": {
    "s3:max-keys": "10"
  }
}

"Condition": {
  "Bool": {
    "aws:SecureTransport": "true"
  }
}

"Condition": {
  "StringEquals": {
      "s3:x-amz-server-side-encryption": "AES256"
   }
}
```

## condition_key_string

Identifies the condition key whose value will be tested to determine
whether the condition is met. AWS defines a set of condition keys that are
available in all AWS services, including `aws:PrincipalType`,
`aws:SecureTransport`, and `aws:userid`.

For a list of AWS condition keys, see AWS global condition context keys.
For condition keys that are specific to a service, see the documentation
for that service such as the following:

Specifying Conditions in a Policy in the Amazon Simple Storage Service
User Guide

IAM Policies for Amazon EC2 in the Amazon EC2 User Guide for Linux
Instances.

```json
"Condition":{
  "Bool": {
      "aws:SecureTransport": "true"
   }
}

"Condition": {
  "StringNotEquals": {
      "s3:x-amz-server-side-encryption": "AES256"
   }
}

"Condition": {
  "StringEquals": {
    "aws:ResourceTag/purpose": "test"
  }
}
```

 */

use crate::error::{type_mismatch, unexpected_value_for_type, IamFormatError};
use crate::model::QualifiedName;
use crate::syntax::{
    display_vec_map_to_json, json_type_name, string_vec_from_json, IamProperty, IamValue,
    CONDITION_NAME, CONDITION_OPERATOR_ARN_EQUALS, CONDITION_OPERATOR_ARN_LIKE,
    CONDITION_OPERATOR_ARN_NOT_EQUALS, CONDITION_OPERATOR_ARN_NOT_LIKE,
    CONDITION_OPERATOR_BINARY_EQUALS, CONDITION_OPERATOR_BOOL, CONDITION_OPERATOR_DATE_EQUALS,
    CONDITION_OPERATOR_DATE_GREATER_THAN, CONDITION_OPERATOR_DATE_GREATER_THAN_EQUALS,
    CONDITION_OPERATOR_DATE_LESS_THAN, CONDITION_OPERATOR_DATE_LESS_THAN_EQUALS,
    CONDITION_OPERATOR_DATE_NOT_EQUALS, CONDITION_OPERATOR_IP_ADDRESS,
    CONDITION_OPERATOR_NOT_IP_ADDRESS, CONDITION_OPERATOR_NULL, CONDITION_OPERATOR_NUMERIC_EQUALS,
    CONDITION_OPERATOR_NUMERIC_GREATER_THAN, CONDITION_OPERATOR_NUMERIC_GREATER_THAN_EQUALS,
    CONDITION_OPERATOR_NUMERIC_LESS_THAN, CONDITION_OPERATOR_NUMERIC_LESS_THAN_EQUALS,
    CONDITION_OPERATOR_NUMERIC_NOT_EQUALS, CONDITION_OPERATOR_STRING_EQUALS,
    CONDITION_OPERATOR_STRING_EQUALS_IGNORE_CASE, CONDITION_OPERATOR_STRING_LIKE,
    CONDITION_OPERATOR_STRING_NOT_EQUALS, CONDITION_OPERATOR_STRING_NOT_EQUALS_IGNORE_CASE,
    CONDITION_OPERATOR_STRING_NOT_LIKE, CONDITION_QUANTIFIER_FOR_ALL, CONDITION_QUANTIFIER_FOR_ANY,
    CONDITION_QUANTIFIER_IF_EXISTS, CONDITION_VALUE_NAME, GLOBAL_CONDITION_KEY_CALLED_VIA,
    GLOBAL_CONDITION_KEY_CALLED_VIA_FIRST, GLOBAL_CONDITION_KEY_CALLED_VIA_LAST,
    GLOBAL_CONDITION_KEY_CURRENT_TIME, GLOBAL_CONDITION_KEY_EPOCH_TIME,
    GLOBAL_CONDITION_KEY_FEDERATED_PROVIDER, GLOBAL_CONDITION_KEY_MULTIFACTOR_AUTH_AGE,
    GLOBAL_CONDITION_KEY_MULTIFACTOR_AUTH_PRESENT, GLOBAL_CONDITION_KEY_NAMESPACE,
    GLOBAL_CONDITION_KEY_PRINCIPAL_ACCOUNT, GLOBAL_CONDITION_KEY_PRINCIPAL_ARN,
    GLOBAL_CONDITION_KEY_PRINCIPAL_IS_AWS_SERVICE, GLOBAL_CONDITION_KEY_PRINCIPAL_ORG_ID,
    GLOBAL_CONDITION_KEY_PRINCIPAL_ORG_PATHS, GLOBAL_CONDITION_KEY_PRINCIPAL_SERVICE_NAME,
    GLOBAL_CONDITION_KEY_PRINCIPAL_SERVICE_NAMES_LIST, GLOBAL_CONDITION_KEY_PRINCIPAL_TAG,
    GLOBAL_CONDITION_KEY_PRINCIPAL_TYPE, GLOBAL_CONDITION_KEY_REFERER,
    GLOBAL_CONDITION_KEY_REQUESTED_REGION, GLOBAL_CONDITION_KEY_REQUEST_TAG,
    GLOBAL_CONDITION_KEY_RESOURCE_ACCOUNT, GLOBAL_CONDITION_KEY_RESOURCE_ORG_ID,
    GLOBAL_CONDITION_KEY_RESOURCE_ORG_PATHS, GLOBAL_CONDITION_KEY_RESOURCE_TAG,
    GLOBAL_CONDITION_KEY_SECURE_TRANSPORT, GLOBAL_CONDITION_KEY_SOURCE_ACCOUNT,
    GLOBAL_CONDITION_KEY_SOURCE_ARN, GLOBAL_CONDITION_KEY_SOURCE_IDENTITY,
    GLOBAL_CONDITION_KEY_SOURCE_IP, GLOBAL_CONDITION_KEY_SOURCE_VPC,
    GLOBAL_CONDITION_KEY_SOURCE_VPCE, GLOBAL_CONDITION_KEY_TAG_KEYS,
    GLOBAL_CONDITION_KEY_TOKEN_ISSUE_TIME, GLOBAL_CONDITION_KEY_USERID,
    GLOBAL_CONDITION_KEY_USERNAME, GLOBAL_CONDITION_KEY_USER_AGENT,
    GLOBAL_CONDITION_KEY_VIA_AWS_SERVICE, GLOBAL_CONDITION_KEY_VPC_SOURCE_IP,
    JSON_TYPE_NAME_OBJECT, NAMESPACE_SEPARATOR,
};
use lazy_static::lazy_static;
use regex::{Captures, Regex};
use serde_json::{Map, Value};
use std::collections::HashMap;
use std::fmt::Display;
use std::iter::FromIterator;
use std::ops::Deref;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub struct Condition(HashMap<Operator, Match>);

#[derive(Debug, Clone, PartialEq)]
pub struct Match(HashMap<QualifiedName, Vec<ConditionValue>>);

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ConditionValue(String);

///
/// Pulls apart the string form of an operator used by IAM. It identifies the
/// quantifiers which are used as string prefixes and recognizes the _if exist_
/// suffix as well.
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Operator {
    /// Used to test multiple keys or multiple values for a single key in a request.
    pub quantifier: Option<Quantifier>,
    /// The condition operator you choose to use.
    pub operator: GlobalOperator,
    /// You use this to say "If the policy key is present in the context of the
    /// request, process the key as specified in the policy. If the key is not
    /// present, evaluate the condition element as true." Other condition elements
    /// in the statement can still result in a nonmatch, but not a missing key
    /// when checked with ...`IfExists`.
    pub if_exists: bool,
}

///
/// You can use the Condition element of a policy to test multiple keys or multiple
/// values for a single key in a request. You can use condition keys to test the
/// values of the matching keys in the request. For example, you can use a condition
/// key to control access to specific attributes of a DynamoDB table or to an Amazon
/// EC2 instance based on tags.
///
/// From [Creating a Condition with Multiple Keys or
/// Values](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_multi-value-conditions.html).
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Quantifier {
    /// The condition **must** hold true for **all** values provided.
    ForAllValues,
    /// The condition **must** hold true for **at least** one value provided.
    ForAnyValue,
}

///
/// Use condition operators in the `Condition` element to match the condition
/// key and value in the policy against values in the request context.
///
/// The condition operator that you can use in a policy depends on the condition
/// key you choose. You can choose a global condition key or a service-specific
/// condition key.
///
/// From [IAM JSON Policy Elements: Condition
/// Operators](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_elements_condition_operators.html).
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GlobalOperator {
    // ----- String Condition Operators
    /// Exact matching, case sensitive
    StringEquals,
    /// Negated matching
    StringNotEquals,
    /// Exact matching, ignoring case
    StringEqualsIgnoreCase,
    /// Negated matching, ignoring case
    StringNotEqualsIgnoreCase,
    /// Case-sensitive matching. The values can include a multi-character
    /// match wildcard (*) or a single-character match wildcard (?) anywhere
    /// in the string.
    StringLike,
    /// Negated case-sensitive matching. The values can include a multi-character
    /// match wildcard (*) or a single-character match wildcard (?) anywhere
    /// in the string.
    StringNotLike,
    // ----- Numeric Condition Operators
    /// Matching
    NumericEquals,
    /// Negated matching
    NumericNotEquals,
    /// "Less than" matching
    NumericLessThan,
    /// "Less than or equals" matching
    NumericLessThanEquals,
    /// "Greater than" matching
    NumericGreaterThan,
    /// "Greater than or equals" matching
    NumericGreaterThanEquals,
    // ----- Date Condition Operators
    /// Matching a specific date
    DateEquals,
    /// Negated matching
    DateNotEquals,
    /// Matching before a specific date and time
    DateLessThan,
    /// Matching at or before a specific date and time
    DateLessThanEquals,
    /// Matching after a specific a date and time
    DateGreaterThan,
    /// Matching at or after a specific date and time
    DateGreaterThanEquals,
    // ----- Boolean Condition Operators
    /// Boolean matching
    Bool,
    // ----- Binary Condition Operators
    /// The BinaryEquals condition operator let you construct Condition
    /// elements that test key values that are in binary format. It compares
    /// the value of the specified key byte for byte against a base-64
    /// encoded representation of the binary value in the policy.
    BinaryEquals,
    // ----- IP Address Condition Operators
    /// The specified IP address or range
    IpAddress,
    /// ll IP addresses except the specified IP address or range
    NotIpAddress,
    // ----- ARN Condition Operators
    /// Case-sensitive matching of the ARN. Each of the six colon-delimited
    /// components of the ARN is checked separately and each can include a
    /// multi-character match wildcard (*) or a single-character match
    /// wildcard (?).
    ArnEquals,
    /// Negated equality for ARN.
    ArnNotEquals,
    /// Case-sensitive matching of the ARN. Each of the six colon-delimited
    /// components of the ARN is checked separately and each can include a
    /// multi-character match wildcard (*) or a single-character match
    /// wildcard (?).
    ArnLike,
    /// Negated matching for ARN.
    ArnNotLike,
    // ------ Check Existence of Condition Keys
    /// Use a Null condition operator to check if a condition key is
    /// present at the time of authorization. In the policy statement, use
    /// either true (the key doesn't exist — it is null) or false (the key
    /// exists and its value is not null).
    Null,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GlobalConditionKey {
    CalledVia,
    CalledViaFirst,
    CalledViaLast,
    CurrentTime,
    EpochTime,
    FederatedProvider,
    MultiFactorAuthAge,
    MultiFactorAuthPresent,
    PrincipalAccount,
    PrincipalArn,
    PrincipalIsAWSService,
    PrincipalOrgID,
    PrincipalOrgPaths,
    PrincipalServiceName,
    PrincipalServiceNamesList,
    PrincipalTag,
    PrincipalType,
    Referer,
    RequestedRegion,
    RequestTag,
    ResourceAccount,
    ResourceOrgID,
    ResourceOrgPaths,
    ResourceTag,
    SecureTransport,
    SourceAccount,
    SourceArn,
    SourceIdentity,
    SourceIp,
    SourceVpc,
    SourceVpce,
    TagKeys,
    TokenIssueTime,
    UserAgent,
    UserId,
    UserName,
    ViaAWSService,
    VpcSourceIp,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IamConditionKey {
    AssociatedResourceArn,
    AWSServiceName,
    OrganizationsPolicyId,
    PassedToService,
    PermissionsBoundary,
    PolicyARN,
    ResourceTag,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Deref for Condition {
    type Target = HashMap<Operator, Match>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<HashMap<Operator, Match>> for Condition {
    fn from(v: HashMap<Operator, Match>) -> Self {
        Self(v)
    }
}

impl IamProperty for Condition {
    fn into_json_object(&self, object: &mut Map<String, Value>) -> Result<(), IamFormatError> {
        let results: Result<Vec<(String, Value)>, IamFormatError> = self
            .iter()
            .map(|(k, v)| match v.to_json() {
                Ok(v) => Ok((k.to_string(), v)),
                Err(e) => Err(e),
            })
            .collect();
        let inner_object = Map::from_iter(results?.into_iter());
        object.insert(CONDITION_NAME.to_string(), Value::Object(inner_object));
        Ok(())
    }

    fn from_json_object_optional(value: &Map<String, Value>) -> Result<Option<Self>, IamFormatError>
    where
        Self: Sized,
    {
        if value.contains_key(CONDITION_NAME) {
            let value = value.get(CONDITION_NAME).unwrap();
            if let Value::Object(object) = value {
                let results: Result<Vec<(Operator, Match)>, IamFormatError> = object
                    .iter()
                    .map(
                        |(k, v)| match (Operator::from_str(k), Match::from_json(v)) {
                            (Ok(k), Ok(v)) => Ok((k, v)),
                            (Ok(_), Err(e)) => Err(e),
                            (Err(e), Ok(_)) => Err(e),
                            (Err(e), Err(_)) => Err(e),
                        },
                    )
                    .collect();
                let inner_object = HashMap::from_iter(results?.into_iter());
                Ok(Some(Self(inner_object)))
            } else {
                type_mismatch(CONDITION_NAME, JSON_TYPE_NAME_OBJECT, json_type_name(value)).into()
            }
        } else {
            Ok(None)
        }
    }

    fn from_json_object(_: &Map<String, Value>) -> Result<Self, IamFormatError>
    where
        Self: Sized,
    {
        unimplemented!()
    }
}

impl Condition {
    pub fn string_equals(matches: Match) -> Self {
        Self::new_match(Operator::string_equals(), matches)
    }

    pub fn string_not_equals(matches: Match) -> Self {
        Self::new_match(Operator::string_not_equals(), matches)
    }

    pub fn string_equals_ignore_case(matches: Match) -> Self {
        Self::new_match(Operator::string_equals_ignore_case(), matches)
    }

    pub fn string_not_equals_ignore_case(matches: Match) -> Self {
        Self::new_match(Operator::string_not_equals_ignore_case(), matches)
    }

    pub fn string_not_like(matches: Match) -> Self {
        Self::new_match(Operator::string_not_like(), matches)
    }

    pub fn numeric_equals(matches: Match) -> Self {
        Self::new_match(Operator::numeric_equals(), matches)
    }

    pub fn numeric_not_equals(matches: Match) -> Self {
        Self::new_match(Operator::numeric_not_equals(), matches)
    }

    pub fn numeric_less_than(matches: Match) -> Self {
        Self::new_match(Operator::numeric_less_than(), matches)
    }

    pub fn numeric_less_than_or_equals(matches: Match) -> Self {
        Self::new_match(Operator::numeric_less_than_or_equals(), matches)
    }

    pub fn numeric_greater_than(matches: Match) -> Self {
        Self::new_match(Operator::numeric_greater_than(), matches)
    }

    pub fn numeric_greater_than_or_equals(matches: Match) -> Self {
        Self::new_match(Operator::numeric_greater_than_or_equals(), matches)
    }

    pub fn date_equals(matches: Match) -> Self {
        Self::new_match(Operator::date_equals(), matches)
    }

    pub fn date_not_equals(matches: Match) -> Self {
        Self::new_match(Operator::date_not_equals(), matches)
    }

    pub fn date_less_than(matches: Match) -> Self {
        Self::new_match(Operator::date_less_than(), matches)
    }

    pub fn date_less_than_or_equals(matches: Match) -> Self {
        Self::new_match(Operator::date_less_than_or_equals(), matches)
    }

    pub fn date_greater_than(matches: Match) -> Self {
        Self::new_match(Operator::date_greater_than(), matches)
    }

    pub fn date_greater_than_or_equals(matches: Match) -> Self {
        Self::new_match(Operator::date_greater_than_or_equals(), matches)
    }

    pub fn bool_equals(matches: Match) -> Self {
        Self::new_match(Operator::bool_equals(), matches)
    }

    pub fn binary_equals(matches: Match) -> Self {
        Self::new_match(Operator::binary_equals(), matches)
    }

    pub fn ip_address(matches: Match) -> Self {
        Self::new_match(Operator::ip_address(), matches)
    }

    pub fn not_ip_address(matches: Match) -> Self {
        Self::new_match(Operator::not_ip_address(), matches)
    }

    pub fn arn_equals(matches: Match) -> Self {
        Self::new_match(Operator::arn_equals(), matches)
    }

    pub fn arn_not_equals(matches: Match) -> Self {
        Self::new_match(Operator::arn_not_equals(), matches)
    }

    pub fn arn_like(matches: Match) -> Self {
        Self::new_match(Operator::arn_like(), matches)
    }

    pub fn arn_not_like(matches: Match) -> Self {
        Self::new_match(Operator::arn_not_like(), matches)
    }

    pub fn null(matches: Match) -> Self {
        Self::new_match(Operator::null(), matches)
    }

    pub fn new_one<S>(operator: Operator, context_key: QualifiedName, value: S) -> Self
    where
        S: Into<ConditionValue>,
    {
        Self::new_match(operator, Match::new_one(context_key, value))
    }

    pub fn new<S>(operator: Operator, context_key: QualifiedName, value: Vec<S>) -> Self
    where
        S: Into<ConditionValue>,
    {
        Self::new_match(operator, Match::new(context_key, value))
    }

    pub fn new_match(operator: Operator, matches: Match) -> Self {
        Self(HashMap::from_iter(vec![(operator, matches)].into_iter()))
    }

    pub fn insert<S>(&mut self, operator: Operator, context_key: QualifiedName, value: S)
    where
        S: Into<ConditionValue>,
    {
        if let Some(existing) = self.0.get_mut(&operator) {
            existing.insert(context_key, value);
        } else {
            self.0.insert(operator, Match::new_one(context_key, value));
        }
    }

    pub fn into_inner(self) -> HashMap<Operator, Match> {
        self.0
    }
}

// ------------------------------------------------------------------------------------------------

impl Deref for Match {
    type Target = HashMap<QualifiedName, Vec<ConditionValue>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<HashMap<QualifiedName, Vec<ConditionValue>>> for Match {
    fn from(v: HashMap<QualifiedName, Vec<ConditionValue>>) -> Self {
        Self(v)
    }
}

impl IamValue for Match {
    fn to_json(&self) -> Result<Value, IamFormatError> {
        display_vec_map_to_json(self)
    }

    fn from_json(value: &Value) -> Result<Self, IamFormatError> {
        if let Value::Object(object) = value {
            let results: Result<Vec<(QualifiedName, Vec<ConditionValue>)>, IamFormatError> = object
                .iter()
                .map(|(k, v)| {
                    match (
                        QualifiedName::from_str(k),
                        string_vec_from_json(v, CONDITION_VALUE_NAME),
                    ) {
                        (Ok(k), Ok(v)) => Ok((k, v)),
                        (Ok(_), Err(e)) => Err(e),
                        (Err(e), Ok(_)) => Err(e),
                        (Err(e), Err(_)) => Err(e),
                    }
                })
                .collect();
            Ok(Self(HashMap::from_iter(results?)))
        } else {
            type_mismatch(CONDITION_NAME, JSON_TYPE_NAME_OBJECT, json_type_name(value)).into()
        }
    }
}

impl Match {
    pub fn new_one<S>(context_key: QualifiedName, value: S) -> Self
    where
        S: Into<ConditionValue>,
    {
        Self::new(context_key, vec![value])
    }

    pub fn new<S>(context_key: QualifiedName, values: Vec<S>) -> Self
    where
        S: Into<ConditionValue>,
    {
        Self(HashMap::from_iter(
            vec![(context_key, values.into_iter().map(|v| v.into()).collect())].into_iter(),
        ))
    }

    pub fn insert<S>(&mut self, context_key: QualifiedName, value: S)
    where
        S: Into<ConditionValue>,
    {
        let context_key = context_key;
        if let Some(existing) = self.0.get_mut(&context_key) {
            existing.push(value.into());
        } else {
            self.0.insert(context_key, vec![value.into()]);
        }
    }

    pub fn extend<S>(&mut self, context_key: QualifiedName, values: Vec<S>)
    where
        S: Into<ConditionValue>,
    {
        let context_key = context_key;
        let values: Vec<ConditionValue> = values.into_iter().map(|v| v.into()).collect();
        if let Some(existing) = self.0.get_mut(&context_key) {
            existing.extend(values);
        } else {
            self.0.insert(context_key, values);
        }
    }

    pub fn into_inner(self) -> HashMap<QualifiedName, Vec<ConditionValue>> {
        self.0
    }
}

// ------------------------------------------------------------------------------------------------

const CHAR_OPERATOR_SEP: char = ':';

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(quantifier) = &self.quantifier {
            write!(f, "{}{}", quantifier, CHAR_OPERATOR_SEP)?;
        }

        write!(f, "{}", self.operator)?;

        if self.if_exists {
            write!(f, "{}", CONDITION_QUANTIFIER_IF_EXISTS)?;
        }

        Ok(())
    }
}

impl FromStr for Operator {
    type Err = IamFormatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts: Vec<&str> = s.split(CHAR_OPERATOR_SEP).collect();
        if parts.len() == 1 || parts.len() == 2 {
            let mut operator = Operator {
                quantifier: None,
                operator: GlobalOperator::Bool,
                if_exists: false,
            };

            if parts.len() == 2 {
                let quantifier = Quantifier::from_str(parts.remove(0))?;
                operator.quantifier = Some(quantifier);
            }

            let mut op_string = parts.remove(0);
            if op_string.ends_with(CONDITION_QUANTIFIER_IF_EXISTS) {
                operator.if_exists = true;
                op_string = &op_string[..op_string.len() - CONDITION_QUANTIFIER_IF_EXISTS.len()];
            }
            operator.operator = GlobalOperator::from_str(op_string)?;
            Ok(operator)
        } else {
            unexpected_value_for_type(CONDITION_NAME, s).into()
        }
    }
}

impl Operator {
    pub fn string_equals() -> Self {
        Self {
            quantifier: None,
            operator: GlobalOperator::StringEquals,
            if_exists: false,
        }
    }

    pub fn string_not_equals() -> Self {
        Self {
            quantifier: None,
            operator: GlobalOperator::StringNotEquals,
            if_exists: false,
        }
    }

    pub fn string_equals_ignore_case() -> Self {
        Self {
            quantifier: None,
            operator: GlobalOperator::StringEqualsIgnoreCase,
            if_exists: false,
        }
    }

    pub fn string_not_equals_ignore_case() -> Self {
        Self {
            quantifier: None,
            operator: GlobalOperator::StringNotEqualsIgnoreCase,
            if_exists: false,
        }
    }

    pub fn string_not_like() -> Self {
        Self {
            quantifier: None,
            operator: GlobalOperator::StringNotLike,
            if_exists: false,
        }
    }

    pub fn numeric_equals() -> Self {
        Self {
            quantifier: None,
            operator: GlobalOperator::StringNotLike,
            if_exists: false,
        }
    }

    pub fn numeric_not_equals() -> Self {
        Self {
            quantifier: None,
            operator: GlobalOperator::NumericNotEquals,
            if_exists: false,
        }
    }

    pub fn numeric_less_than() -> Self {
        Self {
            quantifier: None,
            operator: GlobalOperator::NumericLessThan,
            if_exists: false,
        }
    }

    pub fn numeric_less_than_or_equals() -> Self {
        Self {
            quantifier: None,
            operator: GlobalOperator::NumericLessThanEquals,
            if_exists: false,
        }
    }

    pub fn numeric_greater_than() -> Self {
        Self {
            quantifier: None,
            operator: GlobalOperator::NumericGreaterThan,
            if_exists: false,
        }
    }

    pub fn numeric_greater_than_or_equals() -> Self {
        Self {
            quantifier: None,
            operator: GlobalOperator::NumericGreaterThanEquals,
            if_exists: false,
        }
    }

    pub fn date_equals() -> Self {
        Self {
            quantifier: None,
            operator: GlobalOperator::DateEquals,
            if_exists: false,
        }
    }

    pub fn date_not_equals() -> Self {
        Self {
            quantifier: None,
            operator: GlobalOperator::DateNotEquals,
            if_exists: false,
        }
    }

    pub fn date_less_than() -> Self {
        Self {
            quantifier: None,
            operator: GlobalOperator::DateLessThan,
            if_exists: false,
        }
    }

    pub fn date_less_than_or_equals() -> Self {
        Self {
            quantifier: None,
            operator: GlobalOperator::DateLessThanEquals,
            if_exists: false,
        }
    }

    pub fn date_greater_than() -> Self {
        Self {
            quantifier: None,
            operator: GlobalOperator::DateGreaterThan,
            if_exists: false,
        }
    }

    pub fn date_greater_than_or_equals() -> Self {
        Self {
            quantifier: None,
            operator: GlobalOperator::DateGreaterThanEquals,
            if_exists: false,
        }
    }

    pub fn bool_equals() -> Self {
        Self {
            quantifier: None,
            operator: GlobalOperator::Bool,
            if_exists: false,
        }
    }

    pub fn binary_equals() -> Self {
        Self {
            quantifier: None,
            operator: GlobalOperator::BinaryEquals,
            if_exists: false,
        }
    }

    pub fn ip_address() -> Self {
        Self {
            quantifier: None,
            operator: GlobalOperator::IpAddress,
            if_exists: false,
        }
    }

    pub fn not_ip_address() -> Self {
        Self {
            quantifier: None,
            operator: GlobalOperator::NotIpAddress,
            if_exists: false,
        }
    }

    pub fn arn_equals() -> Self {
        Self {
            quantifier: None,
            operator: GlobalOperator::ArnEquals,
            if_exists: false,
        }
    }

    pub fn arn_not_equals() -> Self {
        Self {
            quantifier: None,
            operator: GlobalOperator::ArnNotEquals,
            if_exists: false,
        }
    }

    pub fn arn_like() -> Self {
        Self {
            quantifier: None,
            operator: GlobalOperator::ArnLike,
            if_exists: false,
        }
    }

    pub fn arn_not_like() -> Self {
        Self {
            quantifier: None,
            operator: GlobalOperator::ArnNotLike,
            if_exists: false,
        }
    }

    pub fn null() -> Self {
        Self {
            quantifier: None,
            operator: GlobalOperator::Null,
            if_exists: false,
        }
    }

    pub fn is_for_any(&self) -> bool {
        matches!(self.quantifier, Some(Quantifier::ForAnyValue))
    }

    pub fn set_for_any(&mut self) {
        self.quantifier = Some(Quantifier::ForAnyValue);
    }

    pub fn is_for_all(&self) -> bool {
        matches!(self.quantifier, Some(Quantifier::ForAllValues))
    }

    pub fn set_for_all(&mut self) {
        self.quantifier = Some(Quantifier::ForAllValues);
    }

    pub fn is_if_exists(&self) -> bool {
        self.if_exists
    }

    pub fn set_if_exists(&mut self) {
        self.if_exists = true;
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for Quantifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::ForAllValues => CONDITION_QUANTIFIER_FOR_ALL,
                Self::ForAnyValue => CONDITION_QUANTIFIER_FOR_ANY,
            }
        )
    }
}

impl FromStr for Quantifier {
    type Err = IamFormatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            CONDITION_QUANTIFIER_FOR_ALL => Ok(Self::ForAllValues),
            CONDITION_QUANTIFIER_FOR_ANY => Ok(Self::ForAnyValue),
            _ => unexpected_value_for_type(CONDITION_NAME, s).into(),
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl From<GlobalOperator> for QualifiedName {
    fn from(v: GlobalOperator) -> Self {
        QualifiedName::new_unchecked(v.to_string())
    }
}

impl Display for GlobalOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::StringEquals => CONDITION_OPERATOR_STRING_EQUALS,
                Self::StringNotEquals => CONDITION_OPERATOR_STRING_NOT_EQUALS,
                Self::StringEqualsIgnoreCase => CONDITION_OPERATOR_STRING_EQUALS_IGNORE_CASE,
                Self::StringNotEqualsIgnoreCase => CONDITION_OPERATOR_STRING_NOT_EQUALS_IGNORE_CASE,
                Self::StringLike => CONDITION_OPERATOR_STRING_LIKE,
                Self::StringNotLike => CONDITION_OPERATOR_STRING_NOT_LIKE,
                Self::NumericEquals => CONDITION_OPERATOR_NUMERIC_EQUALS,
                Self::NumericNotEquals => CONDITION_OPERATOR_NUMERIC_NOT_EQUALS,
                Self::NumericLessThan => CONDITION_OPERATOR_NUMERIC_LESS_THAN,
                Self::NumericLessThanEquals => CONDITION_OPERATOR_NUMERIC_LESS_THAN_EQUALS,
                Self::NumericGreaterThan => CONDITION_OPERATOR_NUMERIC_GREATER_THAN,
                Self::NumericGreaterThanEquals => CONDITION_OPERATOR_NUMERIC_GREATER_THAN_EQUALS,
                Self::DateEquals => CONDITION_OPERATOR_DATE_EQUALS,
                Self::DateNotEquals => CONDITION_OPERATOR_DATE_NOT_EQUALS,
                Self::DateLessThan => CONDITION_OPERATOR_DATE_LESS_THAN,
                Self::DateLessThanEquals => CONDITION_OPERATOR_DATE_LESS_THAN_EQUALS,
                Self::DateGreaterThan => CONDITION_OPERATOR_DATE_GREATER_THAN,
                Self::DateGreaterThanEquals => CONDITION_OPERATOR_DATE_GREATER_THAN_EQUALS,
                Self::Bool => CONDITION_OPERATOR_BOOL,
                Self::BinaryEquals => CONDITION_OPERATOR_BINARY_EQUALS,
                Self::IpAddress => CONDITION_OPERATOR_IP_ADDRESS,
                Self::NotIpAddress => CONDITION_OPERATOR_NOT_IP_ADDRESS,
                Self::ArnEquals => CONDITION_OPERATOR_ARN_EQUALS,
                Self::ArnNotEquals => CONDITION_OPERATOR_ARN_NOT_EQUALS,
                Self::ArnLike => CONDITION_OPERATOR_ARN_LIKE,
                Self::ArnNotLike => CONDITION_OPERATOR_ARN_NOT_LIKE,
                Self::Null => CONDITION_OPERATOR_NULL,
            }
        )
    }
}

impl FromStr for GlobalOperator {
    type Err = IamFormatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            CONDITION_OPERATOR_STRING_EQUALS => Ok(Self::StringEquals),
            CONDITION_OPERATOR_STRING_NOT_EQUALS => Ok(Self::StringEquals),
            CONDITION_OPERATOR_STRING_EQUALS_IGNORE_CASE => Ok(Self::StringEqualsIgnoreCase),
            CONDITION_OPERATOR_STRING_NOT_EQUALS_IGNORE_CASE => Ok(Self::StringNotEqualsIgnoreCase),
            CONDITION_OPERATOR_STRING_LIKE => Ok(Self::StringLike),
            CONDITION_OPERATOR_STRING_NOT_LIKE => Ok(Self::StringNotLike),
            CONDITION_OPERATOR_NUMERIC_EQUALS => Ok(Self::NumericEquals),
            CONDITION_OPERATOR_NUMERIC_NOT_EQUALS => Ok(Self::NumericNotEquals),
            CONDITION_OPERATOR_NUMERIC_LESS_THAN => Ok(Self::NumericLessThan),
            CONDITION_OPERATOR_NUMERIC_LESS_THAN_EQUALS => Ok(Self::NumericLessThanEquals),
            CONDITION_OPERATOR_NUMERIC_GREATER_THAN => Ok(Self::NumericGreaterThan),
            CONDITION_OPERATOR_NUMERIC_GREATER_THAN_EQUALS => Ok(Self::NumericGreaterThanEquals),
            CONDITION_OPERATOR_DATE_EQUALS => Ok(Self::DateEquals),
            CONDITION_OPERATOR_DATE_NOT_EQUALS => Ok(Self::DateNotEquals),
            CONDITION_OPERATOR_DATE_LESS_THAN => Ok(Self::DateLessThan),
            CONDITION_OPERATOR_DATE_LESS_THAN_EQUALS => Ok(Self::DateLessThanEquals),
            CONDITION_OPERATOR_DATE_GREATER_THAN => Ok(Self::DateGreaterThan),
            CONDITION_OPERATOR_DATE_GREATER_THAN_EQUALS => Ok(Self::DateGreaterThanEquals),
            CONDITION_OPERATOR_BOOL => Ok(Self::Bool),
            CONDITION_OPERATOR_BINARY_EQUALS => Ok(Self::BinaryEquals),
            CONDITION_OPERATOR_IP_ADDRESS => Ok(Self::IpAddress),
            CONDITION_OPERATOR_NOT_IP_ADDRESS => Ok(Self::NotIpAddress),
            CONDITION_OPERATOR_ARN_EQUALS => Ok(Self::ArnEquals),
            CONDITION_OPERATOR_ARN_NOT_EQUALS => Ok(Self::ArnNotEquals),
            CONDITION_OPERATOR_ARN_LIKE => Ok(Self::ArnLike),
            CONDITION_OPERATOR_ARN_NOT_LIKE => Ok(Self::ArnNotLike),
            CONDITION_OPERATOR_NULL => Ok(Self::Null),
            _ => unexpected_value_for_type(CONDITION_NAME, s).into(),
        }
    }
}

// ------------------------------------------------------------------------------------------------

lazy_static! {
    static ref REGEX_VARIABLE: Regex = Regex::new(r"\$\{([^$}]+)\}").unwrap();
}

impl Display for ConditionValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for ConditionValue {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> From<T> for ConditionValue
where
    T: Into<String>,
{
    fn from(s: T) -> Self {
        Self(s.into())
    }
}

impl ConditionValue {
    /// Return `true` if the identifier contains variables of the form
    /// `${name}`, else `false`.
    pub fn has_variables(&self) -> bool {
        REGEX_VARIABLE.is_match(self.deref())
    }

    /// Replace any variables in the string with values from the context,
    /// returning a new value if the replacements result in a legal identifier
    /// string. The
    pub fn replace_variables<V>(&self, context: &HashMap<String, V>) -> Result<Self, IamFormatError>
    where
        V: Clone + Into<String>,
    {
        let new_text = REGEX_VARIABLE.replace_all(self.deref(), |caps: &Captures<'_>| {
            if let Some(value) = context.get(&caps[1]) {
                value.clone().into()
            } else {
                format!("${{{}}}", &caps[1])
            }
        });
        Ok(Self(new_text.to_string()))
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for GlobalConditionKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}",
            GLOBAL_CONDITION_KEY_NAMESPACE,
            NAMESPACE_SEPARATOR,
            match self {
                Self::CalledVia => GLOBAL_CONDITION_KEY_CALLED_VIA,
                Self::CalledViaFirst => GLOBAL_CONDITION_KEY_CALLED_VIA_FIRST,
                Self::CalledViaLast => GLOBAL_CONDITION_KEY_CALLED_VIA_LAST,
                Self::CurrentTime => GLOBAL_CONDITION_KEY_CURRENT_TIME,
                Self::EpochTime => GLOBAL_CONDITION_KEY_EPOCH_TIME,
                Self::FederatedProvider => GLOBAL_CONDITION_KEY_FEDERATED_PROVIDER,
                Self::MultiFactorAuthAge => GLOBAL_CONDITION_KEY_MULTIFACTOR_AUTH_AGE,
                Self::MultiFactorAuthPresent => GLOBAL_CONDITION_KEY_MULTIFACTOR_AUTH_PRESENT,
                Self::PrincipalAccount => GLOBAL_CONDITION_KEY_PRINCIPAL_ACCOUNT,
                Self::PrincipalArn => GLOBAL_CONDITION_KEY_PRINCIPAL_ARN,
                Self::PrincipalIsAWSService => GLOBAL_CONDITION_KEY_PRINCIPAL_IS_AWS_SERVICE,
                Self::PrincipalOrgID => GLOBAL_CONDITION_KEY_PRINCIPAL_ORG_ID,
                Self::PrincipalOrgPaths => GLOBAL_CONDITION_KEY_PRINCIPAL_ORG_PATHS,
                Self::PrincipalServiceName => GLOBAL_CONDITION_KEY_PRINCIPAL_SERVICE_NAME,
                Self::PrincipalServiceNamesList =>
                    GLOBAL_CONDITION_KEY_PRINCIPAL_SERVICE_NAMES_LIST,
                Self::PrincipalTag => GLOBAL_CONDITION_KEY_PRINCIPAL_TAG,
                Self::PrincipalType => GLOBAL_CONDITION_KEY_PRINCIPAL_TYPE,
                Self::Referer => GLOBAL_CONDITION_KEY_REFERER,
                Self::RequestedRegion => GLOBAL_CONDITION_KEY_REQUESTED_REGION,
                Self::RequestTag => GLOBAL_CONDITION_KEY_REQUEST_TAG,
                Self::ResourceAccount => GLOBAL_CONDITION_KEY_RESOURCE_ACCOUNT,
                Self::ResourceOrgID => GLOBAL_CONDITION_KEY_RESOURCE_ORG_ID,
                Self::ResourceOrgPaths => GLOBAL_CONDITION_KEY_RESOURCE_ORG_PATHS,
                Self::ResourceTag => GLOBAL_CONDITION_KEY_RESOURCE_TAG,
                Self::SecureTransport => GLOBAL_CONDITION_KEY_SECURE_TRANSPORT,
                Self::SourceAccount => GLOBAL_CONDITION_KEY_SOURCE_ACCOUNT,
                Self::SourceArn => GLOBAL_CONDITION_KEY_SOURCE_ARN,
                Self::SourceIdentity => GLOBAL_CONDITION_KEY_SOURCE_IDENTITY,
                Self::SourceIp => GLOBAL_CONDITION_KEY_SOURCE_IP,
                Self::SourceVpc => GLOBAL_CONDITION_KEY_SOURCE_VPC,
                Self::SourceVpce => GLOBAL_CONDITION_KEY_SOURCE_VPCE,
                Self::TagKeys => GLOBAL_CONDITION_KEY_TAG_KEYS,
                Self::TokenIssueTime => GLOBAL_CONDITION_KEY_TOKEN_ISSUE_TIME,
                Self::UserAgent => GLOBAL_CONDITION_KEY_USER_AGENT,
                Self::UserId => GLOBAL_CONDITION_KEY_USERID,
                Self::UserName => GLOBAL_CONDITION_KEY_USERNAME,
                Self::ViaAWSService => GLOBAL_CONDITION_KEY_VIA_AWS_SERVICE,
                Self::VpcSourceIp => GLOBAL_CONDITION_KEY_VPC_SOURCE_IP,
            }
        )
    }
}

impl From<GlobalConditionKey> for QualifiedName {
    fn from(key: GlobalConditionKey) -> Self {
        QualifiedName::new_unchecked(key.to_string())
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
