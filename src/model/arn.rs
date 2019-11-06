/*!
Provides bare bones support for ARN values.
*/

use regex::Regex;
use serde::export::fmt::Error;
use serde::export::Formatter;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt::Display;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// Amazon Resource Names (ARNs) uniquely identify AWS resources. We require an ARN when you
/// need to specify a resource unambiguously across all of AWS, such as in IAM policies,
/// Amazon Relational Database Service (Amazon RDS) tags, and API calls.
///
///
/// The following are the general formats for ARNs; the specific components and values used
/// depend on the AWS service.
///
/// ```text
/// arn:partition:service:region:account-id:resource-id
/// arn:partition:service:region:account-id:resource-type/resource-id
/// arn:partition:service:region:account-id:resource-type:resource-id
/// ```
///
#[derive(Debug, Clone)]
pub struct ARN {
    /// The partition that the resource is in. For standard AWS Regions, the partition is` aws`.
    /// If you have resources in other partitions, the partition is `aws-partitionname`. For
    /// example, the partition for resources in the China (Beijing) Region is `aws-cn`.
    partition: Option<String>,
    /// The service namespace that identifies the AWS product (for example, Amazon S3, IAM,
    /// or Amazon RDS).
    service: String,
    /// The Region that the resource resides in. The ARNs for some resources do not require
    /// a Region, so this component might be omitted.
    region: Option<String>,
    /// The ID of the AWS account that owns the resource, without the hyphens. For example,
    /// `123456789012`. The ARNs for some resources don't require an account number, so this
    /// component might be omitted.
    account_id: Option<String>,
    /// The content of this part of the ARN varies by service. A resource identifier can
    /// be the name or ID of the resource (for example, `user/Bob` or
    /// `instance/i-1234567890abcdef0`) or a resource path. For example, some resource
    /// identifiers include a parent resource
    /// (`sub-resource-type/parent-resource/sub-resource`) or a qualifier such as a
    /// version (`resource-type:resource-name:qualifier`).
    resource_id: String,
}

///
/// The wildcard character.
///
pub const WILD: &str = "*";

///
/// Errors that may arise parsing an ARN with `FromStr::from_str()`.
///
#[derive(Debug)]
pub enum ArnError {
    /// Missing the 'arn' prefix string.
    MissingPrefix,
    /// Missing the partition component.
    MissingPartition,
    /// The partition component provided is not valid.
    InvalidPartition,
    /// Missing the service component.
    MissingService,
    /// The service component provided is not valid.
    InvalidService,
    /// Missing the region component.
    MissingRegion,
    /// The partition region provided is not valid.
    InvalidRegion,
    /// Missing the account id component.
    MissingAccountId,
    /// The partition account id provided is not valid.
    InvalidAccountId,
    /// Missing the resource component.
    MissingResource,
    /// The partition resource provided is not valid.
    InvalidResource,
}

///
/// Used to store a set of service-specific validators. So a validator registered for the
/// service "s3" could ensure that the resources section conforms to expected patterns.
///
#[derive(Default)]
#[allow(missing_debug_implementations)]
pub struct ArnValidators {
    services: HashMap<String, Box<dyn Fn(&ARN) -> Result<(), ArnError>>>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

const ARN_PREFIX: &str = "arn";

const ARN_SEPARATOR: &str = ":";

const DEFAULT_PARTITION: &str = "aws";

lazy_static! {
    static ref PARTITION: Regex = Regex::new(r"^aws(\-[a-zA-Z][a-zA-Z0-9\-]+)?$").unwrap();
    static ref SERVICE: Regex = Regex::new(r"^[a-zA-Z][a-zA-Z0-9\-]+$").unwrap();
}

impl ARN {
    ///
    /// Validate this ARN, if provided the `validators` struct will be used to also
    /// provide any service-specific validation.
    ///
    pub fn validate(&self, validators: Option<&ArnValidators>) -> Result<(), ArnError> {
        if let Some(partition) = &self.partition {
            if !PARTITION.is_match(&partition) {
                return Err(ArnError::InvalidPartition);
            }
        }
        if !SERVICE.is_match(&self.service) {
            return Err(ArnError::InvalidService);
        }
        if let Some(validators) = validators {
            if let Some(validator) = validators.get(&self.service) {
                return validator(self);
            }
        }
        Ok(())
    }
}

impl Display for ARN {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "{}",
            vec![
                ARN_PREFIX.to_string(),
                self.partition
                    .as_ref()
                    .unwrap_or(&DEFAULT_PARTITION.to_string())
                    .clone(),
                self.service.clone(),
                self.region.as_ref().unwrap_or(&String::new()).clone(),
                self.account_id.as_ref().unwrap_or(&String::new()).clone(),
                self.resource_id.clone()
            ]
            .join(ARN_SEPARATOR)
        )
    }
}

impl ArnValidators {
    ///
    /// Register `validator` as the function to call for service `svc_name`.
    ///
    pub fn register(
        &mut self,
        svc_name: String,
        validator: Box<dyn Fn(&ARN) -> Result<(), ArnError>>,
    ) {
        self.services.insert(svc_name, validator);
    }

    ///
    /// Deregister any validator function registered for service `svc_name`.
    ///
    pub fn deregister(&mut self, svc_name: String) {
        self.services.remove(&svc_name);
    }

    ///
    /// Return the validator function registered for service `svc_name`.
    ///
    pub fn get(&self, svc_name: &String) -> Option<&Box<dyn Fn(&ARN) -> Result<(), ArnError>>> {
        self.services.get(svc_name)
    }
}
