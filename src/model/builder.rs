/*!
Provides a convenient and fluent builder interface for constructing policies.

# Example

```rust
use aws_iam::model::*;
use aws_iam::model::builder::*;

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
println!("{}", policy);
```

*/

use crate::model::*;
use std::collections::HashMap;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct PolicyBuilder {
    version: Option<Version>,
    id: Option<String>,
    statements: Vec<Statement>,
}

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
    pub fn new() -> Self {
        Default::default()
    }

    pub fn version(&mut self, version: Version) -> &mut Self {
        self.version = Some(version);
        self
    }

    pub fn default_version(&mut self) -> &mut Self {
        self.version = Some(Policy::default_version());
        self
    }

    pub fn named(&mut self, id: &str) -> &mut Self {
        self.id = Some(id.to_string());
        self
    }

    pub fn auto_named(&mut self) -> &mut Self {
        self.id = Some(Policy::new_id());
        self
    }

    pub fn evaluate_statement(&mut self, statement: &mut StatementBuilder) -> &mut Self {
        self.statements.push(statement.into());
        self
    }

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
    pub fn new() -> Self {
        Default::default()
    }

    pub fn named(&mut self, sid: &str) -> &mut Self {
        self.sid = Some(sid.to_string());
        self
    }

    pub fn auto_named(&mut self) -> &mut Self {
        self.sid = Some(Statement::new_sid());
        self
    }

    pub fn allows(&mut self) -> &mut Self {
        self.effect = Effect::Allow;
        self
    }

    pub fn does_not_allow(&mut self) -> &mut Self {
        self.effect = Effect::Deny;
        self
    }

    pub fn unspecified_principals(&mut self) -> &mut Self {
        self
    }

    pub fn any_principal(&mut self, p_type: PrincipalType) -> &mut Self {
        self.p_direction = Some(true);
        self.principals.insert(p_type, Vec::new());
        self
    }

    pub fn only_this_principal(&mut self, p_type: PrincipalType, arn: &str) -> &mut Self {
        self.only_these_principals(p_type, vec![arn]);
        self
    }

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

    pub fn not_this_principal(&mut self, p_type: PrincipalType, arn: &str) -> &mut Self {
        self.not_these_principals(p_type, vec![arn]);
        self
    }

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

    pub fn may_perform_any_action(&mut self) -> &mut Self {
        self.a_direction = Some(true);
        self.actions = Vec::new();
        self
    }

    pub fn may_perform_action(&mut self, action: &str) -> &mut Self {
        self.may_perform_actions(vec![action]);
        self
    }

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

    pub fn may_perform_no_action(&mut self) -> &mut Self {
        self.a_direction = Some(false);
        self.actions = Vec::new();
        self
    }

    pub fn may_not_perform_action(&mut self, action: &str) -> &mut Self {
        self.may_not_perform_actions(vec![action]);
        self
    }

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

    pub fn on_any_resource(&mut self) -> &mut Self {
        self.r_direction = Some(true);
        self.resources = Vec::new();
        self
    }

    pub fn on_resource(&mut self, resource: &str) -> &mut Self {
        self.on_resources(vec![resource]);
        self
    }

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

    pub fn on_no_resource(&mut self) -> &mut Self {
        self.r_direction = Some(false);
        self.resources = Vec::new();
        self
    }

    pub fn not_on_resource(&mut self, resource: &str) -> &mut Self {
        self.not_on_resources(vec![resource]);
        self
    }

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

    pub fn if_condition(&mut self, condition: &mut ConditionBuilder) -> &mut Self {
        if let None = self.condition {
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
    pub fn new(operator: GlobalConditionOperator) -> Self {
        ConditionBuilder {
            operator: ConditionOperator {
                quantifier: None,
                operator,
                only_if_exists: false,
            },
            rhs: Default::default(),
        }
    }

    pub fn new_string_equals() -> Self {
        ConditionBuilder {
            operator: ConditionOperator {
                quantifier: None,
                operator: GlobalConditionOperator::StringEquals,
                only_if_exists: false,
            },
            rhs: Default::default(),
        }
    }

    pub fn new_string_not_equals() -> Self {
        ConditionBuilder {
            operator: ConditionOperator {
                quantifier: None,
                operator: GlobalConditionOperator::StringNotEquals,
                only_if_exists: false,
            },
            rhs: Default::default(),
        }
    }

    pub fn new_numeric_equals() -> Self {
        ConditionBuilder {
            operator: ConditionOperator {
                quantifier: None,
                operator: GlobalConditionOperator::NumericEquals,
                only_if_exists: false,
            },
            rhs: Default::default(),
        }
    }

    pub fn new_numeric_not_equals() -> Self {
        ConditionBuilder {
            operator: ConditionOperator {
                quantifier: None,
                operator: GlobalConditionOperator::NumericNotEquals,
                only_if_exists: false,
            },
            rhs: Default::default(),
        }
    }

    pub fn new_bool() -> Self {
        ConditionBuilder {
            operator: ConditionOperator {
                quantifier: None,
                operator: GlobalConditionOperator::Bool,
                only_if_exists: false,
            },
            rhs: Default::default(),
        }
    }

    pub fn for_all(&mut self) -> &mut Self {
        self.operator.quantifier = Some(ConditionOperatorQuantifier::ForAllValues);
        self
    }

    pub fn for_any(&mut self) -> &mut Self {
        self.operator.quantifier = Some(ConditionOperatorQuantifier::ForAnyValue);
        self
    }

    pub fn right_hand_side(&mut self, key: &str, values: &mut Vec<ConditionValue>) -> &mut Self {
        let values = match values.len() {
            0 => panic!("you must specify at least one value"),
            1 => OneOrAll::One(values.remove(0)),
            _ => OneOrAll::All(values.drain(0..).collect()),
        };
        self.rhs.insert(key.parse().unwrap(), values);
        self
    }

    pub fn right_hand_str(&mut self, key: &str, value: &str) -> &mut Self {
        self.rhs.insert(
            key.parse().unwrap(),
            OneOrAll::One(ConditionValue::String(value.to_string())),
        );
        self
    }

    pub fn right_hand_int(&mut self, key: &str, value: i64) -> &mut Self {
        self.rhs.insert(
            key.parse().unwrap(),
            OneOrAll::One(ConditionValue::Integer(value)),
        );
        self
    }

    pub fn right_hand_float(&mut self, key: &str, value: f64) -> &mut Self {
        self.rhs.insert(
            key.parse().unwrap(),
            OneOrAll::One(ConditionValue::Float(value)),
        );
        self
    }

    pub fn right_hand_bool(&mut self, key: &str, value: bool) -> &mut Self {
        self.rhs.insert(
            key.parse().unwrap(),
            OneOrAll::One(ConditionValue::Bool(value)),
        );
        self
    }

    pub fn if_exists(&mut self) -> &mut Self {
        self.operator.only_if_exists = true;
        self
    }
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn this(v: &str) -> OneOrAny {
    OneOrAny::One(v.to_string())
}

pub fn any_of(values: Vec<&str>) -> OneOrAny {
    OneOrAny::AnyOf(values.iter().map(|s| s.to_string()).collect())
}

pub fn condition_one(
    condition: &mut HashMap<ConditionOperator, HashMap<QString, OneOrAll<ConditionValue>>>,
    c_oper: ConditionOperator,
    key: QString,
    value: String,
) -> &mut HashMap<ConditionOperator, HashMap<QString, OneOrAll<ConditionValue>>> {
    let entry: HashMap<QString, OneOrAll<ConditionValue>> =
        vec![(key, OneOrAll::One(ConditionValue::String(value)))]
            .iter()
            .cloned()
            .collect();
    condition.insert(c_oper, entry);
    condition
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

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
        println!("{}", policy);
    }
}
