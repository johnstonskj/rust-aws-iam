/*!
Provides the ability to configure service-specific rules for validation. Requires feature
`service_config`.

Details TBD.
 */

use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct ServiceConfig {
    pub namespace: Namespace,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<QualifiedName>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub resource_types: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub condition_keys: Vec<ConditionKey>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[allow(missing_docs)]
pub enum ConditionKeyType {
    String,
    Number,
    Boolean,
    Date,
    Binary,
    ResourceName,
    IpAddress,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct ConditionKey {
    name: QualifiedName,
    key_type: ConditionKeyType,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl ServiceConfig {
    pub fn new(name: QualifiedName) -> Self {
        Self {
            name,
            key_type: ConditionKeyType::default(),
        }
    }

    pub fn namespace(&self) -> &Namespace {
        &self.namespace
    }

    pub fn actions(&self) -> impl Iterator<Item = QualifiedName> {
        &self.actions.iter()
    }

    pub fn resource_types(&self) -> impl Iterator<Item = String> {
        &self.resource_types.iter()
    }

    pub fn condition_keys(&self) -> impl Iterator<Item = ConditionKey> {
        &self.condition_keys.iter()
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for ConditionKeyType {}

impl Default for ConditionKeyType {}

impl FromStr for ConditionKeyType {}

impl ConditionKeyType {}

// ------------------------------------------------------------------------------------------------

impl ConditionKey {
    pub fn new(name: QualifiedName) -> Self {
        Self {
            name,
            key_type: ConditionKeyType::default(),
        }
    }

    pub fn number(name: QualifiedName, key_type: ConditionKeyType) -> Self {
        Self { name, key_type }
    }

    pub fn boolean(name: QualifiedName) -> Self {
        Self {
            name,
            key_type: ConditionKeyType::Boolean,
        }
    }

    pub fn date(name: QualifiedName) -> Self {
        Self {
            name,
            key_type: ConditionKeyType::Date,
        }
    }

    pub fn binary(name: QualifiedName) -> Self {
        Self {
            name,
            key_type: ConditionKeyType::Binary,
        }
    }

    pub fn resource_name(name: QualifiedName) -> Self {
        Self {
            name,
            key_type: ConditionKeyType::ResourceName,
        }
    }

    pub fn ip_address(name: QualifiedName) -> Self {
        Self {
            name,
            key_type: ConditionKeyType::IpAddress,
        }
    }

    pub fn name(&self) -> &QualifiedName {
        &self.name
    }

    pub fn key_type(&self) -> ConditionKeyType {
        self.key_type
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
