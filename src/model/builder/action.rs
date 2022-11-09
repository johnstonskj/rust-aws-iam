use crate::model::{Action, OrAny, QualifiedName};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// A `Action` builder, used with `StatementBuilder::action()`.
///
#[derive(Clone, Debug)]
pub struct ActionBuilder {
    not_action: bool,
    actions: OrAny<Vec<QualifiedName>>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for ActionBuilder {
    fn default() -> Self {
        Self {
            not_action: false,
            actions: OrAny::Any,
        }
    }
}

impl From<ActionBuilder> for Action {
    fn from(builder: ActionBuilder) -> Self {
        if builder.not_action {
            Action::NotAction(builder.actions)
        } else {
            Action::Action(builder.actions)
        }
    }
}

impl ActionBuilder {
    pub fn any() -> Self {
        Self {
            not_action: false,
            actions: OrAny::Any,
        }
    }

    pub fn none() -> Self {
        Self {
            not_action: true,
            actions: OrAny::Any,
        }
    }

    pub fn any_of() -> Self {
        Self {
            not_action: false,
            actions: OrAny::Any,
        }
    }

    pub fn none_of() -> Self {
        Self {
            not_action: true,
            actions: OrAny::Any,
        }
    }

    /// Sets the action of this statement to be only this value.
    pub fn this(self, action: QualifiedName) -> Self {
        self.these(vec![action]);
        self
    }

    /// Sets the action of this statement to be any of these values.
    pub fn these(self, actions: Vec<QualifiedName>) -> Self {
        if let OrAny::Some(action_vec) = self.actions {
            action_vec.extend(actions);
        }
        self
    }
}
