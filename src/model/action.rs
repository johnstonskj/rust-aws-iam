/*!
One-line description.
More detailed description, with
# Example
 */

use std::str::FromStr;

use crate::error::{missing_property, type_mismatch, unexpected_properties, IamFormatError};
use crate::model::{MaybeAny, OrAny, QualifiedName};
use crate::syntax::{
    display_vec_to_json, from_json_str, json_type_name, IamProperty, IamValue, ACTION_NAME,
    ACTION_VALUE_ACTION, ACTION_VALUE_NOT_ACTION, JSON_TYPE_NAME_STRING, POLICY_WILDCARD_VALUE,
};
use serde_json::{Map, Value};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The Action element describes the specific action or actions that will be allowed or denied.
/// Statements must include either an Action or NotAction element. Each AWS service has its own
/// set of actions that describe tasks that you can perform with that service.
///
/// You specify a value using a service namespace as an action prefix (`iam`, `ec2`, `sqs`,
/// `sns`, `s3`, etc.) followed by the name of the action to allow or deny. The name must match
/// an action that is supported by the service. The prefix and the action name are case
/// insensitive. For example, `iam:ListAccessKeys` is the same as `IAM:listaccesskeys`.
///
/// From [IAM JSON Policy Elements: Action](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_elements_action.html)
/// and [IAM JSON Policy Elements: NotAction](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_elements_notaction.html).
///
/// ## action_string
///
/// Consists of a service namespace, a colon, and the name of an action. Action
/// names can include wildcards. Examples:
///
/// ```json
/// "Action":"ec2:StartInstances"
///
/// "Action":[
///   "ec2:StartInstances",
///   "ec2:StopInstances"
/// ]
///
/// "Action":"cloudformation:*"
///
/// "Action":"*"
///
/// "Action":[
///   "s3:Get*",
///   "s3:List*"
/// ]
/// ```
///
#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    /// Asserts that the action in the request **must** match one of the specified ones.
    Action(OrAny<Vec<QualifiedName>>),
    /// Asserts that the action in the request **must not** match one of the specified ones.
    NotAction(OrAny<Vec<QualifiedName>>),
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for Action {
    fn default() -> Self {
        Self::Action(OrAny::Any)
    }
}

impl IamProperty for Action {
    fn into_json_object(
        &self,
        object: &mut serde_json::Map<String, Value>,
    ) -> Result<(), IamFormatError> {
        let _ = match &self {
            Self::Action(values) => {
                object.insert(ACTION_VALUE_ACTION.to_string(), values.to_json()?)
            }
            Self::NotAction(values) => {
                object.insert(ACTION_VALUE_NOT_ACTION.to_string(), values.to_json()?)
            }
        };
        Ok(())
    }

    fn from_json_object(value: &Map<String, Value>) -> Result<Self, IamFormatError>
    where
        Self: Sized,
    {
        match (
            value.get(ACTION_VALUE_ACTION),
            value.get(ACTION_VALUE_NOT_ACTION),
        ) {
            (Some(v), None) => Ok(Action::Action(OrAny::<Vec<QualifiedName>>::from_json(v)?)),
            (None, Some(v)) => Ok(Action::NotAction(OrAny::<Vec<QualifiedName>>::from_json(
                v,
            )?)),
            (None, None) => missing_property(ACTION_NAME).into(),
            (Some(_), Some(_)) => unexpected_properties(ACTION_NAME).into(),
        }
    }
}

impl Action {
    pub fn this_action(name: QualifiedName) -> Self {
        Self::Action(OrAny::Some(vec![name]))
    }

    pub fn these_actions(names: Vec<QualifiedName>) -> Self {
        Self::Action(OrAny::Some(names))
    }

    pub fn not_this_action(name: QualifiedName) -> Self {
        Self::NotAction(OrAny::Some(vec![name]))
    }

    pub fn not_these_actions(names: Vec<QualifiedName>) -> Self {
        Self::NotAction(OrAny::Some(names))
    }
}

impl MaybeAny<Vec<QualifiedName>> for Action {
    fn new_any() -> Self
    where
        Self: Sized,
    {
        Self::Action(OrAny::Any)
    }

    fn new_none() -> Self
    where
        Self: Sized,
    {
        Self::NotAction(OrAny::Any)
    }

    fn inner(&self) -> &OrAny<Vec<QualifiedName>> {
        match self {
            Action::Action(v) => v,
            Action::NotAction(v) => v,
        }
    }

    fn is_negative(&self) -> bool {
        matches!(self, Action::NotAction(_))
    }
}

// ------------------------------------------------------------------------------------------------

impl IamValue for OrAny<Vec<QualifiedName>> {
    fn to_json(&self) -> Result<Value, IamFormatError> {
        Ok(if let OrAny::Some(values) = self {
            display_vec_to_json(values)?
        } else {
            Value::String(POLICY_WILDCARD_VALUE.to_string())
        })
    }

    fn from_json(value: &Value) -> Result<Self, IamFormatError>
    where
        Self: Sized,
    {
        if let Value::String(s) = value {
            if s == POLICY_WILDCARD_VALUE {
                Ok(OrAny::Any)
            } else {
                Ok(OrAny::Some(vec![QualifiedName::from_str(s)?]))
            }
        } else if let Value::Array(arr) = value {
            let results: Result<Vec<QualifiedName>, IamFormatError> =
                arr.iter().map(|v| from_json_str(v, ACTION_NAME)).collect();
            Ok(OrAny::Some(results?))
        } else {
            type_mismatch(ACTION_NAME, JSON_TYPE_NAME_STRING, json_type_name(value)).into()
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::model::{Action, MaybeAny, OrAny, QualifiedName};
    use crate::syntax::IamProperty;
    use serde_json::{json, Map, Value};
    use std::str::FromStr;

    #[test]
    fn test_any_action_into_json() {
        let mut statement = Map::default();

        let action = Action::new_any();
        action.into_json_object(&mut statement).unwrap();

        assert_eq!(
            Value::Object(statement),
            json!({
                "Action": "*"
            })
        );
    }

    #[test]
    fn test_this_action_into_json() {
        let mut statement = Map::default();

        let action = Action::this_action(QualifiedName::from_str("s3:Get*").unwrap());
        action.into_json_object(&mut statement).unwrap();

        assert_eq!(
            Value::Object(statement),
            json!({
                "Action": "s3:Get*"
            })
        );
    }

    #[test]
    fn test_these_actions_into_json() {
        let mut statement = Map::default();

        let action = Action::these_actions(vec![
            QualifiedName::from_str("s3:Get*").unwrap(),
            QualifiedName::from_str("s3:Put*").unwrap(),
        ]);
        action.into_json_object(&mut statement).unwrap();

        assert_eq!(
            Value::Object(statement),
            json!({
                "Action": [
                    "s3:Get*",
                    "s3:Put*"
                ]
            })
        );
    }

    #[test]
    fn test_no_action_into_json() {
        let mut statement = Map::default();

        let action = Action::new_none();
        action.into_json_object(&mut statement).unwrap();

        assert_eq!(
            Value::Object(statement),
            json!({
                "NotAction": "*"
            })
        );
    }

    #[test]
    fn test_not_this_action_into_json() {
        let mut statement = Map::default();

        let action = Action::not_this_action(QualifiedName::from_str("s3:Get*").unwrap());
        action.into_json_object(&mut statement).unwrap();

        assert_eq!(
            Value::Object(statement),
            json!({
                "NotAction": "s3:Get*"
            })
        );
    }

    #[test]
    fn test_not_these_actions_into_json() {
        let mut statement = Map::default();

        let action = Action::not_these_actions(vec![
            QualifiedName::from_str("s3:Get*").unwrap(),
            QualifiedName::from_str("s3:Put*").unwrap(),
        ]);
        action.into_json_object(&mut statement).unwrap();

        assert_eq!(
            Value::Object(statement),
            json!({
                "NotAction": [
                    "s3:Get*",
                    "s3:Put*"
                ]
            })
        );
    }

    #[test]
    fn test_wildcard_from_json() {
        let action = Value::String("*".to_string());
        let mut container = Map::default();
        container.insert("Action".to_string(), action);

        let result = Action::from_json_object(&container).unwrap();

        assert_eq!(result, Action::Action(OrAny::Any));
    }

    #[test]
    fn test_not_wildcard_from_json() {
        let action = Value::String("*".to_string());
        let mut container = Map::default();
        container.insert("NotAction".to_string(), action);

        let result = Action::from_json_object(&container).unwrap();

        assert_eq!(result, Action::NotAction(OrAny::Any));
    }

    #[test]
    #[should_panic]
    fn test_from_json_missing() {
        let value = Map::default();
        Action::from_json_object(&value).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_from_json_both_keys() {
        let action = Value::String("*".to_string());
        let mut container = Map::default();
        container.insert("Action".to_string(), action.clone());
        container.insert("NotAction".to_string(), action);

        Action::from_json_object(&container).unwrap();
    }

    #[test]
    fn test_one_name_from_json() {
        let action = Value::String("ec2:StartInstances".to_string());
        let mut container = Map::default();
        container.insert("Action".to_string(), action);

        let result = Action::from_json_object(&container).unwrap();

        assert_eq!(
            result,
            Action::Action(OrAny::Some(vec![QualifiedName::from_str(
                "ec2:StartInstances"
            )
            .unwrap()]))
        );
    }

    #[test]
    fn test_name_vec_from_json() {
        let action_1 = Value::String("ec2:StartInstances".to_string());
        let action_2 = Value::String("ec2:StopInstances".to_string());
        let mut container = Map::default();
        container.insert("Action".to_string(), Value::Array(vec![action_1, action_2]));

        let result = Action::from_json_object(&container).unwrap();

        assert_eq!(
            result,
            Action::Action(OrAny::Some(vec![
                QualifiedName::from_str("ec2:StartInstances").unwrap(),
                QualifiedName::from_str("ec2:StopInstances").unwrap()
            ]))
        );
    }
}
