/*!
This module provides the capabilities to walk a Policy struct and generate reports. This could
be concerning domain-specific validation, or simply documentation.
*/

use crate::model::*;
use std::collections::HashMap;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Walk the elements of a `Policy` struct. The implementation of this trait will be called
/// by `walk_policy`.
///
/// 1. `start()`
/// 1. `id()`
/// 1. `version()`
/// 1. let statement visitor = `statement()`
/// 1. if statement visitor, visit each statement in turn (in the order they are in the JSON file)
/// 1. `finish()`
///
pub trait PolicyVisitor {
    /// Called to signal the walker has started a Policy.
    fn start(&mut self) {}

    /// Called by the walker to allow handling of the `id` component of the Policy.
    fn id(&mut self, i: &String) {}

    /// Called by the walker to allow handling of the `version` component of the Policy.
    fn version(&mut self, v: &Version) {}

    /// Return an associated `StatementVisitor` if necessary.
    fn statement_visitor(&mut self) -> Option<Box<&mut dyn StatementVisitor>> {
        None
    }

    /// Called to signal the walker has finished the Policy.
    fn finish(&mut self) {}
}

///
/// Walk the elements of a `Statement` struct. The implementation of this trait will be called
/// by `walk_policy` in the following order.
///
/// 1. `start()`
/// 1. `sid()`
/// 1. `effect()`
/// 1. `principal()`
/// 1. `action()`
/// 1. `resource()`
/// 1. let condition visitor = `condition()`
/// 1. if condition visitor, visit each condition in turn (in the order they are in the JSON file)
/// 1. `finish()`
///
pub trait StatementVisitor {
    /// Called to signal the walker has started a Statement.
    fn start(&mut self) {}

    /// Called by the walker to allow handling of the `sid` component of the Statement.
    fn sid(&mut self, s: &String) {}

    /// Called by the walker to allow handling of the `effect` component of the Statement.
    fn effect(&mut self, e: &Effect) {}

    /// Called by the walker to allow handling of the `principal` component of the Statement.
    fn principal(&mut self, p: &Principal) {}

    /// Called by the walker to allow handling of the `action` component of the Statement.
    fn action(&mut self, a: &Action) {}

    /// Called by the walker to allow handling of the `resource` component of the Statement.
    fn resource(&mut self, r: &Resource) {}

    /// Return an associated `ConditionVisitor` if necessary.
    fn condition_visitor(&mut self) -> Option<Box<&mut dyn ConditionVisitor>> {
        None
    }

    /// Called to signal the walker has finished the Statement.
    fn finish(&mut self) {}
}

///
/// Walk the elements of a `Condition` struct. The implementation of this trait will be called
/// by `walk_policy` in the following order.
///
/// 1. `start()`
/// 1. `left()`
/// 1. `operator()`
/// 1. `right()`
/// 1. `finish()`
///
pub trait ConditionVisitor {
    /// Called to signal the walker has started a Condition.
    fn start(&mut self) {}

    /// Called by the walker to allow handling of the `left` component of the Condition.
    fn left(&mut self, f: &QString, op: &ConditionOperator) {}

    /// Called by the walker to allow handling of the `operator` component of the Condition.
    fn operator(&mut self, op: &ConditionOperator) {}

    /// Called by the walker to allow handling of the `right` component of the Condition.
    fn right(&mut self, v: &OneOrAll<ConditionValue>, op: &ConditionOperator) {}

    /// Called to signal the walker has finished the Condition.
    fn finish(&mut self) {}
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// The entry-point for walking a policy. The `visitor` implementation will be called in-order
/// with each component of `policy` and may choose to use the `writer`.
///
pub fn walk_policy(policy: &Policy, visitor: &mut impl PolicyVisitor) {
    visitor.start();
    if let Some(id) = &policy.id {
        visitor.id(id);
    }
    if let Some(version) = &policy.version {
        visitor.version(version);
    }
    if let Some(statement_visitor) = visitor.statement_visitor() {
        match &policy.statement {
            OneOrAll::One(statement) => walk_statement(statement, statement_visitor),
            OneOrAll::All(statements) => {
                for statement in statements {
                    walk_statement(statement, visitor.statement_visitor().unwrap())
                }
            }
        }
    }
    visitor.finish();
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn walk_statement(statement: &Statement, visitor: Box<&mut dyn StatementVisitor>) {
    visitor.start();
    if let Some(sid) = &statement.sid {
        visitor.sid(sid);
    }
    visitor.effect(&statement.effect);
    if let Some(principal) = &statement.principal {
        visitor.principal(principal);
    }
    visitor.action(&statement.action);
    visitor.resource(&statement.resource);
    if let Some(condition_visitor) = visitor.condition_visitor() {
        if let Some(conditions) = &statement.condition {
            walk_conditions(conditions, condition_visitor)
        }
    }
    visitor.finish();
}

fn walk_conditions(
    conditions: &HashMap<ConditionOperator, HashMap<QString, OneOrAll<ConditionValue>>>,
    visitor: Box<&mut dyn ConditionVisitor>,
) {
    for (op, rhs) in conditions {
        for (field, values) in rhs {
            visitor.start();
            visitor.left(field, op);
            visitor.operator(op);
            visitor.right(values, op);
            visitor.finish();
        }
    }
}
