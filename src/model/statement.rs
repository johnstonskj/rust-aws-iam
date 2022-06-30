/*!
One-line description.
More detailed description, with
# Example
 */

use crate::error::{missing_property, type_mismatch, IamFormatError};
use crate::model::{Action, Condition, Effect, Principal, Resource};
use crate::syntax::{
    display_to_json, from_json_str, json_type_name, IamProperty, IamValue, EFFECT_NAME,
    JSON_TYPE_NAME_OBJECT, JSON_TYPE_NAME_STRING, SID_NAME, STATEMENT_NAME,
};
use serde_json::{Map, Value};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The Statement element is the main element for a policy. This element is required. It can
/// include multiple elements (see the subsequent sections in this page). The Statement element
/// contains an array of individual statements. Each individual statement is a JSON block
/// enclosed in braces `{ }`.
///
/// From [IAM JSON Policy Elements: Statement](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_elements_statement.html).
///
/// ## sid_string
///
/// Provides a way to include information about an individual
/// statement. For IAM policies, basic alphanumeric characters (A-Z,a-z,0-9)
/// are the only allowed characters in the Sid value. Other AWS services that
/// support resource policies may have other requirements for the Sid value.
/// For example, some services require this value to be unique within an AWS
/// account, and some services allow additional characters such as spaces in
/// the Sid value.
///
/// ```text
/// "Sid": "1"
/// "Sid": "ThisStatementProvidesPermissionsForConsoleAccess"
/// ```
///
#[derive(Debug, Clone, PartialEq)]
pub struct Statement {
    ///

    /// The Sid (statement ID) is an optional identifier that you provide for the policy statement.
    /// You can assign a Sid value to each statement in a statement array. In services that let
    /// you specify an ID element, such as SQS and SNS, the Sid value is just a sub-ID of the
    /// policy document's ID. In IAM, the Sid value must be unique within a JSON policy
    ///
    /// In IAM, the Sid is not exposed in the IAM API. You can't retrieve a particular statement
    /// based on this ID.
    ///
    /// From [IAM JSON Policy Elements: Sid](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_elements_sid.html).
    ///
    pub sid: Option<String>,
    ///
    /// The principals, or not-principals to match as part of this statement.
    ///
    pub principal: Option<Principal>,
    ///
    /// The effect, outcome, if this statement is matched.
    ///
    pub effect: Effect,
    ///
    /// The actions, or not-actions to match as part of this statement.
    ///
    pub action: Action,
    ///
    /// The resources, or not-resources to match as part of this statement.
    ///
    pub resource: Resource,
    ///
    /// Any condition(s) attached to this statement.
    ///
    pub condition: Option<Condition>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl IamValue for Statement {
    fn to_json(&self) -> Result<Value, IamFormatError> {
        let mut statement = Map::default();

        if let Some(sid) = &self.sid {
            statement.insert(SID_NAME.to_string(), display_to_json(sid));
        }

        if let Some(values) = &self.principal {
            values.into_json_object(&mut statement)?;
        }

        statement.insert(EFFECT_NAME.to_string(), self.effect.to_json()?);

        self.action.into_json_object(&mut statement)?;

        self.resource.into_json_object(&mut statement)?;

        if let Some(values) = &self.condition {
            values.into_json_object(&mut statement)?;
        }

        Ok(Value::Object(statement))
    }

    fn from_json(value: &Value) -> Result<Self, IamFormatError>
    where
        Self: Sized,
    {
        if let Value::Object(object) = value {
            let sid: Option<String> = if let Some(value) = object.get(SID_NAME) {
                if let Value::String(s) = value {
                    Some(s.to_string())
                } else {
                    return type_mismatch(SID_NAME, JSON_TYPE_NAME_STRING, json_type_name(value))
                        .into();
                }
            } else {
                None
            };

            let principal: Option<Principal> = Principal::from_json_object_optional(object)?;

            let effect: Effect = if let Some(value) = object.get(EFFECT_NAME) {
                from_json_str(value, EFFECT_NAME)?
            } else {
                return missing_property(EFFECT_NAME).into();
            };

            let action: Action = Action::from_json_object(object)?;

            let resource: Resource = Resource::from_json_object(object)?;

            let condition: Option<Condition> = Condition::from_json_object_optional(object)?;

            Ok(Self {
                sid,
                principal,
                effect,
                action,
                resource,
                condition,
            })
        } else {
            type_mismatch(STATEMENT_NAME, JSON_TYPE_NAME_OBJECT, json_type_name(value)).into()
        }
    }
}

impl Statement {
    pub fn unnamed() -> Self {
        Self {
            sid: Default::default(),
            principal: Default::default(),
            effect: Default::default(),
            action: Default::default(),
            resource: Default::default(),
            condition: Default::default(),
        }
    }

    pub fn named(sid: &str) -> Self {
        Self {
            sid: Some(sid.to_string()),
            principal: Default::default(),
            effect: Default::default(),
            action: Default::default(),
            resource: Default::default(),
            condition: Default::default(),
        }
    }

    pub fn allow(mut self) -> Self {
        self.effect = Effect::Allow;
        self
    }

    pub fn deny(mut self) -> Self {
        self.effect = Effect::Deny;
        self
    }

    pub fn action(mut self, action: Action) -> Self {
        self.action = action;
        self
    }

    pub fn resource(mut self, resource: Resource) -> Self {
        self.resource = resource;
        self
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use crate::model::Statement;
    use crate::syntax::IamValue;

    #[test]
    fn test_simple_statement_to_json() {
        let statement = Statement::unnamed();
        let object = statement.to_json().unwrap();
        let obj_str = format!("{:?}", object);

        assert!(obj_str.starts_with(r##"Object({"##));
        assert!(obj_str.contains(r##""Effect": String("Deny")"##));
        assert!(obj_str.contains(r##""Action": String("*")"##));
        assert!(obj_str.contains(r##""Resource": String("*")"##));
        assert!(obj_str.ends_with(r##"})"##));

        println!("{}", serde_json::to_string_pretty(&object).unwrap());
    }

    #[test]
    fn test_named_statement_to_json() {
        let statement = Statement::named("sid-001");
        let object = statement.to_json().unwrap();
        let obj_str = format!("{:?}", object);

        assert!(obj_str.starts_with(r##"Object({"##));
        assert!(obj_str.contains(r##""Effect": String("Deny")"##));
        assert!(obj_str.contains(r##""Sid": String("sid-001")"##));
        assert!(obj_str.contains(r##""Action": String("*")"##));
        assert!(obj_str.contains(r##""Resource": String("*")"##));
        assert!(obj_str.ends_with(r##"})"##));

        println!("{}", serde_json::to_string_pretty(&object).unwrap());
    }

    #[test]
    fn test_from_json_str() {
        const JSON: &str = r##"{
  "Effect": "Allow",
  "Action": [
    "s3:ListAllMyBuckets",
    "s3:GetBucketLocation"
  ],
  "Resource": "arn:aws:s3:::*"
}"##;
        let value: Value = serde_json::from_str(JSON).unwrap();
        let statement = Statement::from_json(&value).unwrap();
        println!("{:?}", statement);
    }
}
