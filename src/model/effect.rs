/*!
One-line description.
More detailed description, with
# Example
 */

use crate::error::{unexpected_value_for_property, IamFormatError};
use crate::syntax::{from_json_str, IamValue, EFFECT_NAME, EFFECT_VALUE_ALLOW, EFFECT_VALUE_DENY};
use serde_json::Value;
use std::fmt::Display;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The Effect element is required and specifies whether the statement results in an allow or an
/// explicit deny. Valid values for Effect are Allow and Deny.
///
/// From [IAM JSON Policy Elements: Effect](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_elements_effect.html).
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Effect {
    /// The result of successful evaluation of this policy is to allow access.
    Allow,
    /// The result of successful evaluation of this policy is to deny access.
    Deny,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for Effect {
    fn default() -> Self {
        Self::Deny
    }
}

impl Display for Effect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Effect::Allow => EFFECT_VALUE_ALLOW,
                Effect::Deny => EFFECT_VALUE_DENY,
            }
        )
    }
}

impl FromStr for Effect {
    type Err = IamFormatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            EFFECT_VALUE_ALLOW => Ok(Self::Allow),
            EFFECT_VALUE_DENY => Ok(Self::Deny),
            _ => unexpected_value_for_property(EFFECT_NAME, s).into(),
        }
    }
}

impl IamValue for Effect {
    fn to_json(&self) -> Result<Value, IamFormatError> {
        Ok(Value::String(self.to_string()))
    }

    fn from_json(value: &Value) -> Result<Self, IamFormatError> {
        from_json_str(value, EFFECT_NAME)
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
    use super::*;

    #[test]
    fn test_effect_display() {
        assert_eq!(Effect::Allow.to_string(), "Allow".to_string());
        assert_eq!(Effect::Deny.to_string(), "Deny".to_string());
    }

    #[test]
    fn test_effect_from_str_ok() {
        assert_eq!(Effect::from_str("Allow").unwrap(), Effect::Allow);
        assert_eq!(Effect::from_str("Deny").unwrap(), Effect::Deny);
    }

    #[test]
    fn test_effect_from_str_err() {
        if let Err(e) = Effect::from_str("allow") {
            assert_eq!(
                e.to_string(),
                "An unexpected value `allow` for property named `Effect` was found".to_string()
            );
        } else {
            panic!("should have failed");
        }
    }

    #[test]
    fn test_effect_to_json() {
        assert_eq!(
            Effect::Allow.to_json().unwrap(),
            Value::String("Allow".to_string())
        );
        assert_eq!(
            Effect::Deny.to_json().unwrap(),
            Value::String("Deny".to_string())
        );
    }
}
