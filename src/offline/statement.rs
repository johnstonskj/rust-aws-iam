use crate::model::{
    Action, ConditionOperator, ConditionOperatorQuantifier, ConditionValue, OneOrAll, OneOrAny,
    Principal, QString, Resource, Statement,
};
use crate::offline::request::{Environment, Principal as RequestPrincipal, Request};
use crate::offline::{
    operators, reduce_optional_results, EvaluationResult, PartialEvaluationResult,
};
use crate::offline::{EvaluationError, Source};
use std::collections::HashMap;
use tracing::{debug, info, instrument};

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

#[instrument]
pub fn evaluate_statement(
    request: &Request,
    statement: &Statement,
    statement_index: i32,
) -> Result<PartialEvaluationResult, EvaluationError> {
    let mut effect: Option<EvaluationResult> = None;

    // >>>>> eval principal
    let result = eval_statement_principal(&request.principal, &statement.principal);
    if let Some(EvaluationResult::Deny(_, _)) = result {
        return Ok(result);
    } else if let Some(EvaluationResult::Allow) = result {
        effect = result;
    }

    // >>>>> eval action
    let result = eval_statement_action(&request.action, &statement.action);
    if let Some(EvaluationResult::Deny(_, _)) = result {
        return Ok(result);
    } else if let Some(EvaluationResult::Allow) = result {
        effect = result;
    }

    // >>>>> eval resource
    let result = eval_statement_resource(&request.resource, &statement.resource);
    if let Some(EvaluationResult::Deny(_, _)) = result {
        return Ok(result);
    } else if let Some(EvaluationResult::Allow) = result {
        effect = result;
    }

    // >>>>> eval conditions
    match eval_statement_conditions(&request.environment, &statement.condition) {
        Ok(None) => Ok(effect),
        result @ _ => result,
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn statement_id(statement: &Statement, statement_index: i32) -> String {
    match &statement.sid {
        Some(id) => id.to_string(),
        None => format!("[{}]", statement_index),
    }
}

#[instrument]
fn eval_statement_principal(
    request_principal: &Option<RequestPrincipal>,
    statement_principal: &Option<Principal>,
) -> PartialEvaluationResult {
    let effect = if let Some(principal) = request_principal {
        match statement_principal {
            None => None,
            Some(Principal::Principal(ps)) => {
                if let Some(p) = ps.get(&principal.principal_type) {
                    match p {
                        OneOrAny::Any => Some(EvaluationResult::Allow),
                        OneOrAny::One(v) => {
                            if string_match(&principal.identifier, v) {
                                Some(EvaluationResult::Allow)
                            } else {
                                Some(EvaluationResult::Deny(
                                    Source::Principal,
                                    "string_match".to_string(),
                                ))
                            }
                        }
                        OneOrAny::AnyOf(vs) => {
                            if contains_match(&principal.identifier, vs) {
                                Some(EvaluationResult::Allow)
                            } else {
                                Some(EvaluationResult::Deny(
                                    Source::Principal,
                                    "contains_match".to_string(),
                                ))
                            }
                        }
                    }
                } else {
                    None
                }
            }
            Some(Principal::NotPrincipal(ps)) => {
                if let Some(p) = ps.get(&principal.principal_type) {
                    match p {
                        OneOrAny::Any => Some(EvaluationResult::Deny(
                            Source::NotPrincipal,
                            "any".to_string(),
                        )),
                        OneOrAny::One(v) => {
                            if string_match(&principal.identifier, v) {
                                Some(EvaluationResult::Deny(
                                    Source::NotPrincipal,
                                    "string_match".to_string(),
                                ))
                            } else {
                                Some(EvaluationResult::Allow)
                            }
                        }
                        OneOrAny::AnyOf(vs) => {
                            if contains_match(&principal.identifier, vs) {
                                Some(EvaluationResult::Deny(
                                    Source::NotPrincipal,
                                    "contains_match".to_string(),
                                ))
                            } else {
                                Some(EvaluationResult::Allow)
                            }
                        }
                    }
                } else {
                    None
                }
            }
        }
    } else {
        None
    };
    info!(
        "Matching principal {:?} returned {:?}",
        request_principal, effect
    );
    effect
}

#[instrument]
fn eval_statement_action(
    request_action: &QString,
    statement_action: &Action,
) -> PartialEvaluationResult {
    let effect = match statement_action {
        Action::Action(a) => match a {
            OneOrAny::Any => Some(EvaluationResult::Allow),
            OneOrAny::One(v) => {
                if string_match(&request_action.to_string(), &v.to_string()) {
                    Some(EvaluationResult::Allow)
                } else {
                    debug!(
                        target = "eval",
                        "action: {} ≈ {} → false", request_action, v
                    );
                    Some(EvaluationResult::Deny(
                        Source::Action,
                        "string_match".to_string(),
                    ))
                }
            }
            OneOrAny::AnyOf(vs) => {
                if contains_qmatch(&request_action.to_string(), vs) {
                    Some(EvaluationResult::Allow)
                } else {
                    debug!(
                        target = "eval",
                        "action: {:?} ≈ {} → false", vs, request_action
                    );
                    Some(EvaluationResult::Deny(
                        Source::Action,
                        "contains_match".to_string(),
                    ))
                }
            }
        },
        Action::NotAction(a) => match a {
            OneOrAny::Any => Some(EvaluationResult::Deny(Source::NotAction, "any".to_string())),
            OneOrAny::One(v) => {
                if string_match(&request_action.to_string(), &v.to_string()) {
                    debug!(
                        target = "eval",
                        "action: {} ≉ {} → false", request_action, v
                    );
                    Some(EvaluationResult::Deny(
                        Source::NotAction,
                        "string_match".to_string(),
                    ))
                } else {
                    Some(EvaluationResult::Allow)
                }
            }
            OneOrAny::AnyOf(vs) => {
                if contains_qmatch(&request_action.to_string(), vs) {
                    debug!(
                        target = "eval",
                        "action: {:?} ≉ {} → false", vs, request_action
                    );
                    Some(EvaluationResult::Deny(
                        Source::NotAction,
                        "contains_match".to_string(),
                    ))
                } else {
                    Some(EvaluationResult::Allow)
                }
            }
        },
    };
    info!("Matching action {:?} returned {:?}", request_action, effect);
    effect
}

#[instrument]
fn eval_statement_resource(
    request_resource: &String,
    statement_resource: &Resource,
) -> PartialEvaluationResult {
    let effect = match statement_resource {
        Resource::Resource(a) => match a {
            OneOrAny::Any => Some(EvaluationResult::Allow),
            OneOrAny::One(v) => {
                if resource_match(request_resource, v) {
                    Some(EvaluationResult::Allow)
                } else {
                    println!(
                        //target = "eval",
                        "resource: {} ≈ {} → false", request_resource, v
                    );
                    Some(EvaluationResult::Deny(
                        Source::Resource,
                        "string_match".to_string(),
                    ))
                }
            }
            OneOrAny::AnyOf(vs) => {
                if contains_resource(request_resource, vs) {
                    Some(EvaluationResult::Allow)
                } else {
                    println!(
                        //target = "eval",
                        "resource: {:?} ≈ {} → false", vs, request_resource
                    );
                    Some(EvaluationResult::Deny(
                        Source::Action,
                        "contains_match".to_string(),
                    ))
                }
            }
        },
        Resource::NotResource(a) => match a {
            OneOrAny::Any => Some(EvaluationResult::Deny(
                Source::NotResource,
                "any".to_string(),
            )),
            OneOrAny::One(v) => {
                if resource_match(request_resource, v) {
                    println!(
                        //target = "eval",
                        "resource: {} ≉ {} → false", request_resource, v
                    );
                    Some(EvaluationResult::Deny(
                        Source::NotResource,
                        "string_match".to_string(),
                    ))
                } else {
                    Some(EvaluationResult::Allow)
                }
            }
            OneOrAny::AnyOf(vs) => {
                if contains_resource(request_resource, vs) {
                    println!(
                        //target = "eval",
                        "resource: {:?} ≉ {} → false", vs, request_resource
                    );
                    Some(EvaluationResult::Deny(
                        Source::NotAction,
                        "contains_match".to_string(),
                    ))
                } else {
                    Some(EvaluationResult::Allow)
                }
            }
        },
    };
    info!(
        "Matching resource {:?} returned {:?}",
        request_resource, effect
    );
    effect
}

//#[instrument]
fn eval_statement_conditions(
    request_environment: &Environment,
    statement_conditions: &Option<
        HashMap<ConditionOperator, HashMap<QString, OneOrAll<ConditionValue>>>,
    >,
) -> Result<PartialEvaluationResult, EvaluationError> {
    let result = if let Some(conditions) = statement_conditions {
        let mut results = conditions
            .iter()
            .map(|(op, vs)| eval_statement_condition_op(request_environment, op, vs))
            .flatten()
            .collect();
        match results {
            Ok(mut results) => Ok(reduce_optional_results(&mut results)),
            Err(err) => Err(err),
        }
    } else {
        Ok(None)
    };
    info!("Matching statement conditions returned {:?}", result);
    result
}

fn eval_statement_condition_op(
    request_environment: &Environment,
    condition_operator: &ConditionOperator,
    condition_values: &HashMap<QString, OneOrAll<ConditionValue>>,
) -> Vec<Result<PartialEvaluationResult, EvaluationError>> {
    info!("Statement condition, operator {:?}", condition_operator);
    let results: Vec<Result<Option<EvaluationResult>, EvaluationError>> = condition_values
        .iter()
        .map(|(key, values)| {
            eval_statement_condition_key(request_environment, condition_operator, key, values)
        })
        .collect();
    info!("Matching statement conditions returned {:?}", results);
    results
}

fn eval_statement_condition_key(
    request_environment: &Environment,
    condition_operator: &ConditionOperator,
    condition_key: &QString,
    condition_values: &OneOrAll<ConditionValue>,
) -> Result<PartialEvaluationResult, EvaluationError> {
    match request_environment.get(condition_key) {
        None => {
            if condition_operator.if_exists {
                Ok(Some(EvaluationResult::Allow))
            } else {
                Ok(None)
            }
        }
        Some(lhs) => match (&condition_operator.quantifier, condition_values) {
            (None, OneOrAll::One(rhs)) => {
                operators::evaluate(request_environment, &condition_operator.operator, lhs, rhs)
                    .map(|r| bool_effect(r, condition_operator, condition_key, "one"))
            }
            (Some(ConditionOperatorQuantifier::ForAllValues), OneOrAll::All(rhs)) => {
                operators::evaluate_all(request_environment, &condition_operator.operator, lhs, rhs)
                    .map(|r| bool_effect(r, condition_operator, condition_key, "for_all"))
            }
            (Some(ConditionOperatorQuantifier::ForAnyValue), OneOrAll::All(rhs)) => {
                operators::evaluate_any(request_environment, &condition_operator.operator, lhs, rhs)
                    .map(|r| bool_effect(r, condition_operator, condition_key, "for_any"))
            }
            _ => Err(EvaluationError::InvalidValueCardinality),
        },
    }
}

#[inline]
fn string_match(lhs: &str, rhs: &str) -> bool {
    if rhs.ends_with('*') {
        lhs.starts_with(&rhs[0..rhs.len() - 1])
    } else {
        lhs == rhs
    }
}

#[inline]
fn contains_match(lhs: &str, rhs: &Vec<String>) -> bool {
    rhs.iter().any(|r| string_match(lhs, r))
}

#[inline]
fn contains_qmatch(lhs: &str, rhs: &Vec<QString>) -> bool {
    rhs.iter().any(|r| string_match(lhs, &r.to_string()))
}

#[inline]
fn resource_match(lhs: &String, rhs: &String) -> bool {
    let lhs = resource_split(lhs);
    let rhs = resource_split(rhs);
    lhs.iter()
        .enumerate()
        .map(|(i, lhs)| string_match(lhs, rhs.get(i).unwrap()))
        .all(|v| v)
}

fn resource_split(lhs: &String) -> Vec<String> {
    let splits: Vec<String> = lhs.split(':').map(|s| s.to_string()).collect();
    if splits.len() < 6 {
        Vec::new()
    } else if splits.len() == 6 {
        if splits.get(0).unwrap() == "arn" {
            splits[1..].to_vec()
        } else {
            Vec::new()
        }
    } else {
        if splits.get(0).unwrap() == "arn" {
            let mut splits = splits[1..5].to_vec();
            splits.push(splits[6..].join(":"));
            splits
        } else {
            Vec::new()
        }
    }
}

#[inline]
fn contains_resource(lhs: &String, rhs: &Vec<String>) -> bool {
    rhs.iter().any(|r| resource_match(lhs, r))
}

fn bool_effect(
    result: bool,
    condition_operator: &ConditionOperator,
    condition_key: &QString,
    message: &str,
) -> Option<EvaluationResult> {
    if result {
        Some(EvaluationResult::Allow)
    } else {
        Some(EvaluationResult::Deny(
            Source::Condition(condition_operator.clone(), condition_key.clone()),
            String::from(message),
        ))
    }
}
