use crate::model::{ConditionValue, GlobalConditionOperator, QString};
use crate::offline::variables::expand_string;
use crate::offline::EvaluationError;
use std::collections::HashMap;
use std::fmt::{Display, Error, Formatter};
use std::string::ToString;
use tracing::{error, instrument};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub type OperatorResult = Result<bool, EvaluationError>;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

#[instrument]
pub fn evaluate_all(
    environment: &HashMap<QString, ConditionValue>,
    operator: &GlobalConditionOperator,
    lhs: &ConditionValue,
    rhs: &[ConditionValue],
) -> OperatorResult {
    Ok(rhs
        .iter()
        .all(|r| match evaluate(environment, operator, lhs, r) {
            Ok(v) => v,
            Err(err) => {
                error!("Evaluation error {:?}", err);
                false
            }
        }))
}

#[instrument]
pub fn evaluate_any(
    environment: &HashMap<QString, ConditionValue>,
    operator: &GlobalConditionOperator,
    lhs: &ConditionValue,
    rhs: &[ConditionValue],
) -> OperatorResult {
    Ok(rhs
        .iter()
        .any(|r| match evaluate(environment, operator, lhs, r) {
            Ok(v) => v,
            Err(err) => {
                error!("Evaluation error {:?}", err);
                false
            }
        }))
}

#[instrument]
pub fn evaluate(
    environment: &HashMap<QString, ConditionValue>,
    operator: &GlobalConditionOperator,
    lhs: &ConditionValue,
    rhs: &ConditionValue,
) -> OperatorResult {
    match operator {
        GlobalConditionOperator::StringEquals => call_operator(
            environment,
            string_equals,
            lhs,
            rhs,
            &ExpectedValueType::String,
        ),
        GlobalConditionOperator::StringNotEquals => call_operator(
            environment,
            string_not_equals,
            lhs,
            rhs,
            &ExpectedValueType::String,
        ),
        GlobalConditionOperator::StringEqualsIgnoreCase => call_operator(
            environment,
            string_equals_ignore_case,
            lhs,
            rhs,
            &ExpectedValueType::String,
        ),
        GlobalConditionOperator::StringNotEqualsIgnoreCase => call_operator(
            environment,
            string_not_equals_ignore_case,
            lhs,
            rhs,
            &ExpectedValueType::String,
        ),
        GlobalConditionOperator::StringLike => call_operator(
            environment,
            string_like,
            lhs,
            rhs,
            &ExpectedValueType::String,
        ),
        GlobalConditionOperator::StringNotLike => call_operator(
            environment,
            string_not_like,
            lhs,
            rhs,
            &ExpectedValueType::String,
        ),
        GlobalConditionOperator::NumericEquals => {
            Err(EvaluationError::UnknownOperator(String::new()))
        }
        GlobalConditionOperator::NumericNotEquals => {
            Err(EvaluationError::UnknownOperator(String::new()))
        }
        GlobalConditionOperator::NumericLessThan => {
            Err(EvaluationError::UnknownOperator(String::new()))
        }
        GlobalConditionOperator::NumericLessThanEquals => {
            Err(EvaluationError::UnknownOperator(String::new()))
        }
        GlobalConditionOperator::NumericGreaterThan => {
            Err(EvaluationError::UnknownOperator(String::new()))
        }
        GlobalConditionOperator::NumericGreaterThanEquals => {
            Err(EvaluationError::UnknownOperator(String::new()))
        }
        GlobalConditionOperator::DateEquals => Err(EvaluationError::UnknownOperator(String::new())),
        GlobalConditionOperator::DateNotEquals => {
            Err(EvaluationError::UnknownOperator(String::new()))
        }
        GlobalConditionOperator::DateLessThan => {
            Err(EvaluationError::UnknownOperator(String::new()))
        }
        GlobalConditionOperator::DateLessThanEquals => {
            Err(EvaluationError::UnknownOperator(String::new()))
        }
        GlobalConditionOperator::DateGreaterThan => {
            Err(EvaluationError::UnknownOperator(String::new()))
        }
        GlobalConditionOperator::DateGreaterThanEquals => {
            Err(EvaluationError::UnknownOperator(String::new()))
        }
        GlobalConditionOperator::Bool => Err(EvaluationError::UnknownOperator(String::new())),
        GlobalConditionOperator::BinaryEquals => {
            Err(EvaluationError::UnknownOperator(String::new()))
        }
        GlobalConditionOperator::IpAddress => Err(EvaluationError::UnknownOperator(String::new())),
        GlobalConditionOperator::NotIpAddress => {
            Err(EvaluationError::UnknownOperator(String::new()))
        }
        GlobalConditionOperator::ArnEquals => Err(EvaluationError::UnknownOperator(String::new())),
        GlobalConditionOperator::ArnLike => Err(EvaluationError::UnknownOperator(String::new())),
        GlobalConditionOperator::ArnNotEquals => {
            Err(EvaluationError::UnknownOperator(String::new()))
        }
        GlobalConditionOperator::ArnNotLike => Err(EvaluationError::UnknownOperator(String::new())),
        GlobalConditionOperator::Null => Err(EvaluationError::UnknownOperator(String::new())),
        GlobalConditionOperator::Other(id) => Err(EvaluationError::UnknownOperator(id.to_string())),
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
#[allow(dead_code)] // TODO: remove this once all operators are implemented.
enum ExpectedValueType {
    String,
    Integer,
    Float,
    Bool,
}

impl Display for ExpectedValueType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{:?}", self)
    }
}

fn call_operator(
    environment: &HashMap<QString, ConditionValue>,
    operator: impl Fn(&ConditionValue, &ConditionValue) -> OperatorResult,
    lhs: &ConditionValue,
    rhs: &ConditionValue,
    value_type: &ExpectedValueType,
) -> OperatorResult {
    let lhs = match (value_type, lhs) {
        (ExpectedValueType::String, ConditionValue::String(_))
        | (ExpectedValueType::Integer, ConditionValue::Integer(_))
        | (ExpectedValueType::Float, ConditionValue::Float(_))
        | (ExpectedValueType::Bool, ConditionValue::Bool(_)) => lhs,
        (ev, _) => return Err(EvaluationError::ExpectingVariableType(ev.to_string())),
    };
    let rhs = match (value_type, rhs) {
        (ExpectedValueType::String, ConditionValue::String(_)) => {
            expand_rhs_value(environment, rhs.clone())?
        }
        (ExpectedValueType::Integer, ConditionValue::Integer(_))
        | (ExpectedValueType::Float, ConditionValue::Float(_))
        | (ExpectedValueType::Bool, ConditionValue::Bool(_)) => rhs.clone(),
        (ev, _) => return Err(EvaluationError::ExpectingVariableType(ev.to_string())),
    };
    operator(lhs, &rhs)
}

fn expand_rhs_value(
    environment: &HashMap<QString, ConditionValue>,
    rhs: ConditionValue,
) -> Result<ConditionValue, EvaluationError> {
    match rhs {
        ConditionValue::String(input) => {
            let output = expand_string(environment, &input)?;
            Ok(ConditionValue::String(output))
        }
        _ => Ok(rhs),
    }
}

fn string_equals(lhs: &ConditionValue, rhs: &ConditionValue) -> OperatorResult {
    match (lhs, rhs) {
        (ConditionValue::String(lhs), ConditionValue::String(rhs)) => Ok(lhs == rhs),
        (_, _) => Err(EvaluationError::ExpectingVariableType("String".to_string())),
    }
}

fn string_not_equals(lhs: &ConditionValue, rhs: &ConditionValue) -> OperatorResult {
    match (lhs, rhs) {
        (ConditionValue::String(lhs), ConditionValue::String(rhs)) => Ok(lhs != rhs),
        (_, _) => Err(EvaluationError::ExpectingVariableType("String".to_string())),
    }
}

fn string_equals_ignore_case(lhs: &ConditionValue, rhs: &ConditionValue) -> OperatorResult {
    match (lhs, rhs) {
        (ConditionValue::String(lhs), ConditionValue::String(rhs)) => {
            Ok(lhs.to_lowercase() == rhs.to_lowercase())
        }
        (_, _) => Err(EvaluationError::ExpectingVariableType("String".to_string())),
    }
}

fn string_not_equals_ignore_case(lhs: &ConditionValue, rhs: &ConditionValue) -> OperatorResult {
    match (lhs, rhs) {
        (ConditionValue::String(lhs), ConditionValue::String(rhs)) => {
            Ok(lhs.to_lowercase() != rhs.to_lowercase())
        }
        (_, _) => Err(EvaluationError::ExpectingVariableType("String".to_string())),
    }
}

fn string_like(_lhs: &ConditionValue, _rhs: &ConditionValue) -> OperatorResult {
    Ok(false)
}

fn string_not_like(_lhs: &ConditionValue, _rhs: &ConditionValue) -> OperatorResult {
    Ok(false)
}
