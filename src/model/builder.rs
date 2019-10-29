use crate::model::*;
use std::collections::HashMap;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn any() -> OneOrAny {
    OneOrAny::Any
}

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
