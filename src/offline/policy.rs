use crate::model::{OneOrAll, Policy};
use crate::offline::request::Request;
use crate::offline::statement::evaluate_statement;
use crate::offline::{EvaluationError, EvaluationResult};
use tracing::{error, info, instrument};

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

#[instrument]
pub fn evaluate_policy(
    request: &Request,
    policy: &Policy,
    policy_index: i32,
) -> Result<Option<EvaluationResult>, EvaluationError> {
    let id = policy_id(policy, policy_index);
    let result = match &policy.statement {
        OneOrAll::One(statement) => match evaluate_statement(request, statement, 0) {
            Err(err) => {
                error!("Returning statement error {:?}", err);
                return Err(err);
            }
            Ok(effect) => effect,
        },
        OneOrAll::All(statements) => {
            let mut results: Vec<Option<EvaluationResult>> = statements
                .iter()
                .enumerate()
                .filter_map(|(idx, statement)| {
                    let result = evaluate_statement(request, statement, idx as i32);
                    match result {
                        Err(err) => {
                            // TODO: deal with errors more effectively
                            error!("Returning statement error {:?}", err);
                            None
                        }
                        Ok(effect) => Some(effect),
                    }
                })
                .collect();
            results.drain(0..).fold::<Option<EvaluationResult>>(
                Some(EvaluationResult::None),
                |acc, result| match result {
                    Ok(Some(EvaluationResult::Allow)) => {
                        if let EvaluationResult::Deny(_, _) = acc {
                            acc
                        } else {
                            Some(EvaluationResult::Allow)
                        }
                    }
                    Ok(Some(EvaluationResult::Deny(s, m))) => {
                        Some(EvaluationResult::Deny(s.clone(), m.clone()))
                    }
                    _ => acc,
                },
            )
        }
    };
    info!("Returning policy {} effect {:?}", id, result);
    Ok(result)
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
