use crate::model::*;
use std::collections::HashMap;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn any() -> Qualified {
    Qualified::Any
}

pub fn this(v: &str) -> Qualified {
    Qualified::One(v.to_string())
}

pub fn any_of(values: Vec<&str>) -> Qualified {
    Qualified::AnyOf(values.iter().map(|s| s.to_string()).collect())
}

pub fn condition_one(
    condition: &mut HashMap<ConditionType, HashMap<String, ConditionValues>>,
    ctype: ConditionType,
    key: String,
    value: String,
) -> &mut HashMap<ConditionType, HashMap<String, ConditionValues>> {
    let entry: HashMap<String, ConditionValues> =
        vec![(key, ConditionValues::One(ConditionValue::String(value)))]
            .iter()
            .cloned()
            .collect();
    condition.insert(ctype, entry);
    condition
}
