/*!
Support for a simplistic offline evaluation for policies, useful for simple testing.
*/

use crate::model::{ConditionValue, Effect, OneOrAll, Policy, PrincipalType, QString, Statement};
use std::collections::HashMap;
use std::str::FromStr;
use uuid::Uuid;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct Principal {
    pub principal_type: PrincipalType,
    pub identifier: String,
}

#[derive(Debug)]
pub struct Context {
    pub principal: Option<Principal>,
    pub action: QString,
    pub resource: String,
    pub map: HashMap<QString, ConditionValue>,
}

#[derive(Debug)]
pub enum EvaluationError {
    UnknownOperator(String),
    UnknownVariableName(String),
    InvalidVariableName(String),
    ExpectingVariableType(String),
    MissingVariableValue(String),
}

#[derive(Debug)]
pub struct EvaluationResult {
    pub effect: Effect,
    pub tracer: Tracer,
}
// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn evaluate(
    request_id: Option<String>,
    context: &Context,
    policy: &Policy,
) -> Result<EvaluationResult, EvaluationError> {
    let mut tracer = Tracer::default();
    let request_id = request_id_or_default(request_id);
    start_request(&request_id, &mut tracer);
    let result = evaluate_policy(&request_id, context, policy, 0, &mut tracer);
    request_result(&result, &mut tracer);
    result.map(|e| EvaluationResult { effect: e, tracer })
}

pub fn evaluate_all(
    request_id: Option<String>,
    context: &Context,
    policies: &Vec<Policy>,
) -> Result<EvaluationResult, EvaluationError> {
    let mut tracer = Tracer::default();
    let request_id = request_id_or_default(request_id);
    start_request(&request_id, &mut tracer);
    let results: Vec<Effect> = policies
        .iter()
        .enumerate()
        .filter_map(|(idx, policy)| {
            let result = evaluate_policy(&request_id, context, policy, idx as i32, &mut tracer);
            match result {
                Err(err) => {
                    tracer.message(&format!("Returning policy error {:?}", err));
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
    request_result(&result, &mut tracer);
    result.map(|e| EvaluationResult { effect: e, tracer })
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Context {
    pub fn get(&self, key: &QString) -> Result<&ConditionValue, EvaluationError> {
        match self.map.get(key) {
            Some(v) => Ok(v),
            None => Err(EvaluationError::UnknownVariableName(key.to_string())),
        }
    }

    pub fn get_(&self, key: &str) -> Result<&ConditionValue, EvaluationError> {
        let key = QString::from_str(key)
            .map_err(|e| EvaluationError::InvalidVariableName(key.to_string()))?;
        match self.map.get(&key) {
            Some(v) => Ok(v),
            None => Err(EvaluationError::UnknownVariableName(key.to_string())),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn start_request(request_id: &String, tracer: &mut Tracer) {
    tracer.message(&format!("Evaluating request {}", request_id));
}

fn request_result(result: &Result<Effect, EvaluationError>, tracer: &mut Tracer) {
    tracer.message(&format!("Returning request result {:?}", result));
}

fn evaluate_policy(
    request_id: &String,
    context: &Context,
    policy: &Policy,
    policy_index: i32,
    tracer: &mut Tracer,
) -> Result<Effect, EvaluationError> {
    let mut effect = Effect::Deny;
    tracer.message(&format!(
        "Evaluating policy {}",
        policy_id(policy, policy_index)
    ));
    let result = match &policy.statement {
        OneOrAll::One(statement) => match evaluate_statement(context, statement, 0, tracer) {
            Err(err) => {
                tracer.message(&format!("Returning statement error {:?}", err));
                None
            }
            Ok(effect) => effect,
        },
        OneOrAll::All(statements) => {
            let results: Vec<Option<Effect>> = statements
                .iter()
                .enumerate()
                .filter_map(|(idx, statement)| {
                    let result = evaluate_statement(context, statement, idx as i32, tracer);
                    match result {
                        Err(err) => {
                            tracer.message(&format!("Returning statement error {:?}", err));
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
    tracer.message(&format!("Returning policy effect {:?}", effect));
    Ok(effect)
00]\}

fn evaluate_statement(
    context: &Context,
    statement: &Statement,
    statement_index: i32,
    tracer: &mut Tracer,
) -> Result<Option<Effect>, EvaluationError> {
    let effect: Option<Effect> = None;
    tracer.open();
    tracer.message(&format!(
        "Evaluating statement {}",
        statement_id(statement, statement_index)
    ));
    tracer.message(&format!(
        "Returning statement effect {}",
        match &effect {
            None => "Undecided".to_string(),
            Some(effect) => format!("{:?}", effect),
        }
    ));
    tracer.close();
    Ok(effect)
}

fn request_id_or_default(request_id: Option<String>) -> String {
    match request_id {
        Some(id) => id,
        None => Uuid::new_v4()
            .to_hyphenated()
            .encode_lower(&mut Uuid::encode_buffer())
            .to_string(),
    }
}

fn policy_id(policy: &Policy, policy_index: i32) -> String {
    match &policy.id {
        Some(id) => id.to_string(),
        None => format!("[{}]", policy_index),
    }
}

fn statement_id(statement: &Statement, statement_index: i32) -> String {
    match &statement.sid {
        Some(id) => id.to_string(),
        None => format!("[{}]", statement_index),
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod operators;

mod trace;
pub use trace::Tracer;

mod variables;

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    //    use super::*;

    #[test]
    fn test_something() {}
}
