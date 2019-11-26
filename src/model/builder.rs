/*!
Provides a convenient and fluent builder interface for constructing policies.

# Example

```rust
use aws_iam::model::*;
use aws_iam::model::builder::*;
use aws_iam::io::write_to_writer;
use std::io::stdout;

let policy: Policy = PolicyBuilder::new()
    .named("confidential-data-access")
    .evaluate_statement(
        StatementBuilder::new()
            .auto_named()
            .allows()
            .unspecified_principals()
            .may_perform_actions(vec!["s3:List*", "s3:Get*"])
            .on_resources(vec![
                "arn:aws:s3:::confidential-data",
                "arn:aws:s3:::confidential-data/_*",
            ])
            .if_condition(
                ConditionBuilder::new_bool()
                    .right_hand_bool("aws:MultiFactorAuthPresent", true)
                    .if_exists(),
            ),
    )
    .into();
write_to_writer(stdout(), &policy);
```
*/

use crate::model::*;
use std::collections::HashMap;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The top-level `Policy` builder.
///
#[derive(Debug)]
pub struct PolicyBuilder {
    version: Option<Version>,
    id: Option<String>,
    statements: Vec<Statement>,
}

///
/// A `Statement` builder, used with `PolicyBuilder::evaluate_statement()`.
///
#[derive(Debug, Clone)]
pub struct StatementBuilder {
    sid: Option<String>,
    effect: Effect,
    principals: HashMap<PrincipalType, Vec<String>>,
    p_direction: Option<bool>,
    actions: Vec<QString>,
    a_direction: Option<bool>,
    resources: Vec<String>,
    r_direction: Option<bool>,
    condition: Option<HashMap<ConditionOperator, HashMap<QString, OneOrAll<ConditionValue>>>>,
}

///
/// A `Condition` builder, used with `StatementBuilder::if_condition()`.
#[derive(Debug)]
pub struct ConditionBuilder {
    operator: ConditionOperator,
    rhs: HashMap<QString, OneOrAll<ConditionValue>>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for PolicyBuilder {
    fn default() -> Self {
        PolicyBuilder {
            version: None,
            id: None,
            statements: Vec::new(),
        }
    }
}

impl PolicyBuilder {
    /// Create a new, empty, policy builder
    pub fn new() -> Self {
        Default::default()
    }

    /// Set the version of this policy.
    pub fn version(&mut self, version: Version) -> &mut Self {
        self.version = Some(version);
        self
    }

    /// Use the IAM default for the version of this policy
    pub fn default_version(&mut self) -> &mut Self {
        self.version = Some(Policy::default_version());
        self
    }

    /// Set the id of this policy
    pub fn named(&mut self, id: &str) -> &mut Self {
        self.id = Some(id.to_string());
        self
    }

    /// Set the id of this policy to a randomly generate value.
    pub fn auto_named(&mut self) -> &mut Self {
        self.id = Some(Policy::new_id());
        self
    }

    /// Add a statement to this policy.
    pub fn evaluate_statement(&mut self, statement: &mut StatementBuilder) -> &mut Self {
        self.statements.push(statement.into());
        self
    }

    /// Add a list of statements to this policy.
    pub fn evaluate_statements(&mut self, statements: &mut Vec<StatementBuilder>) -> &mut Self {
        self.statements.extend(
            statements
                .iter_mut()
                .map(|sb| sb.into())
                .collect::<Vec<Statement>>(),
        );
        self
    }
}

impl From<&mut PolicyBuilder> for Policy {
    fn from(pb: &mut PolicyBuilder) -> Self {
        Policy {
            version: pb.version.clone(),
            id: pb.id.clone(),
            statement: match pb.statements.len() {
                0 => panic!("no statements!"),
                1 => OneOrAll::One(pb.statements.remove(0)),
                _ => OneOrAll::All(pb.statements.drain(0..).map(|sb| sb).collect()),
            },
        }
    }
}

impl Default for StatementBuilder {
    fn default() -> Self {
        StatementBuilder {
            sid: None,
            effect: Effect::Deny,
            principals: HashMap::new(),
            p_direction: None,
            actions: Vec::new(),
            a_direction: None,
            resources: Vec::new(),
            r_direction: None,
            condition: None,
        }
    }
}
impl StatementBuilder {
    /// Create a new, empty, statement builder
    pub fn new() -> Self {
        Default::default()
    }

    /// Set the id of this statement
    pub fn named(&mut self, sid: &str) -> &mut Self {
        self.sid = Some(sid.to_string());
        self
    }

    /// Set the id of this statement to a randomly generate value.
    pub fn auto_named(&mut self) -> &mut Self {
        self.sid = Some(Statement::new_sid());
        self
    }

    /// Set the effect of this statement to `Allow`.
    pub fn allows(&mut self) -> &mut Self {
        self.effect = Effect::Allow;
        self
    }

    /// Set the effect of this statement to `Deny`.
    pub fn does_not_allow(&mut self) -> &mut Self {
        self.effect = Effect::Deny;
        self
    }

    /// Unsets the principal associated with this statement
    pub fn unspecified_principals(&mut self) -> &mut Self {
        self.principals.clear();
        self
    }

    /// Sets the principal of this statement to be a wildcard.
    pub fn any_principal(&mut self, p_type: PrincipalType) -> &mut Self {
        self.p_direction = Some(true);
        self.principals.insert(p_type, Vec::new());
        self
    }

    /// Sets the principal of this statement to be only this value.
    pub fn only_this_principal(&mut self, p_type: PrincipalType, arn: &str) -> &mut Self {
        self.only_these_principals(p_type, vec![arn]);
        self
    }

    /// Sets the principal of this statement to be any of these values.
    pub fn only_these_principals(&mut self, p_type: PrincipalType, arns: Vec<&str>) -> &mut Self {
        match self.p_direction {
            None => self.p_direction = Some(true),
            Some(false) => panic!("you can't have principal *and* not principal"),
            _ => (),
        };
        let existing = self.principals.entry(p_type).or_default();
        existing.extend(arns.iter().map(|s| s.to_string()).collect::<Vec<String>>());
        self
    }

    /// Sets the principal of this statement to exclude this value.
    pub fn not_this_principal(&mut self, p_type: PrincipalType, arn: &str) -> &mut Self {
        self.not_these_principals(p_type, vec![arn]);
        self
    }

    /// Sets the principal of this statement to exclude of these values.
    pub fn not_these_principals(&mut self, p_type: PrincipalType, arns: Vec<&str>) -> &mut Self {
        match self.p_direction {
            None => self.p_direction = Some(false),
            Some(true) => panic!("you can't have principal *and* not principal"),
            _ => (),
        };
        let existing = self.principals.entry(p_type).or_default();
        existing.extend(arns.iter().map(|s| s.to_string()).collect::<Vec<String>>());
        self
    }

    /// Sets the action of this statement to be a wildcard.
    pub fn may_perform_any_action(&mut self) -> &mut Self {
        self.a_direction = Some(true);
        self.actions = Vec::new();
        self
    }

    /// Sets the action of this statement to be only this value.
    pub fn may_perform_action(&mut self, action: &str) -> &mut Self {
        self.may_perform_actions(vec![action]);
        self
    }

    /// Sets the action of this statement to be any of these values.
    pub fn may_perform_actions(&mut self, actions: Vec<&str>) -> &mut Self {
        match self.a_direction {
            None => self.a_direction = Some(true),
            Some(false) => panic!("you can't have action *and* not action"),
            _ => (),
        };
        self.actions.extend(
            actions
                .iter()
                .map(|s| s.parse().unwrap())
                .collect::<Vec<QString>>(),
        );
        self
    }

    /// Sets the action of this statement to exclude the wildcard.
    pub fn may_perform_no_action(&mut self) -> &mut Self {
        self.a_direction = Some(false);
        self.actions = Vec::new();
        self
    }

    /// Sets the action of this statement to exclude this value.
    pub fn may_not_perform_action(&mut self, action: &str) -> &mut Self {
        self.may_not_perform_actions(vec![action]);
        self
    }

    /// Sets the action of this statement to exclude any of these values.
    pub fn may_not_perform_actions(&mut self, actions: Vec<&str>) -> &mut Self {
        match self.a_direction {
            None => self.a_direction = Some(false),
            Some(true) => panic!("you can't have action *and* not action"),
            _ => (),
        };
        self.actions.extend(
            actions
                .iter()
                .map(|s| s.parse().unwrap())
                .collect::<Vec<QString>>(),
        );
        self
    }

    /// Sets the resource of this statement to be a wildcard.
    pub fn on_any_resource(&mut self) -> &mut Self {
        self.r_direction = Some(true);
        self.resources = Vec::new();
        self
    }

    /// Sets the resource of this statement to be only this value.
    pub fn on_resource(&mut self, resource: &str) -> &mut Self {
        self.on_resources(vec![resource]);
        self
    }

    /// Sets the resource of this statement to be any of these values.
    pub fn on_resources(&mut self, resources: Vec<&str>) -> &mut Self {
        match self.r_direction {
            None => self.r_direction = Some(true),
            Some(false) => panic!("you can't have resource *and* not resource"),
            _ => (),
        };
        self.resources.extend(
            resources
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>(),
        );
        self
    }

    /// Sets the resource of this statement to exclude the wildcard.
    pub fn on_no_resource(&mut self) -> &mut Self {
        self.r_direction = Some(false);
        self.resources = Vec::new();
        self
    }

    /// Sets the resource of this statement to exclude this value.
    pub fn not_on_resource(&mut self, resource: &str) -> &mut Self {
        self.not_on_resources(vec![resource]);
        self
    }

    /// Sets the resource of this statement to exclude any of these values.
    pub fn not_on_resources(&mut self, resources: Vec<&str>) -> &mut Self {
        match self.r_direction {
            None => self.r_direction = Some(false),
            Some(true) => panic!("you can't have resource *and* not resource"),
            _ => (),
        };
        self.resources.extend(
            resources
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>(),
        );
        self
    }

    /// Adds this condition to the statement.
    pub fn if_condition(&mut self, condition: &mut ConditionBuilder) -> &mut Self {
        if self.condition.is_none() {
            self.condition = Some(HashMap::new());
        }
        let conditions = self.condition.as_mut().unwrap();
        let existing = conditions.entry(condition.operator.clone()).or_default();
        existing.extend(condition.rhs.drain());
        self
    }
}

impl From<&mut StatementBuilder> for Statement {
    fn from(sb: &mut StatementBuilder) -> Self {
        let principal = match sb.p_direction {
            None => None,
            Some(direction) => {
                let inner: HashMap<PrincipalType, OneOrAny> = sb
                    .principals
                    .iter_mut()
                    .map(|(k, v)| {
                        (
                            k.clone(),
                            match v.len() {
                                0 => OneOrAny::Any,
                                1 => OneOrAny::One(v.remove(0)),
                                _ => OneOrAny::AnyOf(v.drain(0..).map(|s| s).collect()),
                            },
                        )
                    })
                    .collect();
                Some(if direction {
                    Principal::Principal(inner)
                } else {
                    Principal::NotPrincipal(inner)
                })
            }
        };

        let action_inner = match sb.actions.len() {
            0 => OneOrAny::Any,
            1 => OneOrAny::One(sb.actions.remove(0)),
            _ => OneOrAny::AnyOf(sb.actions.drain(0..).map(|s| s).collect()),
        };
        let action = match sb.a_direction {
            None => panic!("must have an action"),
            Some(true) => Action::Action(action_inner),
            Some(false) => Action::NotAction(action_inner),
        };

        let resource_inner = match sb.resources.len() {
            0 => OneOrAny::Any,
            1 => OneOrAny::One(sb.resources.remove(0)),
            _ => OneOrAny::AnyOf(sb.resources.drain(0..).map(|s| s).collect()),
        };
        let resource = match sb.r_direction {
            None => panic!("must have a resource"),
            Some(true) => Resource::Resource(resource_inner),
            Some(false) => Resource::NotResource(resource_inner),
        };

        Statement {
            sid: sb.sid.clone(),
            principal,
            effect: sb.effect.clone(),
            action,
            resource,
            condition: sb.condition.clone(),
        }
    }
}

impl ConditionBuilder {
    /// Create a new Condition with the provided operator.
    pub fn new(operator: GlobalConditionOperator) -> Self {
        ConditionBuilder {
            operator: ConditionOperator {
                quantifier: None,
                operator,
                if_exists: false,
            },
            rhs: Default::default(),
        }
    }

    /// Create a new Condition with operator = `StringEquals`
    pub fn new_string_equals() -> Self {
        ConditionBuilder {
            operator: ConditionOperator {
                quantifier: None,
                operator: GlobalConditionOperator::StringEquals,
                if_exists: false,
            },
            rhs: Default::default(),
        }
    }

    /// Create a new Condition with operator = `StringNotEquals`
    pub fn new_string_not_equals() -> Self {
        ConditionBuilder {
            operator: ConditionOperator {
                quantifier: None,
                operator: GlobalConditionOperator::StringNotEquals,
                if_exists: false,
            },
            rhs: Default::default(),
        }
    }

    /// Create a new Condition with operator = `NumericEquals`
    pub fn new_numeric_equals() -> Self {
        ConditionBuilder {
            operator: ConditionOperator {
                quantifier: None,
                operator: GlobalConditionOperator::NumericEquals,
                if_exists: false,
            },
            rhs: Default::default(),
        }
    }

    /// Create a new Condition with operator = `NumericNotEquals`
    pub fn new_numeric_not_equals() -> Self {
        ConditionBuilder {
            operator: ConditionOperator {
                quantifier: None,
                operator: GlobalConditionOperator::NumericNotEquals,
                if_exists: false,
            },
            rhs: Default::default(),
        }
    }

    /// Create a new Condition with operator = `Bool`
    pub fn new_bool() -> Self {
        ConditionBuilder {
            operator: ConditionOperator {
                quantifier: None,
                operator: GlobalConditionOperator::Bool,
                if_exists: false,
            },
            rhs: Default::default(),
        }
    }

    /// Add the _for-all-values_ quantifier.
    pub fn for_all(&mut self) -> &mut Self {
        self.operator.quantifier = Some(ConditionOperatorQuantifier::ForAllValues);
        self
    }

    /// Add the _for-any-value_ quantifier.
    pub fn for_any(&mut self) -> &mut Self {
        self.operator.quantifier = Some(ConditionOperatorQuantifier::ForAnyValue);
        self
    }

    /// Add a list of values to the _right-hand-sidse_ of this condition.
    pub fn right_hand_side(&mut self, key: &str, values: &mut Vec<ConditionValue>) -> &mut Self {
        let values = match values.len() {
            0 => panic!("you must specify at least one value"),
            1 => OneOrAll::One(values.remove(0)),
            _ => OneOrAll::All(values.drain(0..).collect()),
        };
        self.rhs.insert(key.parse().unwrap(), values);
        self
    }

    /// Add a string value to the _right-hand-sidse_ of this condition.
    pub fn right_hand_str(&mut self, key: &str, value: &str) -> &mut Self {
        self.rhs.insert(
            key.parse().unwrap(),
            OneOrAll::One(ConditionValue::String(value.to_string())),
        );
        self
    }

    /// Add a integer value to the _right-hand-sidse_ of this condition.
    pub fn right_hand_int(&mut self, key: &str, value: i64) -> &mut Self {
        self.rhs.insert(
            key.parse().unwrap(),
            OneOrAll::One(ConditionValue::Integer(value)),
        );
        self
    }

    /// Add a float value to the _right-hand-sidse_ of this condition.
    pub fn right_hand_float(&mut self, key: &str, value: f64) -> &mut Self {
        self.rhs.insert(
            key.parse().unwrap(),
            OneOrAll::One(ConditionValue::Float(value)),
        );
        self
    }

    /// Add a boolean value to the _right-hand-sidse_ of this condition.
    pub fn right_hand_bool(&mut self, key: &str, value: bool) -> &mut Self {
        self.rhs.insert(
            key.parse().unwrap(),
            OneOrAll::One(ConditionValue::Bool(value)),
        );
        self
    }

    /// Add the _if-exists_ constraint
    pub fn if_exists(&mut self) -> &mut Self {
        self.operator.if_exists = true;
        self
    }

    ///
    /// Convert this one condition into a complete Condition for a statement.
    ///
    pub fn build_as_condition(
        &self,
    ) -> HashMap<ConditionOperator, HashMap<QString, OneOrAll<ConditionValue>>> {
        let mut map: HashMap<ConditionOperator, HashMap<QString, OneOrAll<ConditionValue>>> =
            HashMap::default();
        map.insert(self.operator.clone(), self.rhs.clone());
        map
    }
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::io::write_to_writer;
    use std::io::stdout;

    #[test]
    fn test_simple_builder() {
        let policy: Policy = PolicyBuilder::new()
            .named("confidential-data-access")
            .evaluate_statement(
                StatementBuilder::new()
                    .auto_named()
                    .allows()
                    .unspecified_principals()
                    .may_perform_actions(vec!["s3:List*", "s3:Get*"])
                    .on_resources(vec![
                        "arn:aws:s3:::confidential-data",
                        "arn:aws:s3:::confidential-data/*",
                    ])
                    .if_condition(
                        ConditionBuilder::new_bool()
                            .right_hand_bool("aws:MultiFactorAuthPresent", true)
                            .if_exists(),
                    ),
            )
            .into();
        write_to_writer(stdout(), &policy).expect("well that was unexpected");
    }
}
