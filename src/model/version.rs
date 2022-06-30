/*!
One-line description.
More detailed description, with
# Example
 */

use crate::error::IamFormatError;
use crate::syntax::{display_to_json, IamValue};
use crate::syntax::{
    json_type_name, JSON_TYPE_NAME_STRING, VERSION_NAME, VERSION_VALUE_2008, VERSION_VALUE_2012,
};
use serde_json::Value;
use std::fmt::Display;
use std::str::FromStr;
// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The Version policy element is used within a policy and defines the version of
/// the policy language.
///
/// If you do not include a Version element, the value defaults to 2008-10-17,
/// but newer features, such as policy variables, will not work with your policy.
/// For example, variables such as ${aws:username} aren't recognized as variables
/// and are instead treated as literal strings in the policy.
///
/// From [IAM JSON Policy Elements: Version](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_elements_version.html).
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Version {
    /// This is the current version of the policy language, and you should always
    /// include a Version element and set it to 2012-10-17. Otherwise, you cannot
    /// use features such as policy variables that were introduced with this version.
    V2012,

    /// This was an earlier version of the policy language. You might see this
    /// version on older existing policies. Do not use this version for any new
    /// policies or when you update any existing policies.
    V2008,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for Version {
    fn default() -> Self {
        Self::V2012
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Version::V2012 => VERSION_VALUE_2012,
                Version::V2008 => VERSION_VALUE_2008,
            }
        )
    }
}

impl FromStr for Version {
    type Err = IamFormatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            VERSION_VALUE_2012 => Ok(Self::V2012),
            VERSION_VALUE_2008 => Ok(Self::V2008),
            _ => Err(IamFormatError::UnexpectedValue {
                name: VERSION_NAME.to_string(),
                value: s.to_string(),
            }),
        }
    }
}

impl IamValue for Version {
    fn to_json(&self) -> Result<Value, IamFormatError> {
        Ok(display_to_json(self))
    }

    fn from_json(value: &Value) -> Result<Self, IamFormatError> {
        if let Value::String(s) = value {
            Ok(Version::from_str(s)?)
        } else {
            Err(IamFormatError::TypeMismatch {
                name: VERSION_NAME.to_string(),
                expecting: JSON_TYPE_NAME_STRING.to_string(),
                found: json_type_name(value),
            })
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
    use super::*;

    #[test]
    fn test_version_display() {
        assert_eq!(Version::V2012.to_string(), "2012-10-17".to_string());
        assert_eq!(Version::V2008.to_string(), "2008-10-17".to_string());
    }

    #[test]
    fn test_version_from_str_ok() {
        assert_eq!(Version::from_str("2012-10-17").unwrap(), Version::V2012);
        assert_eq!(Version::from_str("2008-10-17").unwrap(), Version::V2008);
    }

    #[test]
    fn test_version_from_str_err() {
        if let Err(e) = Version::from_str("2022-06-27") {
            assert_eq!(
                e.to_string(),
                "An unexpected value `2022-06-27` for property named `Version` was found"
                    .to_string()
            );
        } else {
            panic!("should have failed");
        }
    }

    #[test]
    fn test_version_to_json() {
        assert_eq!(
            Version::V2012.to_json().unwrap(),
            Value::String("2012-10-17".to_string())
        );
        assert_eq!(
            Version::V2008.to_json().unwrap(),
            Value::String("2008-10-17".to_string())
        );
    }
}
