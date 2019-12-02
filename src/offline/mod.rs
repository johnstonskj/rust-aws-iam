/*!
* Support for a simplistic offline evaluation for policies, useful for policy testing. Requires
* feature `offline_eval`.
*
* TBD
*
* # Example
*
* ```rust
* use aws_iam::{constants, io, model::*, offline::*};
* use std::path::PathBuf;use std::str::FromStr;
*
* let policy = io::read_from_file(
*         &PathBuf::from("tests/data/good/example-021.json")
*     ).expect("Error reading file");
*
* let environment: Environment = [
*         (
*             QString::from_str(constants::AWS_EPOCH_TIME).unwrap(),
*             ConditionValue::Integer(1000),
*         ),
*         (
*             QString::from_str(constants::AWS_REQUESTED_REGION).unwrap(),
*             ConditionValue::String("us-east-1".to_string()),
*         ),
*         (
*             QString::from_str(constants::AWS_SECURE_TRANSPORT).unwrap(),
*             ConditionValue::Bool(true),
*         ),
*     ]
*     .iter()
*     .cloned()
*     .collect();
* let request = Request {
*     request_id: Request::request_id(),
*     principal: None,
*     action: QString::from_str("").expect("bad QString"),
*     resource: "".to_string(),
*     environment,
* };
*
* println!("result: {:?}", evaluate(&request, &policy).expect("An error occurred"));
* ```
*/

use crate::model::{Effect, Policy};
use crate::offline::policy::evaluate_policy;
use tracing::{error, info, instrument};

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

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// Evaluated a policy against the request context.
///
pub fn evaluate(request: &Request, policy: &Policy) -> Result<Effect, EvaluationError> {
    evaluate_all(request, &[policy])
}

///
/// Evaluated a set of policies against the request context.
///
#[instrument]
pub fn evaluate_all(request: &Request, policies: &[&Policy]) -> Result<Effect, EvaluationError> {
    let results: Vec<Effect> = policies
        .iter()
        .enumerate()
        .filter_map(|(idx, policy)| {
            let result = evaluate_policy(request, policy, idx as i32);
            match result {
                Err(err) => {
                    error!("Returning policy error {:?}", err);
                    None
                }
                Ok(effect) => Some(effect),
            }
        })
        .collect();
    let result = Ok(if results.contains(&Effect::Deny) {
        Effect::Deny
    } else if results.contains(&Effect::Allow) {
        Effect::Allow
    } else {
        Effect::Deny
    });
    info!("Returning request result {:?}", result);
    result
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
    //    use super::*;

    #[test]
    fn test_simple_deny() {}

    #[test]
    fn test_simple_allow() {}
}
