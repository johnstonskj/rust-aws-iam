/*!
One-line description.

More detailed description, with

# Example

 */

use regex::Regex;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::str::FromStr;

use crate::error::{unexpected_value_for_type, IamFormatError};
use crate::syntax::{
    CHAR_WILD, CHAR_WILD_ALL, HOSTNAME_SEPARATOR, HOST_NAME_NAME, NAMESPACE_NAME,
    NAMESPACE_SEPARATOR, QUALIFIED_NAME_NAME, QUALIFIED_TAG_SEPARATOR, SERVICE_NAME_NAME,
    USER_ID_NAME,
};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Namespace(String);

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct QualifiedName(String);

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ServiceName(String);

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct HostName(String);

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

    static ref SERVICE_NAME_SYNTAX: Regex = Regex::new(
        r"^[a-zA-Z]|[a-zA-Z][a-zA-Z0-9\-]*[a-zA-Z0-9]$")
        .unwrap();

    static ref NAMESPACE_SYNTAX: Regex = Regex::new(
        r"^([a-zA-Z][a-zA-Z0-9\-]*)$")
        .unwrap();

    static ref QNAME_SYNTAX: Regex = Regex::new(
        r"^([a-zA-Z][a-zA-Z0-9\-]*):([a-zA-Z?*][a-zA-Z0-9\-?*]*)(/([a-zA-Z?*][a-zA-Z0-9\-?*]*)?)?$")
        .unwrap();
}

const AWS_SERVICE_TAIL: &str = "amazonaws.com";

// ------------------------------------------------------------------------------------------------

impl Display for Namespace {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Namespace> for String {
    fn from(v: Namespace) -> Self {
        v.0
    }
}

impl Deref for Namespace {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Namespace {
    type Err = IamFormatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if Self::is_valid(s) {
            Ok(Self(s.to_string()))
        } else {
            unexpected_value_for_type(NAMESPACE_NAME, s).into()
        }
    }
}

impl Namespace {
    pub fn new_unchecked<S>(s: S) -> Self
    where
        S: Into<String>,
    {
        Self(s.into())
    }

    pub fn is_valid(s: &str) -> bool {
        NAMESPACE_SYNTAX.is_match(s)
    }

    pub fn to_qualified_name<S>(&self, name: S) -> Result<QualifiedName, IamFormatError>
    where
        S: Into<String>,
    {
        QualifiedName::new(self.to_string(), name)
    }

    pub fn to_service_name(&self) -> ServiceName {
        ServiceName::new_unchecked(self.0)
    }
}

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
    pub fn new_unchecked<S>(s: S) -> Self
    where
        S: Into<String>,
    {
        Self(s.into())
    }

    pub fn new<S1, S2>(namespace: S1, name: S2) -> Result<Self, IamFormatError>
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Self::from_str(&format!(
            "{}{}{}",
            namespace.into(),
            NAMESPACE_SEPARATOR,
            name.into()
        ))
    }

    pub fn new_tagged<S1, S2, S3>(
        namespace: S1,
        name: S2,
        tag_name: S3,
    ) -> Result<Self, IamFormatError>
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<String>,
    {
        let name = name.into();
        let append_slash = !name.ends_with(QUALIFIED_TAG_SEPARATOR);
        Self::from_str(&format!(
            "{}{}{}{}{}",
            namespace.into(),
            NAMESPACE_SEPARATOR,
            name,
            if append_slash {
                QUALIFIED_TAG_SEPARATOR.to_string()
            } else {
                String::new()
            },
            tag_name.into()
        ))
    }

    pub fn with_name<S>(self, name: S) -> Result<Self, IamFormatError>
    where
        S: Into<String>,
    {
        let (namespace, _, tag_name) = self.split();
        let name = name.into();
        match (name.ends_with(QUALIFIED_TAG_SEPARATOR), tag_name) {
            (true, Some(tag_name)) => Self::new_tagged(namespace, name, tag_name),
            _ => Self::new(namespace, name),
        }
    }

    pub fn with_tag<S>(self, tag: S) -> Result<Self, IamFormatError>
    where
        S: Into<String>,
    {
        let (namespace, name, _) = self.split();
        let tag = tag.into();
        if name.ends_with(QUALIFIED_TAG_SEPARATOR) {
            Self::new_tagged(namespace, name, tag)
        } else {
            Self::new(namespace, name)
        }
    }

    pub fn namespace(&self) -> Namespace {
        let (name, _, _) = self.split();
        Namespace::new_unchecked(name)
    }

    pub fn name(&self) -> &str {
        let (_, name, _) = self.split();
        name
    }

    pub fn tag(&self) -> Option<&str> {
        let (_, _, tag) = self.split();
        tag
    }

    pub fn has_wildcard(&self) -> bool {
        self.0.chars().any(|c| c == CHAR_WILD || c == CHAR_WILD_ALL)
    }

    pub fn is_valid(s: &str) -> bool {
        QNAME_SYNTAX.is_match(s)
    }

    fn split(&self) -> (&str, &str, Option<&str>) {
        let groups = QNAME_SYNTAX.captures(&self.0).unwrap();
        (
            groups.get(1).unwrap().as_str(),
            groups.get(2).unwrap().as_str(),
            groups.get(4).map(|s| s.as_str()),
        )
    }
}

// ------------------------------------------------------------------------------------------------

impl Display for HostName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<HostName> for String {
    fn from(v: HostName) -> Self {
        v.0
    }
}

impl Deref for HostName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for HostName {
    type Err = IamFormatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if Self::is_valid(s) {
            Ok(Self(s.to_string()))
        } else {
            unexpected_value_for_type(HOST_NAME_NAME, s).into()
        }
    }
}

impl HostName {
    pub fn new_unchecked<S>(s: S) -> Self
    where
        S: Into<String>,
    {
        Self(s.into())
    }

    pub fn new<S>(s: S) -> Result<Self, IamFormatError>
    where
        S: Into<String>,
    {
        let s = s.into();
        if HOST_NAME_SYNTAX.is_match(&s) {
            Ok(Self(s))
        } else {
            unexpected_value_for_type(SERVICE_NAME_NAME, s).into()
        }
    }

    pub fn is_valid(s: &str) -> bool {
        HOST_NAME_SYNTAX.is_match(s)
    }

    pub fn head(&self) -> &str {
        self.0.split_once(HOSTNAME_SEPARATOR).unwrap().0
    }

    pub fn tail(&self) -> &str {
        self.0.split_once(HOSTNAME_SEPARATOR).unwrap().1
    }

    pub fn is_aws_service_name(&self) -> bool {
        self.tail() == AWS_SERVICE_TAIL
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

impl TryFrom<HostName> for ServiceName {
    type Error = IamFormatError;

    fn try_from(value: HostName) -> Result<Self, Self::Error> {
        ServiceName::from_str(value.deref())
    }
}

impl From<ServiceName> for HostName {
    fn from(v: ServiceName) -> Self {
        HostName::new_unchecked(format!("{}{}{}", v, HOSTNAME_SEPARATOR, AWS_SERVICE_TAIL))
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
        let service_name = if let Some((head, tail)) = s.split_once(HOSTNAME_SEPARATOR) {
            if tail == AWS_SERVICE_TAIL {
                head
            } else {
                return unexpected_value_for_type(SERVICE_NAME_NAME, s).into();
            }
        } else {
            s
        };
        if Self::is_valid(service_name) {
            Ok(Self::new_unchecked(service_name))
        } else {
            unexpected_value_for_type(SERVICE_NAME_NAME, s).into()
        }
    }
}

impl ServiceName {
    pub fn new_unchecked<S>(s: S) -> Self
    where
        S: Into<String>,
    {
        Self(s.into())
    }

    pub fn new<S>(s: S) -> Result<Self, IamFormatError>
    where
        S: Into<String>,
    {
        let s = s.into();
        if SERVICE_NAME_SYNTAX.is_match(&s) {
            Ok(Self(s))
        } else {
            unexpected_value_for_type(SERVICE_NAME_NAME, s).into()
        }
    }

    pub fn is_valid(s: &str) -> bool {
        SERVICE_NAME_SYNTAX.is_match(s)
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
    pub fn new_unchecked<S>(s: S) -> Self
    where
        S: Into<String>,
    {
        Self(s.into())
    }

    pub fn is_valid(s: &str) -> bool {
        USER_ID_SYNTAX.is_match(s)
    }
}
