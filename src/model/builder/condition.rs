use crate::model::{
    Condition, ConditionValue, GlobalOperator, Operator, QualifiedName, Quantifier,
};
use std::collections::HashMap;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// A `Condition` builder, used with `StatementBuilder::if_condition()`.
///
#[derive(Clone, Debug)]
pub struct ConditionBuilder {
    operator: Operator,
    matches: HashMap<QualifiedName, Vec<ConditionValue>>,
}

#[derive(Clone, Debug)]
pub struct MatchBuilder {
    condition_key: QualifiedName,
    values: Vec<ConditionValue>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for ConditionBuilder {
    fn default() -> Self {
        Self {
            operator: Default::default(),
            matches: Default::default(),
        }
    }
}

impl From<ConditionBuilder> for Condition {
    fn from(builder: ConditionBuilder) -> Self {
        todo!()
    }
}

impl ConditionBuilder {
    /// Create a new Condition with the provided operator.
    pub fn new(operator: GlobalOperator) -> Self {
        ConditionBuilder {
            operator: Operator {
                quantifier: None,
                operator,
                if_exists: false,
            },
            matches: Default::default(),
        }
    }

    /// Create a new Condition with operator = `StringEquals`
    pub fn new_string_equals() -> Self {
        Self::new(GlobalOperator::StringEquals)
    }

    /// Create a new Condition with operator = `StringNotEquals`
    pub fn new_string_not_equals() -> Self {
        Self::new(GlobalOperator::StringNotEquals)
    }

    /// Create a new Condition with operator = `NumericEquals`
    pub fn new_numeric_equals() -> Self {
        Self::new(GlobalOperator::NumericEquals)
    }

    /// Create a new Condition with operator = `NumericNotEquals`
    pub fn new_numeric_not_equals() -> Self {
        Self::new(GlobalOperator::NumericEquals)
    }

    /// Create a new Condition with operator = `Bool`
    pub fn new_bool() -> Self {
        Self::new(GlobalOperator::Bool)
    }

    /// Add the _for-all-values_ quantifier.
    pub fn for_all(self) -> Self {
        self.operator.quantifier = Some(Quantifier::ForAllValues);
        self
    }

    /// Add the _for-any-value_ quantifier.
    pub fn for_any(self) -> Self {
        self.operator.quantifier = Some(Quantifier::ForAnyValue);
        self
    }

    pub fn match_push(&self, match_value: Match) {
        todo!()
    }
}

// ------------------------------------------------------------------------------------------------

impl MatchBuilder {
    pub fn new(condition_key: QualifiedName, values: Vec<ConditionValue>) -> Self {
        Self {
            condition_key,
            values,
        }
    }

    pub fn aws_called_via(values: Vec<ConditionValue>) -> Self {
        Self::new(condition::aws_called_via(), values)
    }

    pub fn aws_called_via_first(value: ConditionValue) -> Self {
        Self::new(condition::aws_called_via_first(), vec![value])
    }

    pub fn aws_called_via_last(value: ConditionValue) -> Self {
        // type: String
        // single-valued
    }

    pub fn aws_current_time(value: ConditionValue) -> Self {
        // type: Date
        // single-valued
    }

    pub fn aws_epoch_time(value: ConditionValue) -> Self {
        // type: Date or Number
        // single-valued
    }

    pub fn aws_federated_provider(value: ConditionValue) -> Self {
        // type: String
        // single-valued
    }
}
