/*!
Support for a simplistic offline evaluation for policies, useful for policy testing.
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
    let request_id = &request.request_id_or_default();
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
pub use request::{Principal, Request};

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
