use crate::model::{ConditionValue, QString};
use crate::offline::request::Environment;
use crate::offline::EvaluationError;
use regex::Regex;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// Expand an input string based on any embedded variables of the form `${QString}`. Values
/// are determined from the environment properties of the request object.
///
pub fn expand_string(
    environment: &Environment,
    input_string: &str,
) -> Result<String, EvaluationError> {
    lazy_static! {
        static ref VAR: Regex = Regex::new(r"(\$\{[^\}]+\})").unwrap();
    }
    let mut output: String = String::new();
    let mut from_idx: usize = 0;
    for cap in VAR.captures_iter(input_string) {
        let variable = cap.get(0).unwrap();
        if variable.start() > from_idx {
            // copy over an preceding text
            output.push_str(&input_string[from_idx..variable.start()]);
        }
        let key: &str = variable.as_str();
        let key = &key[2..key.len() - 1];
        let key = QString::from_str(key)
            .map_err(|_| EvaluationError::InvalidVariableName(key.to_string()))?;
        match environment.get(&key) {
            Some(ConditionValue::String(v)) => output.push_str(v),
            None => return Err(EvaluationError::UnknownVariableName(key.to_string())),
            _ => return Err(EvaluationError::ExpectingVariableType("String".to_string())),
        };
        from_idx = variable.end();
    }
    if from_idx < input_string.len() {
        output.push_str(&input_string[from_idx..]);
    }
    Ok(output)
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::constants;
    use crate::model::{ConditionValue, QString};
    use crate::offline::request::Environment;
    use crate::offline::variables::expand_string;
    use crate::offline::EvaluationError;
    use std::str::FromStr;

    fn make_environment() -> Environment {
        let environment: Environment = [
            (
                QString::from_str(constants::AWS_EPOCH_TIME).unwrap(),
                ConditionValue::Integer(1000),
            ),
            (
                QString::from_str(constants::AWS_REQUESTED_REGION).unwrap(),
                ConditionValue::String("us-east-1".to_string()),
            ),
            (
                QString::from_str(constants::AWS_SECURE_TRANSPORT).unwrap(),
                ConditionValue::Bool(true),
            ),
        ]
        .iter()
        .cloned()
        .collect();
        environment
    }

    #[test]
    fn test_no_variables() {
        let test_str = "arn:aws:s3:::src_bucket";
        assert_eq!(
            expand_string(&make_environment(), test_str).unwrap(),
            test_str.to_string()
        );
    }

    #[test]
    fn test_variable_replaced_in() {
        let test_str = "arn:aws:sqs:${aws:RequestedRegion}:444455556666:my-queue";
        assert_eq!(
            expand_string(&make_environment(), test_str).unwrap(),
            "arn:aws:sqs:us-east-1:444455556666:my-queue"
        );
    }

    #[test]
    fn test_variable_replaced_all() {
        let test_str = "${aws:RequestedRegion}";
        assert_eq!(
            expand_string(&make_environment(), test_str).unwrap(),
            "us-east-1"
        );
    }

    #[test]
    fn test_unknown_variable() {
        let test_str = "${aws:FooBarRegion}";
        assert_eq!(
            expand_string(&make_environment(), test_str),
            Err(EvaluationError::UnknownVariableName(
                "aws:FooBarRegion".to_string()
            ))
        );
    }

    #[test]
    fn test_bad_variable_name() {
        let test_str = "${aws:Foo Bar Region}";
        assert_eq!(
            expand_string(&make_environment(), test_str),
            Err(EvaluationError::InvalidVariableName(
                "aws:Foo Bar Region".to_string()
            ))
        );
    }

    #[test]
    fn test_bad_variable_type() {
        let test_str = "${aws:SecureTransport}";
        assert_eq!(
            expand_string(&make_environment(), test_str),
            Err(EvaluationError::ExpectingVariableType("String".to_string()))
        );
    }
}
