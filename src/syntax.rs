/*!
One-line description.
More detailed description, with
# Example
 */

use crate::error::{type_mismatch, IamFormatError};
use aws_arn::ARN;
use serde_json::{Map, Value};
use std::collections::HashMap;
use std::fmt::Display;
use std::iter::FromIterator;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[allow(unused_variables)]
pub trait IamValue {
    fn to_json(&self) -> Result<Value, IamFormatError>;

    fn from_json(value: &Value) -> Result<Self, IamFormatError>
    where
        Self: Sized;
}

#[allow(unused_variables)]
pub trait IamProperty {
    fn into_json_object(&self, object: &mut Map<String, Value>) -> Result<(), IamFormatError>;

    fn from_json_object(value: &Map<String, Value>) -> Result<Self, IamFormatError>
    where
        Self: Sized;

    fn from_json_object_optional(value: &Map<String, Value>) -> Result<Option<Self>, IamFormatError>
    where
        Self: Sized,
    {
        Ok(None)
    }
}

// ------------------------------------------------------------------------------------------------
// Public Values
// ------------------------------------------------------------------------------------------------

pub const JSON_TYPE_NAME_NULL: &str = "Null";
pub const JSON_TYPE_NAME_BOOL: &str = "Bool";
pub const JSON_TYPE_NAME_NUMBER: &str = "Number";
pub const JSON_TYPE_NAME_STRING: &str = "String";
pub const JSON_TYPE_NAME_ARRAY: &str = "Array";
pub const JSON_TYPE_NAME_OBJECT: &str = "Object";

pub const JSON_NUMBER_TYPE_NAME_FLOAT: &str = "Float";
pub const JSON_NUMBER_TYPE_NAME_INTEGER: &str = "Integer";
pub const JSON_NUMBER_TYPE_NAME_UNSIGNED: &str = "Unsigned";

pub const POLICY_WILDCARD_VALUE: &str = "*";

pub const POLICY_NAME: &str = "Policy";

pub const VERSION_NAME: &str = "Version";
pub const VERSION_VALUE_2012: &str = "2012-10-17";
pub const VERSION_VALUE_2008: &str = "2008-10-17";

pub const ID_NAME: &str = "Id";

pub const STATEMENT_NAME: &str = "Statement";

pub const SID_NAME: &str = "Sid";

pub const PRINCIPAL_NAME: &str = "Principal";
pub const PRINCIPAL_VALUE_PRINCIPAL: &str = "Principal";
pub const PRINCIPAL_VALUE_NOT_PRINCIPAL: &str = "NotPrincipal";

pub const PRINCIPAL_TYPE_AWS: &str = "AWS";
pub const PRINCIPAL_TYPE_FEDERATED: &str = "Federated";
pub const PRINCIPAL_TYPE_SERVICE: &str = "Service";
pub const PRINCIPAL_TYPE_CANONICAL_USER: &str = "CanonicalUser";

pub const EFFECT_NAME: &str = "Effect";
pub const EFFECT_VALUE_ALLOW: &str = "Allow";
pub const EFFECT_VALUE_DENY: &str = "Deny";

pub const ACTION_NAME: &str = "Action";
pub const ACTION_VALUE_ACTION: &str = "Action";
pub const ACTION_VALUE_NOT_ACTION: &str = "NotAction";

pub const RESOURCE_NAME: &str = "Resource";
pub const RESOURCE_VALUE_RESOURCE: &str = "Resource";
pub const RESOURCE_VALUE_NOT_RESOURCE: &str = "NotResource";

pub const CONDITION_NAME: &str = "Condition";

pub const CONDITION_VALUE_NAME: &str = "Value";

pub const CONDITION_QUANTIFIER_FOR_ANY: &str = "ForAnyValue";
pub const CONDITION_QUANTIFIER_FOR_ALL: &str = "ForAllValues";

pub const CONDITION_QUANTIFIER_IF_EXISTS: &str = "IfExists";

pub const CONDITION_OPERATOR_STRING_EQUALS: &str = "StringEquals";
pub const CONDITION_OPERATOR_STRING_NOT_EQUALS: &str = "StringNotEquals";
pub const CONDITION_OPERATOR_STRING_EQUALS_IGNORE_CASE: &str = "StringEqualsIgnoreCase";
pub const CONDITION_OPERATOR_STRING_NOT_EQUALS_IGNORE_CASE: &str = "StringNotEqualsIgnoreCase";
pub const CONDITION_OPERATOR_STRING_LIKE: &str = "StringLike";
pub const CONDITION_OPERATOR_STRING_NOT_LIKE: &str = "StringNotLike";
pub const CONDITION_OPERATOR_NUMERIC_EQUALS: &str = "NumericEquals";
pub const CONDITION_OPERATOR_NUMERIC_NOT_EQUALS: &str = "NumericNotEquals";
pub const CONDITION_OPERATOR_NUMERIC_LESS_THAN: &str = "NumericLessThan";
pub const CONDITION_OPERATOR_NUMERIC_LESS_THAN_EQUALS: &str = "NumericLessThanEquals";
pub const CONDITION_OPERATOR_NUMERIC_GREATER_THAN: &str = "NumericGreaterThan";
pub const CONDITION_OPERATOR_NUMERIC_GREATER_THAN_EQUALS: &str = "NumericGreaterThanEquals";
pub const CONDITION_OPERATOR_DATE_EQUALS: &str = "DateEquals";
pub const CONDITION_OPERATOR_DATE_NOT_EQUALS: &str = "DateNotEquals";
pub const CONDITION_OPERATOR_DATE_LESS_THAN: &str = "DateLessThan";
pub const CONDITION_OPERATOR_DATE_LESS_THAN_EQUALS: &str = "DateLessThanEquals";
pub const CONDITION_OPERATOR_DATE_GREATER_THAN: &str = "DateGreaterThan";
pub const CONDITION_OPERATOR_DATE_GREATER_THAN_EQUALS: &str = "DateGreaterThanEquals";
pub const CONDITION_OPERATOR_BOOL: &str = "Bool";
pub const CONDITION_OPERATOR_BINARY_EQUALS: &str = "BinaryEquals";
pub const CONDITION_OPERATOR_IP_ADDRESS: &str = "IpAddress";
pub const CONDITION_OPERATOR_NOT_IP_ADDRESS: &str = "NotIpAddress";
pub const CONDITION_OPERATOR_ARN_EQUALS: &str = "ArnEquals";
pub const CONDITION_OPERATOR_ARN_NOT_EQUALS: &str = "ArnNotEquals";
pub const CONDITION_OPERATOR_ARN_LIKE: &str = "ArnLike";
pub const CONDITION_OPERATOR_ARN_NOT_LIKE: &str = "ArnNotLike";
pub const CONDITION_OPERATOR_NULL: &str = "Null";

pub const SERVICE_NAME_NAME: &str = "ServiceName";

pub const QUALIFIED_NAME_NAME: &str = "QualifiedName";

pub const USER_ID_NAME: &str = "CanonicalUserId";

pub const VALUE_NAME: &str = "Value";

pub const ARN_NAME: &str = "ARN";

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl IamValue for ARN {
    fn to_json(&self) -> Result<Value, IamFormatError> {
        Ok(Value::String(self.to_string()))
    }

    fn from_json(value: &Value) -> Result<Self, IamFormatError>
    where
        Self: Sized,
    {
        if let Value::String(value) = value {
            Ok(Self::from_str(value)?)
        } else {
            type_mismatch(ARN_NAME, JSON_TYPE_NAME_STRING, json_type_name(value)).into()
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

#[inline]
pub(crate) fn json_type_name(v: &Value) -> String {
    match v {
        Value::Null => JSON_TYPE_NAME_NULL,
        Value::Bool(_) => JSON_TYPE_NAME_BOOL,
        Value::Number(_) => JSON_TYPE_NAME_NUMBER,
        Value::String(_) => JSON_TYPE_NAME_STRING,
        Value::Array(_) => JSON_TYPE_NAME_ARRAY,
        Value::Object(_) => JSON_TYPE_NAME_OBJECT,
    }
    .to_string()
}

#[inline]
pub(crate) fn vec_map_to_json<K, V>(map: &HashMap<K, Vec<V>>) -> Result<Value, IamFormatError>
where
    K: Display,
    V: IamValue,
{
    let result: Result<Vec<(String, Value)>, IamFormatError> = map
        .iter()
        .map(|(k, v)| match vec_to_json(v) {
            Ok(v) => Ok((k.to_string(), v)),
            Err(e) => Err(e),
        })
        .collect();
    let object = Map::from_iter(result?.into_iter());
    Ok(Value::Object(object))
}

// #[inline]
// pub(crate) fn map_to_json<K, V>(map: &HashMap<K, V>) -> Result<Value, IamFormatError>
// where
//     K: Display,
//     V: IamValue,
// {
//     let result: Result<Vec<(String, Value)>, IamFormatError> = map
//         .iter()
//         .map(|(k, v)| match v.to_json() {
//             Some(v) => Ok((k.to_string(), v)),
//             None => Err(IamFormatError::CouldNotSerialize),
//         })
//         .collect();
//     let object = Map::from_iter(result?.into_iter());
//     Ok(Value::Object(object))
// }

#[inline]
pub(crate) fn vec_to_json<T>(vec: &Vec<T>) -> Result<Value, IamFormatError>
where
    T: IamValue,
{
    let value = match vec.len() {
        0 => Value::Null,
        1 => vec.get(0).unwrap().to_json()?,
        _ => {
            let result: Result<Vec<Value>, IamFormatError> =
                vec.iter().map(|v| v.to_json()).collect();
            Value::Array(result?)
        }
    };
    Ok(value)
}

#[inline]
pub(crate) fn display_to_json<T>(value: T) -> Value
where
    T: Display,
{
    Value::String(value.to_string())
}

#[inline]
pub(crate) fn display_vec_to_json<T>(vec: &Vec<T>) -> Result<Value, IamFormatError>
where
    T: Display,
{
    let value = match vec.len() {
        0 => Value::Null,
        1 => display_to_json(vec.get(0).unwrap()),
        _ => Value::Array(vec.iter().map(display_to_json).collect()),
    };
    Ok(value)
}

#[inline]
pub(crate) fn vec_from_json<V>(value: &Value, name: &str) -> Result<Vec<V>, IamFormatError>
where
    V: IamValue,
{
    if let Value::Array(arr) = value {
        arr.iter().map(V::from_json).collect()
    } else {
        type_mismatch(name, JSON_TYPE_NAME_ARRAY, json_type_name(value)).into()
    }
}

#[inline]
pub(crate) fn vec_from_str_json<V, E>(value: &Value, name: &str) -> Result<Vec<V>, IamFormatError>
where
    V: FromStr<Err = E>,
    E: Into<IamFormatError>,
{
    if let Value::String(s) = value {
        Ok(vec![V::from_str(s).map_err(E::into)?])
    } else if let Value::Array(arr) = value {
        arr.iter()
            .map(|v| {
                if let Value::String(s) = v {
                    Ok(V::from_str(s).map_err(E::into)?)
                } else {
                    Err(type_mismatch(
                        name,
                        JSON_TYPE_NAME_STRING,
                        json_type_name(value),
                    ))
                }
            })
            .collect()
    } else {
        type_mismatch(name, JSON_TYPE_NAME_ARRAY, json_type_name(value)).into()
    }
}

#[inline]
pub(crate) fn from_json_str<T, E>(value: &Value, name: &str) -> Result<T, IamFormatError>
where
    T: FromStr<Err = E>,
    E: Into<IamFormatError>,
{
    if let Value::String(s) = value {
        Ok(T::from_str(s).map_err(E::into)?)
    } else {
        type_mismatch(name, JSON_TYPE_NAME_STRING, json_type_name(value)).into()
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
