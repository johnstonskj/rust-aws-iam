/*!
One-line description.
More detailed description, with
# Example
 */

use std::convert::TryFrom;

use super::id;
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
                version: None,
                id: Default::default(),
                statement: statements,
            })
        }
    }

    pub fn named<S>(policy_id: S, statements: Vec<Statement>) -> Result<Self, IamFormatError>
    where
        S: Into<String>,
    {
        if !id::is_valid_external_id(policy_id) {
            unexpected_value_for_type(ID_NAME, policy_id).into()
        } else if statements.is_empty() {
            empty_vector_property(STATEMENT_NAME).into()
        } else {
            Ok(Self {
                version: None,
                id: Some(policy_id.into()),
                statement: statements,
            })
        }
    }

    pub fn unnamed_with_version(
        statements: Vec<Statement>,
        version: Version,
    ) -> Result<Self, IamFormatError> {
        if statements.is_empty() {
            empty_vector_property(STATEMENT_NAME).into()
        } else {
            Ok(Self {
                version: Some(version),
                id: Default::default(),
                statement: statements,
            })
        }
    }

    pub fn named_with_version<S>(
        policy_id: S,
        statements: Vec<Statement>,
        version: Version,
    ) -> Result<Self, IamFormatError>
    where
        S: Into<String>,
    {
        if !id::is_valid_external_id(policy_id) {
            unexpected_value_for_type(ID_NAME, policy_id).into()
        } else if statements.is_empty() {
            empty_vector_property(STATEMENT_NAME).into()
        } else {
            Ok(Self {
                version: Some(version),
                id: Some(policy_id.into()),
                statement: statements,
            })
        }
    }

    // --------------------------------------------------------------------------------------------

    pub fn version(&self) -> Option<Version> {
        self.version
    }

    pub fn set_version(&mut self, version: Version) {
        self.version = Some(version)
    }

    // --------------------------------------------------------------------------------------------

    pub fn id(&self) -> Option<&String> {
        self.id.as_ref()
    }

    pub fn set_id<S>(&mut self, policy_id: S) -> Result<(), IamFormatError>
    where
        S: Into<String>,
    {
        if !id::is_valid_external_id(policy_id) {
            unexpected_value_for_type(ID_NAME, policy_id).into()
        } else {
            self.id = Some(policy_id.into());
            Ok(())
        }
    }

    pub fn unset_id(&mut self) {
        self.id = None
    }

    pub fn set_auto_id(&mut self) {
        self.id = Some(id::new_external_id())
    }

    // --------------------------------------------------------------------------------------------

    pub fn statements(&self) -> impl Iterator<Item = &Statement> {
        self.statement.iter()
    }

    pub fn statements_mut(&mut self) -> impl Iterator<Item = &mut Statement> {
        self.statement.iter_mut()
    }

    pub fn statements_push(&mut self, statement: Statement) {
        self.statement.push(statement)
    }

    pub fn statements_extend(&mut self, statements: Vec<Statement>) {
        self.statement.extend(statements.into_iter())
    }
}
