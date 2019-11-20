use crate::model::{ConditionValue, GlobalConditionOperator, QString};
use crate::offline::trace::Tracer;
use crate::offline::{Context, EvaluationError};
use serde::export::fmt::Error;
use serde::export::Formatter;
use std::fmt::Display;
use std::string::ToString;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn evaluate_all(
    context: &Context,
    operator: &GlobalConditionOperator,
    lhs: &QString,
    rhs: &Vec<ConditionValue>,
    tracer: &mut Tracer,
) -> Result<bool, EvaluationError> {
    Ok(rhs
        .iter()
        .all(|r| match evaluate(context, operator, lhs, r, tracer) {
            Ok(v) => v,
            Err(err) => {
                tracer.message(&format!("|   |   |   '-- evaluation error {:?}", err));
                false
            }
        }))
}

pub fn evaluate_any(
    context: &Context,
    operator: &GlobalConditionOperator,
    lhs: &QString,
    rhs: &Vec<ConditionValue>,
    tracer: &mut Tracer,
) -> Result<bool, EvaluationError> {
    Ok(rhs
        .iter()
        .any(|r| match evaluate(context, operator, lhs, r, tracer) {
            Ok(v) => v,
            Err(err) => {
                tracer.message(&format!("|   |   |   '-- evaluation error {:?}", err));
                false
            }
        }))
}

pub fn evaluate(
    context: &Context,
    operator: &GlobalConditionOperator,
    lhs: &QString,
    rhs: &ConditionValue,
    tracer: &mut Tracer,
) -> Result<bool, EvaluationError> {
    match operator {
        GlobalConditionOperator::StringEquals => {
            call_operator(context, string_equals, lhs, rhs, &ExpectedValueType::String)
        }
        GlobalConditionOperator::StringNotEquals => call_operator(
            context,
            string_not_equals,
            lhs,
            rhs,
            &ExpectedValueType::String,
        ),
        GlobalConditionOperator::StringEqualsIgnoreCase => call_operator(
            context,
            string_equals_ignore_case,
            lhs,
            rhs,
            &ExpectedValueType::String,
        ),
        GlobalConditionOperator::StringNotEqualsIgnoreCase => call_operator(
            context,
            string_not_equals_ignore_case,
            lhs,
            rhs,
            &ExpectedValueType::String,
        ),
        GlobalConditionOperator::StringLike => {
            call_operator(context, string_like, lhs, rhs, &ExpectedValueType::String)
        }
        GlobalConditionOperator::StringNotLike => call_operator(
            context,
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
    context: &Context,
    operator: impl Fn(&ConditionValue, &ConditionValue) -> Result<bool, EvaluationError>,
    lhs: &QString,
    rhs: &ConditionValue,
    value_type: &ExpectedValueType,
) -> Result<bool, EvaluationError> {
    let lhs = lhs_value(context, lhs)?;
    let lhs = match (value_type, lhs) {
        (ExpectedValueType::String, ConditionValue::String(_))
        | (ExpectedValueType::Integer, ConditionValue::Integer(_))
        | (ExpectedValueType::Float, ConditionValue::Float(_))
        | (ExpectedValueType::Bool, ConditionValue::Bool(_)) => lhs,
        (ev, _) => return Err(EvaluationError::ExpectingVariableType(ev.to_string())),
    };
    let rhs = match (value_type, rhs) {
        (ExpectedValueType::String, ConditionValue::String(_)) => expand_rhs_value(context, rhs)?,
        (ExpectedValueType::Integer, ConditionValue::Integer(_))
        | (ExpectedValueType::Float, ConditionValue::Float(_))
        | (ExpectedValueType::Bool, ConditionValue::Bool(_)) => rhs,
        (ev, _) => return Err(EvaluationError::ExpectingVariableType(ev.to_string())),
    };
    operator(lhs, rhs)
}

fn lhs_value<'a>(
    context: &'a Context,
    lhs_name: &QString,
) -> Result<&'a ConditionValue, EvaluationError> {
    Err(EvaluationError::InvalidVariableName(lhs_name.to_string()))
}

fn expand_rhs_value<'a>(
    context: &'a Context,
    rhs: &ConditionValue,
) -> Result<&'a ConditionValue, EvaluationError> {
    Err(EvaluationError::InvalidVariableName("".to_string()))
}

fn string_equals(lhs: &ConditionValue, rhs: &ConditionValue) -> Result<bool, EvaluationError> {
    match (lhs, rhs) {
        (ConditionValue::String(lhs), ConditionValue::String(rhs)) => Ok(lhs == rhs),
        (_, _) => Err(EvaluationError::ExpectingVariableType("String".to_string())),
    }
}

fn string_not_equals(lhs: &ConditionValue, rhs: &ConditionValue) -> Result<bool, EvaluationError> {
    match (lhs, rhs) {
        (ConditionValue::String(lhs), ConditionValue::String(rhs)) => Ok(lhs != rhs),
        (_, _) => Err(EvaluationError::ExpectingVariableType("String".to_string())),
    }
}

fn string_equals_ignore_case(
    lhs: &ConditionValue,
    rhs: &ConditionValue,
) -> Result<bool, EvaluationError> {
    match (lhs, rhs) {
        (ConditionValue::String(lhs), ConditionValue::String(rhs)) => {
            Ok(lhs.to_lowercase() == rhs.to_lowercase())
        }
        (_, _) => Err(EvaluationError::ExpectingVariableType("String".to_string())),
    }
}

fn string_not_equals_ignore_case(
    lhs: &ConditionValue,
    rhs: &ConditionValue,
) -> Result<bool, EvaluationError> {
    match (lhs, rhs) {
        (ConditionValue::String(lhs), ConditionValue::String(rhs)) => {
            Ok(lhs.to_lowercase() != rhs.to_lowercase())
        }
        (_, _) => Err(EvaluationError::ExpectingVariableType("String".to_string())),
    }
}

fn string_like(lhs: &ConditionValue, rhs: &ConditionValue) -> Result<bool, EvaluationError> {
    Ok(false)
}

fn string_not_like(lhs: &ConditionValue, rhs: &ConditionValue) -> Result<bool, EvaluationError> {
    Ok(false)
}
