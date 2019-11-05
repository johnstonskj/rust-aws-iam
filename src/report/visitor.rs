use crate::model::*;
use std::collections::HashMap;
use std::io::Write;
use std::ops::Deref;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[allow(unused_variables)]
pub trait PolicyVisitor {
    fn start(&self, writer: &mut dyn Write) {}

    fn id(&self, writer: &mut dyn Write, i: &String) {}

    fn version(&self, writer: &mut dyn Write, v: &Version) {}

    fn statement<'a>(&'a self) -> Option<Box<&'a dyn StatementVisitor>> {
        None
    }

    fn finish(&self, writer: &mut dyn Write) {}
}

#[allow(unused_variables)]
pub trait StatementVisitor {
    fn start(&self, writer: &mut dyn Write) {}

    fn sid(&self, writer: &mut dyn Write, s: &String) {}

    fn effect(&self, writer: &mut dyn Write, e: &Effect) {}

    fn principal(&self, writer: &mut dyn Write, p: &Principal) {}

    fn action(&self, writer: &mut dyn Write, a: &Action) {}

    fn resource(&self, writer: &mut dyn Write, r: &Resource) {}

    fn condition<'a>(&'a self) -> Option<Box<&'a dyn ConditionVisitor>> {
        None
    }

    fn finish(&self, writer: &mut dyn Write) {}
}

#[allow(unused_variables)]
pub trait ConditionVisitor {
    fn start(&self, writer: &mut dyn Write) {}

    fn left(&self, writer: &mut dyn Write, f: &QString) {}

    fn operator(&self, writer: &mut dyn Write, op: &ConditionOperator) {}

    fn right(&self, writer: &mut dyn Write, v: &OneOrAll<ConditionValue>) {}

    fn finish(&self, writer: &mut dyn Write) {}
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

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
            visitor.left(writer, field);
            visitor.operator(writer, op);
            visitor.right(writer, values);
            visitor.finish(writer);
        }
    }
}
