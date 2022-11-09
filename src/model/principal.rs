/*!
One-line description.
More detailed description, with
# Example
 */

use std::str::FromStr;

use crate::{
    error::{type_mismatch, unexpected_properties, unexpected_value_for_type, IamFormatError},
    model::{MaybeAny, OrAny},
    syntax::{
        display_vec_to_json, json_type_name, vec_from_str_json, IamProperty, IamValue,
        JSON_TYPE_NAME_ARRAY, JSON_TYPE_NAME_OBJECT, JSON_TYPE_NAME_STRING, POLICY_WILDCARD_VALUE,
        PRINCIPAL_NAME, PRINCIPAL_TYPE_AWS, PRINCIPAL_TYPE_CANONICAL_USER,
        PRINCIPAL_TYPE_FEDERATED, PRINCIPAL_TYPE_SERVICE, PRINCIPAL_VALUE_NOT_PRINCIPAL,
        PRINCIPAL_VALUE_PRINCIPAL,
    },
};
use aws_arn::{AccountIdentifier, ArnError, ARN};
use serde_json::{Map, Value};

use super::naming::{CanonicalUserId, HostName, ServiceName};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Use the Principal element to specify the IAM user, federated user, IAM role, AWS account,
/// AWS service, or other principal entity that is allowed or denied access to a resource. You
/// cannot use the Principal element in an IAM identity-based policy. You can use it in the
/// trust policies for IAM roles and in resource-based policies. Resource-based policies are
/// policies that you embed directly in an IAM resource.
///
/// From [AWS JSON Policy Elements: Principal](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_elements_principal.html)
/// and [AWS JSON Policy Elements: NotPrincipal](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_elements_notprincipal.html).
///
/// ## principal_id_string
///     
/// Provides a way to specify a principal using the Amazon Resource Name (ARN) of
/// the AWS account, IAM user, IAM role, federated user, or assumed-role user. For
/// an AWS account, you can also use the short form AWS:accountnumber instead of
/// the full ARN. For all of the options including AWS services, assumed roles,
/// and so on, see Specifying a principal.
///
/// Note that you can use * only to specify "everyone/anonymous." You cannot use
/// it to specify part of a name or ARN.
///
#[derive(Debug, Clone, PartialEq)]
pub enum Principal {
    /// Asserts that the principal in the request **must** match one of the specified ones.
    Principal(OrAny<PrincipalMap>),
    /// Asserts that the principal in the request **must not** match one of the specified ones.
    NotPrincipal(OrAny<PrincipalMap>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum PrincipalKind {
    Aws(ARN),
    Federated(HostName),
    Service(ServiceName),
    CanonicalUser(CanonicalUserId),
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct PrincipalMap {
    aws: Vec<ARN>,
    federated: Vec<HostName>,
    services: Vec<ServiceName>,
    canonical_users: Vec<CanonicalUserId>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl IamProperty for Principal {
    fn into_json_object(&self, object: &mut Map<String, Value>) -> Result<(), IamFormatError> {
        let _ = match self {
            Self::Principal(values) => object.insert(
                PRINCIPAL_VALUE_PRINCIPAL.to_string(),
                values.to_json().unwrap(),
            ),
            Self::NotPrincipal(values) => object.insert(
                PRINCIPAL_VALUE_NOT_PRINCIPAL.to_string(),
                values.to_json().unwrap(),
            ),
        };
        Ok(())
    }

    fn from_json_object_optional(value: &Map<String, Value>) -> Result<Option<Self>, IamFormatError>
    where
        Self: Sized,
    {
        match (
            value.get(PRINCIPAL_VALUE_PRINCIPAL),
            value.get(PRINCIPAL_VALUE_NOT_PRINCIPAL),
        ) {
            (Some(v), None) => Ok(Some(Principal::Principal(
                OrAny::<PrincipalMap>::from_json(v)?,
            ))),
            (None, Some(v)) => Ok(Some(Principal::NotPrincipal(
                OrAny::<PrincipalMap>::from_json(v)?,
            ))),
            (Some(_), Some(_)) => unexpected_properties(PRINCIPAL_NAME).into(),
            _ => Ok(None),
        }
    }

    fn from_json_object(_: &Map<String, Value>) -> Result<Self, IamFormatError>
    where
        Self: Sized,
    {
        unimplemented!()
    }
}

impl MaybeAny<PrincipalMap> for Principal {
    fn new_any() -> Self
    where
        Self: Sized,
    {
        Self::Principal(OrAny::Any)
    }

    fn new_none() -> Self
    where
        Self: Sized,
    {
        Self::NotPrincipal(OrAny::Any)
    }

    fn inner(&self) -> &OrAny<PrincipalMap> {
        match self {
            Self::Principal(v) => v,
            Self::NotPrincipal(v) => v,
        }
    }

    fn is_negative(&self) -> bool {
        matches!(self, Self::NotPrincipal(_))
    }
}

impl Principal {
    pub fn this<T>(principal: T) -> Self
    where
        T: Into<PrincipalKind>,
    {
        Self::Principal(OrAny::Some(PrincipalMap::from(principal.into())))
    }

    pub fn these<T>(principals: Vec<T>) -> Self
    where
        T: Into<PrincipalKind>,
    {
        Self::Principal(OrAny::Some(PrincipalMap::from(principals)))
    }

    pub fn not_this<T>(principal: T) -> Self
    where
        T: Into<PrincipalKind>,
    {
        Self::NotPrincipal(OrAny::Some(PrincipalMap::from(principal.into())))
    }

    pub fn not_these<T>(principals: Vec<T>) -> Self
    where
        T: Into<PrincipalKind>,
    {
        Self::NotPrincipal(OrAny::Some(PrincipalMap::from(principals)))
    }

    pub fn is_any(&self) -> bool {
        matches!(self.inner(), OrAny::Any)
    }

    pub fn is_some(&self) -> bool {
        matches!(self.inner(), OrAny::Some(_))
    }

    fn inner_mut(&mut self) -> &mut OrAny<PrincipalMap> {
        match self {
            Principal::Principal(map) => map,
            Principal::NotPrincipal(map) => map,
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl From<ARN> for PrincipalKind {
    fn from(v: ARN) -> Self {
        Self::Aws(v)
    }
}

impl From<HostName> for PrincipalKind {
    fn from(v: HostName) -> Self {
        Self::Federated(v)
    }
}

impl From<ServiceName> for PrincipalKind {
    fn from(v: ServiceName) -> Self {
        Self::Service(v)
    }
}

impl From<CanonicalUserId> for PrincipalKind {
    fn from(v: CanonicalUserId) -> Self {
        Self::CanonicalUser(v)
    }
}

// ------------------------------------------------------------------------------------------------

impl IamValue for OrAny<PrincipalMap> {
    fn to_json(&self) -> Result<Value, IamFormatError> {
        Ok(if let OrAny::Some(values) = self {
            values.to_json()?
        } else {
            Value::String(POLICY_WILDCARD_VALUE.to_string())
        })
    }

    fn from_json(value: &Value) -> Result<Self, IamFormatError>
    where
        Self: Sized,
    {
        if let Value::String(s) = value {
            if s == POLICY_WILDCARD_VALUE {
                Ok(OrAny::Any)
            } else {
                unexpected_value_for_type(PRINCIPAL_NAME, s).into()
            }
        } else {
            Ok(OrAny::Some(PrincipalMap::from_json(value)?))
            // TODO: check for "AWS": "*"
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl From<PrincipalKind> for PrincipalMap {
    fn from(v: PrincipalKind) -> Self {
        let mut map = Self::default();
        map.insert(v);
        map
    }
}

impl From<Vec<PrincipalKind>> for PrincipalMap {
    fn from(v: Vec<PrincipalKind>) -> Self {
        let mut map = Self::default();
        map.extend(v);
        map
    }
}

impl IamValue for PrincipalMap {
    fn to_json(&self) -> Result<Value, IamFormatError> {
        let mut object = Map::default();

        if !self.aws.is_empty() {
            object.insert(
                PRINCIPAL_TYPE_AWS.to_string(),
                display_vec_to_json(&self.aws)?,
            );
        }

        if !self.federated.is_empty() {
            object.insert(
                PRINCIPAL_TYPE_FEDERATED.to_string(),
                display_vec_to_json(&self.federated)?,
            );
        }

        if !self.services.is_empty() {
            object.insert(
                PRINCIPAL_TYPE_SERVICE.to_string(),
                display_vec_to_json(&self.services)?,
            );
        }

        if !self.canonical_users.is_empty() {
            object.insert(
                PRINCIPAL_TYPE_CANONICAL_USER.to_string(),
                display_vec_to_json(&self.canonical_users)?,
            );
        }

        Ok(Value::Object(object))
    }

    fn from_json(value: &Value) -> Result<Self, IamFormatError>
    where
        Self: Sized,
    {
        if let Value::Object(object) = value {
            let mut principals = PrincipalMap::default();
            if let Some(value) = object.get(PRINCIPAL_TYPE_AWS) {
                let results: Vec<ARN> = arn_vec_from_str_json(value)?;
                principals.aws = results;
            }
            if let Some(value) = object.get(PRINCIPAL_TYPE_FEDERATED) {
                let results: Vec<HostName> = vec_from_str_json(value, PRINCIPAL_TYPE_FEDERATED)?;
                principals.federated = results;
            }
            if let Some(value) = object.get(PRINCIPAL_TYPE_SERVICE) {
                let results: Vec<ServiceName> = vec_from_str_json(value, PRINCIPAL_TYPE_SERVICE)?;
                principals.services = results;
            }
            if let Some(value) = object.get(PRINCIPAL_TYPE_CANONICAL_USER) {
                let results: Vec<CanonicalUserId> =
                    vec_from_str_json(value, PRINCIPAL_TYPE_CANONICAL_USER)?;
                principals.canonical_users = results;
            }
            Ok(principals)
        } else {
            type_mismatch(PRINCIPAL_NAME, JSON_TYPE_NAME_OBJECT, json_type_name(value)).into()
        }
    }
}

impl PrincipalMap {
    pub fn insert<T>(&mut self, principal: T)
    where
        T: Into<PrincipalKind>,
    {
        match principal.into() {
            PrincipalKind::Aws(v) => self.insert_aws(v),
            PrincipalKind::Federated(v) => self.insert_federated(v),
            PrincipalKind::Service(v) => self.insert_service(v),
            PrincipalKind::CanonicalUser(v) => self.insert_canonical_user(v),
        }
    }
    pub fn extend<T>(&mut self, principals: Vec<T>)
    where
        T: Into<PrincipalKind>,
    {
        principals.into_iter().for_each(|p| self.insert(p))
    }

    pub fn insert_aws(&mut self, value: ARN) {
        self.aws.push(value)
    }

    pub fn extend_aws(&mut self, values: Vec<ARN>) {
        self.aws.extend(values.into_iter());
    }

    pub fn insert_federated(&mut self, value: HostName) {
        self.federated.push(value)
    }

    pub fn extend_federated(&mut self, values: Vec<HostName>) {
        self.federated.extend(values.into_iter());
    }

    pub fn insert_service(&mut self, value: ServiceName) {
        self.services.push(value)
    }

    pub fn extend_services(&mut self, values: Vec<ServiceName>) {
        self.services.extend(values.into_iter());
    }

    pub fn insert_canonical_user(&mut self, value: CanonicalUserId) {
        self.canonical_users.push(value)
    }

    pub fn extend_canonical_users(&mut self, values: Vec<CanonicalUserId>) {
        self.canonical_users.extend(values.into_iter());
    }

    /// When you use an AWS account identifier as the principal in a policy, you delegate
    /// authority to the account. Within that account, the permissions in the policy statement
    /// can be granted to all identities. This includes IAM users and roles in that account.
    /// When you specify an AWS account, you can use the account ARN
    /// (`arn:aws:iam::AWS-account-ID:root`), or a shortened form that consists of the `AWS:`
    /// prefix followed by the account ID.
    pub fn aws_iter(&self) -> impl Iterator<Item = &ARN> {
        self.aws.iter()
    }

    /// Federated users either using web identity federation or using a SAML identity provider.
    pub fn federated_iter(&self) -> impl Iterator<Item = &HostName> {
        self.federated.iter()
    }

    /// IAM roles that can be assumed by an AWS service are called service roles. Service roles
    /// must include a trust policy. Trust policies are resource-based policies that are attached
    /// to a role that define which principals can assume the role. Some service roles have
    /// predefined trust policies. However, in some cases, you must specify the service principal
    /// in the trust policy. A service principal is an identifier that is used to grant
    /// permissions to a service.
    pub fn service_iter(&self) -> impl Iterator<Item = &ServiceName> {
        self.services.iter()
    }

    /// The canonical user ID is an identifier for your account. Because this identifier is
    /// used by Amazon S3, only this service provides IAM users with access to the canonical
    /// user ID. You can also view the canonical user ID for your account from the AWS
    /// Management Console while signed in as the AWS account root user.
    pub fn canonical_user_iter(&self) -> impl Iterator<Item = &CanonicalUserId> {
        self.canonical_users.iter()
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

#[inline]
pub fn arn_vec_from_str_json(value: &Value) -> Result<Vec<ARN>, IamFormatError> {
    fn from_str(s: &str) -> Result<ARN, IamFormatError> {
        if s.contains(':') {
            ARN::from_str(s).map_err(ArnError::into)
        } else {
            let account = AccountIdentifier::from_str(s)?;
            Ok(account.into())
        }
    }
    if let Value::String(s) = value {
        Ok(vec![from_str(s)?])
    } else if let Value::Array(arr) = value {
        arr.iter()
            .map(|v| {
                if let Value::String(s) = v {
                    Ok(from_str(s)?)
                } else {
                    Err(type_mismatch(
                        PRINCIPAL_TYPE_AWS,
                        JSON_TYPE_NAME_STRING,
                        json_type_name(value),
                    ))
                }
            })
            .collect()
    } else {
        type_mismatch(
            PRINCIPAL_TYPE_AWS,
            JSON_TYPE_NAME_ARRAY,
            json_type_name(value),
        )
        .into()
    }
}
