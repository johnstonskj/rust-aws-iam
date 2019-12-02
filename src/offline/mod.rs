/*!
Support for a simplistic offline evaluation for policies, useful for policy testing.
*/

use crate::model::{ConditionOperator, Effect, Policy};
use crate::offline::policy::evaluate_policy;
use std::fmt::Display;
use tracing::instrument;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Errors which may occur during evaluation.
///
#[derive(Clone, Debug, PartialEq)]
pub enum EvaluationError {
    /// The condition operator is unknown to this implementation.
    UnknownOperator(String),
    /// The variable name is not a key in the context environment.
    UnknownVariableName(String),
    /// The variable name is not a valid environment key.
    InvalidVariableName(String),
    /// The value in the environment for the variable does not match the operator type.
    ExpectingVariableType(String),
    /// The variable does not have an associated value.
    MissingVariableValue(String),
    /// A condition expected more, or less, values than provided.
    InvalidValueCardinality,
    /// A collection of errors reported by an underlying function.
    Errors(Vec<EvaluationError>),
}

///
/// The component of a Policy Statement that caused the request to be denied.
///
#[derive(Clone, Debug)]
pub enum Source {
    Default,
    Principal,
    NotPrincipal,
    Action,
    NotAction,
    Resource,
    NotResource,
    Condition(ConditionOperator),
}

///
/// The result of an evaluation, this casts directly into a `model::Effect` but in
/// the case of `Deny` will return the source of the failure and any message.
///
#[derive(Debug)]
pub enum EvaluationResult {
    Allow,
    Deny(Source, String),
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// Evaluated a policy against the request context.
///
pub fn evaluate(request: &Request, policy: &Policy) -> Result<EvaluationResult, EvaluationError> {
    evaluate_all(request, &[policy])
}

///
/// Evaluated a set of policies against the request context.
///
#[instrument]
pub fn evaluate_all(
    request: &Request,
    policies: &[&Policy],
) -> Result<EvaluationResult, EvaluationError> {
    let (mut effects, mut errors): (
        Vec<Result<Option<EvaluationResult>, EvaluationError>>,
        Vec<Result<Option<EvaluationResult>, EvaluationError>>,
    ) = policies
        .iter()
        .enumerate()
        .map(|(idx, policy)| evaluate_policy(request, policy, idx as i32))
        .partition(|r| r.is_ok());
    reduce_results(&mut effects, &mut errors)
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for EvaluationResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Self::Allow => write!(f, "Request allowed"),
            Self::Deny(source, message) => match source {
                Source::Condition(op) => write!(
                    f,
                    "Request denied, statement condition operator {:?}, message: {}",
                    op, message
                ),
                _ => write!(
                    f,
                    "Request denied, statement source {:?}, message: {}",
                    source, message
                ),
            },
        }
    }
}

impl Into<Effect> for EvaluationResult {
    fn into(self) -> Effect {
        match self {
            Self::Allow => Effect::Allow,
            Self::Deny(_, _) => Effect::Deny,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn reduce_results(
    effects: &mut Vec<Result<Option<EvaluationResult>, EvaluationError>>,
    errors: &mut Vec<Result<Option<EvaluationResult>, EvaluationError>>,
) -> Result<EvaluationResult, EvaluationError> {
    match reduce_optional_results(effects, errors) {
        Ok(None) => Ok(EvaluationResult::Deny(
            Source::Default,
            "no explicit effect set".to_string(),
        )),
        Ok(Some(result)) => Ok(result),
        Err(err) => Err(err),
    }
}

pub(crate) fn reduce_optional_results(
    effects: &mut Vec<Result<Option<EvaluationResult>, EvaluationError>>,
    errors: &mut Vec<Result<Option<EvaluationResult>, EvaluationError>>,
) -> Result<Option<EvaluationResult>, EvaluationError> {
    if errors.len() == 1 {
        Err(errors.remove(0).err().unwrap())
    } else if errors.len() > 1 {
        Err(EvaluationError::Errors(
            errors.drain(0..).map(|r| r.err().unwrap()).collect(),
        ))
    } else {
        let effect: Option<EvaluationResult> =
            effects.drain(0..).fold(None, |acc, result| match result {
                Ok(Some(EvaluationResult::Allow)) => {
                    if let Some(EvaluationResult::Deny(_, _)) = acc {
                        acc
                    } else {
                        Some(EvaluationResult::Allow)
                    }
                }
                Ok(Some(EvaluationResult::Deny(s, m))) => {
                    Some(EvaluationResult::Deny(s.clone(), m.clone()))
                }
                _ => acc,
            });
        Ok(effect)
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod policy;

mod statement;

mod operators;

mod request;
pub use request::{Principal, Request};
use serde::export::fmt::Error;
use serde::export::Formatter;

mod variables;

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::constants;
    use crate::io;
    use crate::model::{ConditionValue, Effect, QString};
    use crate::offline::{evaluate, request::Environment, Principal, Request};
    use std::str::FromStr;

    fn make_request(
        test_case: &str,
        principal: Option<Principal>,
        action: &str,
        resource: &str,
    ) -> Request {
        let environment: Environment = [
            (
                QString::from_str(constants::AWS_EPOCH_TIME).unwrap(),
                ConditionValue::Integer(1000),
            ),
            (
                QString::from_str(constants::AWS_REQUESTED_REGION).unwrap(),
                ConditionValue::String("us-east-1".to_string()),
            ),
            (
                QString::from_str(constants::AWS_SECURE_TRANSPORT).unwrap(),
                ConditionValue::Bool(true),
            ),
        ]
        .iter()
        .cloned()
        .collect();
        Request {
            request_id: Some(String::from(test_case)),
            principal,
            action: QString::from_str(action).unwrap(),
            resource: String::from(resource),
            environment,
        }
    }

    #[test]
    fn test_simple_deny() {
        let policy = r#"{
  "Version": "2012-10-17",
  "Statement": {
    "Effect": "Allow",
    "Action": "dynamodb:*",
    "Resource": "arn:aws:dynamodb:us-east-2:123456789012:table/Books"
  }
}"#;
        let policy = io::read_from_string(policy).expect("error parsing policy");
        let request = make_request(
            "test_simple_deny",
            None,
            "dynamodb:read",
            "arn:aws:dynamodb:us-east-2:123456789012:table/NotBooks",
        );
        let result = evaluate(&request, &policy);
        assert_eq!(result, Ok(Effect::Deny));
    }

    #[test]
    fn test_simple_allow() {
        let policy = r#"{
  "Version": "2012-10-17",
  "Statement": {
    "Effect": "Allow",
    "Action": "dynamodb:*",
    "Resource": "arn:aws:dynamodb:us-east-2:123456789012:table/Books"
  }
}"#;
        let policy = io::read_from_string(policy).expect("error parsing policy");
        let request = make_request(
            "test_simple_allow",
            None,
            "dynamodb:read",
            "arn:aws:dynamodb:us-east-2:123456789012:table/Books",
        );
        let result = evaluate(&request, &policy);
        assert_eq!(result, Ok(Effect::Allow));
    }
}
