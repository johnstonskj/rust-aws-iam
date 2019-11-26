use crate::model::{
    Action, ConditionOperator, ConditionOperatorQuantifier, ConditionValue, Effect, OneOrAll,
    OneOrAny, Principal, QString, Resource, Statement,
};
use crate::offline::operators;
use crate::offline::request::{Environment, Principal as RequestPrincipal, Request};
use crate::offline::EvaluationError;
use std::collections::HashMap;
use tracing::{info, instrument};

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

#[instrument]
pub fn evaluate_statement(
    request: &Request,
    statement: &Statement,
    statement_index: i32,
) -> Result<Option<Effect>, EvaluationError> {
    let mut effects: Vec<Option<Effect>> = Default::default();
    let id = statement_id(statement, statement_index);

    // >>>>> eval principal
    effects.push(eval_statement_principal(
        &request.principal,
        &statement.principal,
    ));

    // >>>>> eval action
    effects.push(eval_statement_action(&request.action, &statement.action));

    // >>>>> eval resource
    effects.push(eval_statement_resource(
        &request.resource,
        &statement.resource,
    ));

    // >>>>> eval conditions
    effects.push(eval_statement_conditions(
        &request.environment,
        &statement.condition,
    )?);

    // collapse result
    let effect = if effects.contains(&Some(Effect::Deny)) {
        Some(Effect::Deny)
    } else if effects.contains(&Some(Effect::Allow)) {
        Some(Effect::Allow)
    } else {
        None
    };
    info!("Returning statement {} effect {:?}", id, &effect);
    Ok(effect)
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
) -> Option<Effect> {
    let effect = if let Some(principal) = request_principal {
        match statement_principal {
            None => None,
            Some(Principal::Principal(ps)) => {
                if let Some(p) = ps.get(&principal.principal_type) {
                    match p {
                        OneOrAny::Any => Some(Effect::Allow),
                        OneOrAny::One(v) => {
                            if v == &principal.identifier {
                                Some(Effect::Allow)
                            } else {
                                Some(Effect::Deny)
                            }
                        }
                        OneOrAny::AnyOf(vs) => {
                            if vs.contains(&principal.identifier) {
                                Some(Effect::Allow)
                            } else {
                                Some(Effect::Deny)
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
                        OneOrAny::Any => Some(Effect::Deny),
                        OneOrAny::One(v) => {
                            if v == &principal.identifier {
                                Some(Effect::Deny)
                            } else {
                                Some(Effect::Allow)
                            }
                        }
                        OneOrAny::AnyOf(vs) => {
                            if vs.contains(&principal.identifier) {
                                Some(Effect::Deny)
                            } else {
                                Some(Effect::Allow)
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
fn eval_statement_action(request_action: &QString, statement_action: &Action) -> Option<Effect> {
    let effect = match statement_action {
        Action::Action(a) => match a {
            OneOrAny::Any => Some(Effect::Allow),
            OneOrAny::One(v) => {
                if v == request_action {
                    Some(Effect::Allow)
                } else {
                    Some(Effect::Deny)
                }
            }
            OneOrAny::AnyOf(vs) => {
                if vs.contains(request_action) {
                    Some(Effect::Allow)
                } else {
                    Some(Effect::Deny)
                }
            }
        },
        Action::NotAction(a) => match a {
            OneOrAny::Any => Some(Effect::Deny),
            OneOrAny::One(v) => {
                if v == request_action {
                    Some(Effect::Deny)
                } else {
                    Some(Effect::Allow)
                }
            }
            OneOrAny::AnyOf(vs) => {
                if vs.contains(request_action) {
                    Some(Effect::Deny)
                } else {
                    Some(Effect::Allow)
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
) -> Option<Effect> {
    let effect = match statement_resource {
        Resource::Resource(a) => match a {
            OneOrAny::Any => Some(Effect::Allow),
            OneOrAny::One(v) => {
                if v == request_resource {
                    Some(Effect::Allow)
                } else {
                    Some(Effect::Deny)
                }
            }
            OneOrAny::AnyOf(vs) => {
                if vs.contains(request_resource) {
                    Some(Effect::Allow)
                } else {
                    Some(Effect::Deny)
                }
            }
        },
        Resource::NotResource(a) => match a {
            OneOrAny::Any => Some(Effect::Deny),
            OneOrAny::One(v) => {
                if v == request_resource {
                    Some(Effect::Deny)
                } else {
                    Some(Effect::Allow)
                }
            }
            OneOrAny::AnyOf(vs) => {
                if vs.contains(request_resource) {
                    Some(Effect::Deny)
                } else {
                    Some(Effect::Allow)
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

#[instrument]
fn eval_statement_conditions(
    request_environment: &Environment,
    statement_conditions: &Option<
        HashMap<ConditionOperator, HashMap<QString, OneOrAll<ConditionValue>>>,
    >,
) -> Result<Option<Effect>, EvaluationError> {
    let result = if let Some(conditions) = statement_conditions {
        let (effects, errors): (
            Vec<Result<Option<Effect>, EvaluationError>>,
            Vec<Result<Option<Effect>, EvaluationError>>,
        ) = conditions
            .iter()
            .map(|(op, vs)| eval_statement_condition_op(request_environment, op, vs))
            .flatten()
            .partition(|r| r.is_ok());
        if !errors.is_empty() {
            Err(EvaluationError::Errors(
                errors
                    .iter()
                    .map(|r| r.as_ref().err().unwrap())
                    .cloned()
                    .collect(),
            ))
        } else if effects.contains(&Ok(Some(Effect::Deny))) {
            Ok(Some(Effect::Deny))
        } else if effects.contains(&Ok(Some(Effect::Allow))) {
            Ok(Some(Effect::Allow))
        } else {
            Ok(None)
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
) -> Vec<Result<Option<Effect>, EvaluationError>> {
    info!("Statement condition, operator {:?}", condition_operator);
    let results: Vec<Result<Option<Effect>, EvaluationError>> = condition_values
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
) -> Result<Option<Effect>, EvaluationError> {
    match request_environment.get(condition_key) {
        None => {
            if condition_operator.if_exists {
                Ok(Some(Effect::Allow))
            } else {
                Ok(None)
            }
        }
        Some(lhs) => match (&condition_operator.quantifier, condition_values) {
            (None, OneOrAll::One(rhs)) => {
                operators::evaluate(request_environment, &condition_operator.operator, lhs, rhs)
                    .map(bool_effect)
            }
            (Some(ConditionOperatorQuantifier::ForAllValues), OneOrAll::All(rhs)) => {
                operators::evaluate_all(request_environment, &condition_operator.operator, lhs, rhs)
                    .map(bool_effect)
            }
            (Some(ConditionOperatorQuantifier::ForAnyValue), OneOrAll::All(rhs)) => {
                operators::evaluate_any(request_environment, &condition_operator.operator, lhs, rhs)
                    .map(bool_effect)
            }
            _ => Err(EvaluationError::InvalidValueCardinality),
        },
    }
}

fn bool_effect(result: bool) -> Option<Effect> {
    if result {
        Some(Effect::Allow)
    } else {
        Some(Effect::Deny)
    }
}
