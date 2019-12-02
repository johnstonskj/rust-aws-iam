use crate::model::{OneOrAll, Policy};
use crate::offline::request::Request;
use crate::offline::statement::evaluate_statement;
use crate::offline::{reduce_optional_results, EvaluationError, PartialEvaluationResult};
use tracing::{info, instrument};

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

#[instrument]
pub fn evaluate_policy(
    request: &Request,
    policy: &Policy,
    policy_index: i32,
) -> Result<PartialEvaluationResult, EvaluationError> {
    let id = policy_id(policy, policy_index);
    let result = match &policy.statement {
        OneOrAll::One(statement) => evaluate_statement(request, statement, 0),
        OneOrAll::All(statements) => {
            let results: Result<Vec<PartialEvaluationResult>, EvaluationError> = statements
                .iter()
                .enumerate()
                .map(|(idx, statement)| evaluate_statement(request, statement, idx as i32))
                .collect();
            match results {
                Ok(mut results) => Ok(reduce_optional_results(&mut results)),
                Err(err) => Err(err),
            }
        }
    };
    info!("Returning policy {} effect {:?}", id, result);
    result
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn policy_id(policy: &Policy, policy_index: i32) -> String {
    match &policy.id {
        Some(id) => id.to_string(),
        None => format!("[{}]", policy_index),
    }
}
