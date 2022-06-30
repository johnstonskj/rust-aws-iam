/*!
One-line description.

More detailed description, with

# Example

 */

use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::str::FromStr;

use crate::error::{unexpected_value_for_type, IamFormatError};
use crate::syntax::{QUALIFIED_NAME_NAME, SERVICE_NAME_NAME, USER_ID_NAME};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct QualifiedName(String);

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ServiceName(String);

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CanonicalUserId(String);

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

const CHAR_PERIOD_DELIM: char = '.';

// ------------------------------------------------------------------------------------------------

lazy_static! {
    // Note that the published/common version of this allows 0..n dots where we
    // want 1..n; we replaced the '*' with '+' here:
    // ----------------------------------------------------v
    static ref HOST_NAME_SYNTAX: Regex = Regex::new(
        r"^(([a-zA-Z]|[a-zA-Z][a-zA-Z0-9\-]*[a-zA-Z0-9])\.)+([A-Za-z]|[A-Za-z][A-Za-z0-9\-]*[A-Za-z0-9])$")
        .unwrap();

    static ref USER_ID_SYNTAX: Regex = Regex::new(
        r"^[a-z0-9]{64}$")
        .unwrap();

    static ref HOST_PART_SYNTAX: Regex = Regex::new(
        r"^[a-zA-Z]|[a-zA-Z][a-zA-Z0-9\-]*[a-zA-Z0-9]$")
        .unwrap();

    static ref QNAME_SYNTAX: Regex = Regex::new(
        r"^([a-zA-Z][a-zA-Z0-9\-]*):([a-zA-Z?*][a-zA-Z0-9\-?*]*)(/([a-zA-Z?*][a-zA-Z0-9\-?*]*)?)?$")
        .unwrap();
}

const AWS_SERVICE_TAIL: &str = "amazonaws.com";

// ------------------------------------------------------------------------------------------------

impl Display for QualifiedName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<QualifiedName> for String {
    fn from(v: QualifiedName) -> Self {
        v.0
    }
}

impl Deref for QualifiedName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for QualifiedName {
    type Err = IamFormatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if Self::is_valid(s) {
            Ok(Self(s.to_string()))
        } else {
            unexpected_value_for_type(QUALIFIED_NAME_NAME, s).into()
        }
    }
}

impl QualifiedName {
    pub fn namespace(&self) -> &str {
        let (name, _, _, _) = self.split();
        name
    }

    pub fn name(&self) -> Option<&str> {
        let (_, name, _, _) = self.split();
        name
    }

    pub fn tag_name(&self) -> Option<&str> {
        let (_, _, name, _) = self.split();
        name
    }

    pub fn is_wildcard(&self) -> bool {
        let (_, _, _, wild) = self.split();
        wild
    }

    pub fn is_valid(s: &str) -> bool {
        QNAME_SYNTAX.is_match(s)
    }

    fn split(&self) -> (&str, Option<&str>, Option<&str>, bool) {
        let groups = QNAME_SYNTAX.captures(&self.0).unwrap();
        (
            groups.get(1).unwrap().as_str(),
            groups.get(4).map(|s| s.as_str()),
            groups.get(6).map(|s| s.as_str()),
            groups.get(7).is_some(),
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for ServiceName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<ServiceName> for String {
    fn from(v: ServiceName) -> Self {
        v.0
    }
}

impl Deref for ServiceName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for ServiceName {
    type Err = IamFormatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if Self::is_valid(s) {
            Ok(Self(s.to_string()))
        } else {
            unexpected_value_for_type(SERVICE_NAME_NAME, s).into()
        }
    }
}

impl ServiceName {
    pub fn new_unchecked(s: &str) -> Self {
        Self(s.to_string())
    }

    pub fn new_service_name(s: &str) -> Result<Self, IamFormatError> {
        if HOST_PART_SYNTAX.is_match(s) {
            Ok(Self(format!(
                "{}{}{}",
                s, CHAR_PERIOD_DELIM, AWS_SERVICE_TAIL
            )))
        } else {
            unexpected_value_for_type(SERVICE_NAME_NAME, s).into()
        }
    }

    pub fn is_valid(s: &str) -> bool {
        HOST_NAME_SYNTAX.is_match(s)
    }

    pub fn head(&self) -> &str {
        self.0.split_once(CHAR_PERIOD_DELIM).unwrap().0
    }

    pub fn tail(&self) -> &str {
        self.0.split_once(CHAR_PERIOD_DELIM).unwrap().1
    }

    pub fn is_aws_service_name(&self) -> bool {
        self.tail() == AWS_SERVICE_TAIL
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for CanonicalUserId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<CanonicalUserId> for String {
    fn from(v: CanonicalUserId) -> Self {
        v.0
    }
}

impl Deref for CanonicalUserId {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for CanonicalUserId {
    type Err = IamFormatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if Self::is_valid(s) {
            Ok(Self(s.to_string()))
        } else {
            unexpected_value_for_type(USER_ID_NAME, s).into()
        }
    }
}

impl CanonicalUserId {
    pub fn new_unchecked(s: &str) -> Self {
        Self(s.to_string())
    }

    pub fn is_valid(s: &str) -> bool {
        USER_ID_SYNTAX.is_match(s)
    }
}
