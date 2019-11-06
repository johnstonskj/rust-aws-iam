/*!
This module provides the capabilities to walk a Policy struct and generate reports. This could
be concerning domain-specific validation, or simply documentation.
*/

use crate::model::*;
use std::collections::HashMap;
use std::io::Write;
use std::ops::Deref;

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
#[allow(unused_variables)]
pub trait PolicyVisitor {
    /// Called to signal the walker has started a Policy.
    fn start(&self, writer: &mut dyn Write) {}

    /// Called by the walker to allow handling of the `id` component of the Policy.
    fn id(&self, writer: &mut dyn Write, i: &String) {}

    /// Called by the walker to allow handling of the `version` component of the Policy.
    fn version(&self, writer: &mut dyn Write, v: &Version) {}

    /// Return an associated `StatementVisitor` if necessary.
    fn statement<'a>(&'a self) -> Option<Box<&'a dyn StatementVisitor>> {
        None
    }

    /// Called to signal the walker has finished the Policy.
    fn finish(&self, writer: &mut dyn Write) {}
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
#[allow(unused_variables)]
pub trait StatementVisitor {
    /// Called to signal the walker has started a Statement.
    fn start(&self, writer: &mut dyn Write) {}

    /// Called by the walker to allow handling of the `sid` component of the Statement.
    fn sid(&self, writer: &mut dyn Write, s: &String) {}

    /// Called by the walker to allow handling of the `effect` component of the Statement.
    fn effect(&self, writer: &mut dyn Write, e: &Effect) {}

    /// Called by the walker to allow handling of the `principal` component of the Statement.
    fn principal(&self, writer: &mut dyn Write, p: &Principal) {}

    /// Called by the walker to allow handling of the `action` component of the Statement.
    fn action(&self, writer: &mut dyn Write, a: &Action) {}

    /// Called by the walker to allow handling of the `resource` component of the Statement.
    fn resource(&self, writer: &mut dyn Write, r: &Resource) {}

    /// Return an associated `ConditionVisitor` if necessary.
    fn condition<'a>(&'a self) -> Option<Box<&'a dyn ConditionVisitor>> {
        None
    }

    /// Called to signal the walker has finished the Statement.
    fn finish(&self, writer: &mut dyn Write) {}
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
#[allow(unused_variables)]
pub trait ConditionVisitor {
    /// Called to signal the walker has started a Condition.
    fn start(&self, writer: &mut dyn Write) {}

    /// Called by the walker to allow handling of the `left` component of the Condition.
    fn left(&self, writer: &mut dyn Write, f: &QString, op: &ConditionOperator) {}

    /// Called by the walker to allow handling of the `operator` component of the Condition.
    fn operator(&self, writer: &mut dyn Write, op: &ConditionOperator) {}

    /// Called by the walker to allow handling of the `right` component of the Condition.
    fn right(&self, writer: &mut dyn Write, v: &OneOrAll<ConditionValue>, op: &ConditionOperator) {}

    /// Called to signal the walker has finished the Condition.
    fn finish(&self, writer: &mut dyn Write) {}
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// The entry-point for walking a policy. The `visitor` implementation will be called in-order
/// with each component of `policy` and may choose to use the `writer`.
///
pub fn walk_policy(policy: &Policy, visitor: &dyn PolicyVisitor, writer: &mut dyn Write) {
    visitor.start(writer);
    if let Some(id) = &policy.id {
        visitor.id(writer, id);
    }
    if let Some(version) = &policy.version {
        visitor.version(writer, version);
    }
    if let Some(statement_visitor) = visitor.statement() {
        let statement_visitor = *statement_visitor.deref();
        match &policy.statement {
            OneOrAll::One(statement) => walk_statement(statement, statement_visitor, writer),
            OneOrAll::All(statements) => {
                for statement in statements {
                    walk_statement(statement, statement_visitor, writer)
                }
            }
        }
    }
    visitor.finish(writer);
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn walk_statement(statement: &Statement, visitor: &dyn StatementVisitor, writer: &mut dyn Write) {
    visitor.start(writer);
    if let Some(sid) = &statement.sid {
        visitor.sid(writer, sid);
    }
    visitor.effect(writer, &statement.effect);
    if let Some(principal) = &statement.principal {
        visitor.principal(writer, principal);
    }
    visitor.action(writer, &statement.action);
    visitor.resource(writer, &statement.resource);
    if let Some(condition_visitor) = visitor.condition() {
        let condition_visitor = *condition_visitor.deref();
        if let Some(conditions) = &statement.condition {
            walk_conditions(conditions, condition_visitor, writer)
        }
    }
    visitor.finish(writer);
}

fn walk_conditions(
    conditions: &HashMap<ConditionOperator, HashMap<QString, OneOrAll<ConditionValue>>>,
    visitor: &dyn ConditionVisitor,
    writer: &mut dyn Write,
) {
    for (op, rhs) in conditions {
        for (field, values) in rhs {
            visitor.start(writer);
            visitor.left(writer, field, op);
            visitor.operator(writer, op);
            visitor.right(writer, values, op);
            visitor.finish(writer);
        }
    }
}
