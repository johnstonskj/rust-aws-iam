/*!
Provides the ability to configure service-specific rules for validation.
*/
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct ServiceConfig {
    pub namespace: String,
    pub actions: Vec<String>,
    pub resource_types: Vec<String>,
    pub condition_keys: Vec<ConditionKey>,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(missing_docs)]
pub enum ConditionKeyType {
    String,
    Numeric,
    Boolean,
    Binary,
    ARN,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct ConditionKey {
    pub name: String,
    pub key_type: ConditionKeyType,
}
