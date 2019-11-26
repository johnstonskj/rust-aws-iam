use crate::model::{ConditionValue, PrincipalType, QString};
use crate::offline::EvaluationError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;
use uuid::Uuid;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Alias for the hash used to store environment values.
///
pub type Environment = HashMap<QString, ConditionValue>;

///
/// This struct represents a request and it's environment against which a policy, or policies,
/// will be evaluated.
///
#[derive(Debug, Deserialize, Serialize)]
pub struct Request {
    /// An optional request identifier.
    pub request_id: Option<String>,
    /// The principal making the request.
    pub principal: Option<Principal>,
    /// The action being requested.
    pub action: QString,
    /// The resource to which the action is applied.
    pub resource: String,
    /// Additional properties which may be used in conditions.
    pub environment: Environment,
}

///
/// A structure representing a single principal.
///
#[derive(Debug, Deserialize, Serialize)]
pub struct Principal {
    /// The principal type used in Policy documents.
    pub principal_type: PrincipalType,
    /// The corresponding principal ID.
    pub identifier: String,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Request {
    /// Return the value of an environment variable.
    pub fn get(&self, key: &QString) -> Result<&ConditionValue, EvaluationError> {
        match self.environment.get(key) {
            Some(v) => Ok(v),
            None => Err(EvaluationError::UnknownVariableName(key.to_string())),
        }
    }

    /// Return the value of an environment variable.
    pub fn get_(&self, key: &str) -> Result<&ConditionValue, EvaluationError> {
        let key = QString::from_str(key)
            .map_err(|_| EvaluationError::InvalidVariableName(key.to_string()))?;
        match self.environment.get(&key) {
            Some(v) => Ok(v),
            None => Err(EvaluationError::UnknownVariableName(key.to_string())),
        }
    }

    /// Return the request_id within the request or generate one if it is `None`.
    pub fn request_id_or_default(&self) -> String {
        match &self.request_id {
            Some(id) => id.to_string(),
            None => Uuid::new_v4()
                .to_hyphenated()
                .encode_lower(&mut Uuid::encode_buffer())
                .to_string(),
        }
    }
}
