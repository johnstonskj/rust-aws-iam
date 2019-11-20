use crate::model::ConditionValue;
use crate::offline::{Context, EvaluationError};
use regex::Regex;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn expand_string(context: &Context, value: &str) -> Result<String, EvaluationError> {
    lazy_static! {
        static ref VAR: Regex = Regex::new(r"(\$\{\})").unwrap();
    }
    let mut output: String = String::new();
    let mut from_idx: usize = 0;
    for cap in VAR.captures_iter(value) {
        let variable = cap.get(0).unwrap();
        if variable.start() > from_idx {
            // copy over an preceding text
            output.push_str(&value[from_idx..variable.start()]);
        }
        let key: &str = variable.as_str();
        let key = &key[2..key.len() - 1];
        match context.get_(&key)? {
            ConditionValue::String(v) => output.push_str(v),
            _ => return Err(EvaluationError::ExpectingVariableType("String".to_string())),
        };
        from_idx = variable.end();
    }
    if from_idx < value.len() {
        output.push_str(&value[from_idx..]);
    }
    Ok(output)
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn make_context() -> Context {
        Context {
            principal: None,
            action: "s3:ListBucket".parse().unwrap(),
            resource: "".to_string(),
            map: Default::default(),
        }
    }

    #[test]
    fn test_no_variables() {
        let test_str = "hello world";
        assert_eq!(
            expand_string(&make_context(), test_str).unwrap(),
            test_str.to_string()
        );
    }
}
