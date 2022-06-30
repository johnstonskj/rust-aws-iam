/*!
One-line description.
More detailed description, with
# Example
 */

use std::convert::TryFrom;

use crate::error::{empty_vector_property, unexpected_value_for_type, IamFormatError};
use crate::model::{Statement, Version};
use crate::syntax::{
    display_to_json, json_type_name, IamValue, ID_NAME, JSON_TYPE_NAME_ARRAY,
    JSON_TYPE_NAME_OBJECT, JSON_TYPE_NAME_STRING, POLICY_NAME, STATEMENT_NAME, VERSION_NAME,
};
use serde_json::{Map, Value};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// An IAM policy resource.
///
/// ## policy_id_string
///
/// Provides a way to include information about the policy as a whole. Some
/// services, such as Amazon SQS and Amazon SNS, use the Id element in
/// reserved ways. Unless otherwise restricted by an individual service,
/// policy_id_string can include spaces. Some services require this value to
/// be unique within an AWS account.
///
/// > The id_block is allowed in resource-based policies, but not in identity-based policies.
///
/// There is no limit to the length, although this string contributes to the
/// overall length of the policy, which is limited.
///
/// ```text
/// "Id":"Admin_Policy"
/// "Id":"cd3ad3d9-2776-4ef1-a904-4c229d1642ee"
/// ```
///
#[derive(Debug, Clone, PartialEq)]
pub struct Policy {
    /// The IAM version of the policy grammar used in this resource
    pub version: Option<Version>,
    /// The identifier of this policy, if any
    pub id: Option<String>,
    /// One or more policy statements
    pub statement: Vec<Statement>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl From<Statement> for Policy {
    fn from(st: Statement) -> Self {
        Policy::unnamed(vec![st]).unwrap()
    }
}

impl TryFrom<Vec<Statement>> for Policy {
    type Error = IamFormatError;

    fn try_from(sts: Vec<Statement>) -> Result<Self, Self::Error> {
        Policy::unnamed(sts)
    }
}

impl IamValue for Policy {
    fn to_json(&self) -> Result<Value, IamFormatError> {
        let mut policy: Map<String, Value> = Default::default();
        if let Some(version) = &self.version {
            let _ = policy.insert(VERSION_NAME.to_string(), display_to_json(version));
        }
        if let Some(id) = &self.id {
            let _ = policy.insert(ID_NAME.to_string(), display_to_json(id));
        }
        let _ = policy.insert(
            STATEMENT_NAME.to_string(),
            Value::Array(
                self.statement
                    .iter()
                    .map(|st| st.to_json().unwrap())
                    .collect(),
            ),
        );
        Ok(Value::Object(policy))
    }

    fn from_json(value: &Value) -> Result<Self, IamFormatError> {
        let mut policy = Policy {
            version: None,
            id: None,
            statement: Default::default(),
        };
        let mut count = 0;

        if let Value::Object(object) = value {
            if let Some(version) = object.get(VERSION_NAME) {
                policy.version = Some(Version::from_json(version)?);
                count += 1;
            }
            if let Some(id) = object.get(ID_NAME) {
                if let Value::String(id) = id {
                    policy.id = Some(id.to_string());
                } else {
                    return Err(IamFormatError::TypeMismatch {
                        name: ID_NAME.to_string(),
                        expecting: JSON_TYPE_NAME_STRING.to_string(),
                        found: json_type_name(value),
                    });
                }
                count += 1;
            }
            if let Some(statement) = object.get(STATEMENT_NAME) {
                if let Value::Array(statement) = statement {
                    let statements: Result<Vec<Statement>, IamFormatError> =
                        statement.iter().map(Statement::from_json).collect();
                    policy.statement = statements?;
                } else {
                    return Err(IamFormatError::TypeMismatch {
                        name: STATEMENT_NAME.to_string(),
                        expecting: JSON_TYPE_NAME_ARRAY.to_string(),
                        found: json_type_name(value),
                    });
                }
                count += 1;
            }
            if object.len() != count {
                Err(IamFormatError::UnexpectedProperties {
                    type_name: POLICY_NAME.to_string(),
                })
            } else {
                Ok(policy)
            }
        } else {
            Err(IamFormatError::TypeMismatch {
                name: POLICY_NAME.to_string(),
                expecting: JSON_TYPE_NAME_OBJECT.to_string(),
                found: json_type_name(value),
            })
        }
    }
}

impl Policy {
    pub fn unnamed(statements: Vec<Statement>) -> Result<Self, IamFormatError> {
        if statements.is_empty() {
            empty_vector_property(STATEMENT_NAME).into()
        } else {
            Ok(Self {
                version: Default::default(),
                id: Default::default(),
                statement: statements,
            })
        }
    }

    pub fn named(id: &str, statements: Vec<Statement>) -> Result<Self, IamFormatError> {
        if !Self::is_valid_external_id(id) {
            unexpected_value_for_type(ID_NAME, id).into()
        } else if statements.is_empty() {
            empty_vector_property(STATEMENT_NAME).into()
        } else {
            Ok(Self {
                version: Default::default(),
                id: Some(id.to_string()),
                statement: statements,
            })
        }
    }

    pub fn for_version(mut self, version: Version) -> Self {
        self.version = Some(version);
        self
    }

    pub fn version(&self) -> Option<Version> {
        self.version
    }

    pub fn id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    pub fn statements(&self) -> impl Iterator<Item = &Statement> {
        self.statement.iter()
    }

    pub fn into_iterator(self) -> impl Iterator<Item = Statement> {
        self.statement.into_iter()
    }

    // https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_iam-quotas.html
    // The external ID value that a third party uses to assume a role must
    // have a minimum of 2 characters and a maximum of 1,224 characters. The
    // value must be alphanumeric without white space. It can also include the
    // following symbols: plus (+), equal (=), comma (,), period (.), at (@),
    // colon (:), forward slash (/), and hyphen (-). For more information
    // about the external ID, see How to use an external ID when granting
    // access to your AWS resources to a third party.
    pub fn is_valid_external_id(s: &str) -> bool {
        s.len() >= 2
            && s.len() <= 1224
            && s.chars().any(|c| {
                c.is_ascii_alphanumeric() || ['+', '=', ',', '.', '@', ':', '/', '-'].contains(&c)
            })
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
    use serde_json::json;

    use crate::model::{Policy, Statement, Version};
    use crate::syntax::IamValue;

    #[test]
    fn test_simple_policy_to_json() {
        let policy = Policy::unnamed(vec![Statement::unnamed()]).unwrap();
        let object = policy.to_json().unwrap();

        assert_eq!(
            object,
            json!({
              "Statement": [
                {
                  "Action": "*",
                  "Effect": "Deny",
                  "Resource": "*"
                }
              ]
            })
        );
    }

    #[test]
    fn test_named_policy_to_json() {
        let policy = Policy::named("SomePolicyName", vec![Statement::unnamed()])
            .unwrap()
            .for_version(Version::V2012);
        let object = policy.to_json().unwrap();

        assert_eq!(
            object,
            json!({
              "Id": "SomePolicyName",
              "Statement": [
                {
                  "Action": "*",
                  "Effect": "Deny",
                  "Resource": "*"
                }
              ],
              "Version": "2012-10-17"
            })
        );
    }

    #[test]
    fn test_example_policy_from_json() {
        let json = json!({
        "Version": "2012-10-17",
        "Statement": [
          {
            "Sid": "UsePrincipalArnInsteadOfNotPrincipalWithDeny",
            "Effect": "Deny",
            "Action": "s3:*",
            "Principal": "*",
            "Resource": [
              "arn:aws:s3:::BUCKETNAME/*",
              "arn:aws:s3:::BUCKETNAME"
            ],
            "Condition": {
              "ArnNotEquals": {
                "aws:PrincipalArn": "arn:aws:iam::444455556666:user/user-name"
              }
            }
          }
        ]
              });

        let policy = Policy::from_json(&json);

        println!("{:#?}", policy);
    }
}
