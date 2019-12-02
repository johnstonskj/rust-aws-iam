/*!
Support for a simplistic offline evaluation for policies, useful for policy testing. Enablec by
the feature `offline_eval`.

This module provides a simple method for testing a policy by creating one or more
[`Request`](request/struct.Request.html) objects and evaluating the policy for the given request.
This implementation is not exhaustive but for those elements it implements it should be a
reasonable approximation. Note that the value returned from [`evaluate`](fn.evaluate.html) also
contains information regarding the reason for any decision, useful for debugging.

# Example

```rust
use aws_iam::{constants, io, model::*, offline::*};
use std::path::PathBuf;use std::str::FromStr;

let policy = io::read_from_file(
        &PathBuf::from("tests/data/good/example-021.json")
    ).expect("Error reading file");

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
let request = Request {
    request_id: Request::request_id(),
    principal: None,
    action: QString::from_str("").expect("bad QString"),
    resource: "".to_string(),
    environment,
};

println!("result: {:?}", evaluate(&request, &policy).expect("An error occurred"));
```

# Request Serialization

The `Request` structure supports Serde serialization and deserialization to supporting testing with
pre-defined example requests. The following is an example serialization of the request composed
in the example above.

```json
{
  "request_id": "test_deny_resource_string_match",
  "action": "dynamodb:read",
  "resource": "arn:aws:dynamodb:us-east-2:123456789012:table/NotBooks",
  "environment": {
    "aws:RequestedRegion": "us-east-1",
    "aws:SecureTransport": true,
    "aws:EpochTime": 1000
  }
}
```
*/

use crate::model::{ConditionOperator, Effect, Policy, QString};
use crate::offline::policy::evaluate_policy;
use std::fmt::{Display, Error, Formatter};
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
#[derive(Clone, Debug, PartialEq)]
pub enum Source {
    /// No explicit allow or deny occured, therefore the default denial was returned.
    Default,
    /// The *is-a* principal test failed.
    Principal,
    /// The *is-not-a* principal test failed.
    NotPrincipal,
    /// The *is-a* action test failed.
    Action,
    /// The *is-not-a* action test failed.
    NotAction,
    /// The *is-a* resource test failed.
    Resource,
    /// The *is-not-a* resource test failed.
    NotResource,
    /// The *match* a condition failed; to help narrow down the actual failure the condition
    /// operator and key are included.
    Condition(ConditionOperator, QString),
}

///
/// The result of an evaluation, this casts directly into a `model::Effect` but in
/// the case of `Deny` will return the source of the failure and any message.
///
#[derive(Debug, PartialEq)]
pub enum EvaluationResult {
    /// Evaluation resulted in an *allow* effect.
    Allow,
    /// Evaluation resulted in an *deny* effect. In this case the source represents a statement
    /// component that caused the denial and the string represents an accompanying message.
    Deny(Source, String),
}

#[derive(Debug, PartialEq)]
pub(crate) enum PartialEvaluationResult {
    Success,
    Failure { source: Source, message: String },
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
    let results: Result<Vec<PartialEvaluationResult>, EvaluationError> = policies
        .iter()
        .enumerate()
        .map(|(idx, policy)| evaluate_policy(request, policy, idx as i32))
        .collect();
    match results {
        Ok(mut results) => Ok(reduce_results(&mut results)),
        Err(err) => Err(err),
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for EvaluationResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Self::Allow => write!(f, "Request allowed"),
            Self::Deny(source, message) => match source {
                Source::Condition(op, key) => write!(
                    f,
                    "Request denied, statement condition operator {:?} for key {:?}, message: {}",
                    op, key, message
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

fn reduce_results(results: &mut Vec<PartialEvaluationResult>) -> EvaluationResult {
    match reduce_optional_results(results) {
        None => EvaluationResult::Deny(Source::Default, "no explicit effect set".to_string()),
        Some(result) => result,
    }
}

pub(crate) fn reduce_optional_results(
    results: &mut Vec<PartialEvaluationResult>,
) -> PartialEvaluationResult {
    let effect_or_none: PartialEvaluationResult =
        results.drain(0..).fold(None, |acc, result| match result {
            Some(EvaluationResult::Allow) => {
                if let Some(EvaluationResult::Deny(_, _)) = acc {
                    acc
                } else {
                    Some(EvaluationResult::Allow)
                }
            }
            Some(EvaluationResult::Deny(s, m)) => {
                Some(EvaluationResult::Deny(s.clone(), m.clone()))
            }
            _ => acc,
        });
    effect_or_none
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod policy;

mod statement;

mod operators;

mod request;
pub use request::{Environment, Principal, Request};

mod variables;

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::constants;
    use crate::io;
    use crate::model::{ConditionValue, QString};
    use crate::offline::{
        evaluate, request::Environment, EvaluationResult, Principal, Request, Source,
    };
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
    fn test_serialize_resource() {
        let request = make_request(
            "test_deny_resource_string_match",
            None,
            "dynamodb:read",
            "arn:aws:dynamodb:us-east-2:123456789012:table/NotBooks",
        );
        println!("{}", serde_json::to_string_pretty(&request).unwrap());
    }

    #[test]
    fn test_deny_resource_string_match() {
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
            "test_deny_resource_string_match",
            None,
            "dynamodb:read",
            "arn:aws:dynamodb:us-east-2:123456789012:table/NotBooks",
        );
        let result = evaluate(&request, &policy);
        assert_eq!(
            result,
            Ok(EvaluationResult::Deny(
                Source::Resource,
                String::from("string_match")
            ))
        );
    }

    #[test]
    fn test_deny_action_qstring_match() {
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
            "test_deny_action_qstring_match",
            None,
            "s3:read",
            "arn:aws:dynamodb:us-east-2:123456789012:table/Books",
        );
        let result = evaluate(&request, &policy);
        assert_eq!(
            result,
            Ok(EvaluationResult::Deny(
                Source::Action,
                String::from("string_match")
            ))
        );
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
        assert_eq!(result, Ok(EvaluationResult::Allow));
    }
}
