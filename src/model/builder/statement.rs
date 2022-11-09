use super::{ActionBuilder, ConditionBuilder, PrincipalBuilder, ResourceBuilder};
use crate::model::{Effect, Statement};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// A `Statement` builder, used with `PolicyBuilder::evaluate_statement()`.
///
#[derive(Clone, Debug)]
pub struct StatementBuilder {
    sid: Option<String>,
    effect: Effect,

    principals: Option<PrincipalBuilder>,

    actions: ActionBuilder,

    resources: ResourceBuilder,

    condition: ConditionBuilder,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for StatementBuilder {
    fn default() -> Self {
        StatementBuilder {
            sid: None,
            effect: Effect::Deny,
            principals: Default::default(),
            actions: Default::default(),
            resources: Default::default(),
            condition: Default::default(),
        }
    }
}

impl From<StatementBuilder> for Statement {
    fn from(builder: StatementBuilder) -> Self {
        Statement {
            sid: builder.sid.clone(),
            principal: builder.principals.map(|builder| builder.into()),
            effect: builder.effect.clone(),
            action: builder.actions.into(),
            resource: builder.resources.into(),
            condition: builder.condition.clone(),
        }
    }
}

impl StatementBuilder {
    /// Create a new, empty, statement builder
    pub fn new() -> Self {
        Default::default()
    }

    /// Set the id of this statement
    pub fn named(self, sid: &str) -> Self {
        self.sid = Some(sid.to_string());
        self
    }

    /// Set the id of this statement to a randomly generate value.
    pub fn auto_name(self) -> Self {
        self.sid = Some(random_id());
        self
    }

    /// Set the effect of this statement to `Allow`.
    pub fn allows(self) -> Self {
        self.effect = Effect::Allow;
        self
    }

    /// Set the effect of this statement to `Deny`.
    pub fn does_not_allow(self) -> Self {
        self.effect = Effect::Deny;
        self
    }

    pub fn principals(self, principals: PrincipalBuilder) -> Self {
        self.principals = Some(principals);
        self
    }

    pub fn actions(self, actions: ActionBuilder) -> Self {
        self.actions = actions;
        self
    }

    pub fn resources(self, resources: ResourceBuilder) -> Self {
        self.resources = resources;
        self
    }

    /// Adds this condition to the statement.
    pub fn if_condition(self, condition: ConditionBuilder) -> Self {
        self.condition = condition;
        self
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

#[inline]
fn random_id() -> String {
    let id = uuid::Uuid::new_v4();
    id.to_string()
}
