use crate::model::{Effect, OneOrAll, Policy};
use crate::offline::request::Request;
use crate::offline::statement::evaluate_statement;
use crate::offline::EvaluationError;
use tracing::{error, info, instrument};

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

#[instrument]
pub fn evaluate_policy(
    request: &Request,
    policy: &Policy,
    policy_index: i32,
) -> Result<Effect, EvaluationError> {
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
            let results: Vec<Option<Effect>> = statements
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
            if results.contains(&Some(Effect::Deny)) {
                Some(Effect::Deny)
            } else if results.contains(&Some(Effect::Allow)) {
                Some(Effect::Allow)
            } else {
                None
            }
        }
    };
    let effect = match result {
        None => Effect::Deny,
        Some(effect) => effect,
    };
    info!("Returning policy {} effect {:?}", id, effect);
    Ok(effect)
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
