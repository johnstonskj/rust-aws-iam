/*!
Constants defines in the AWS Documentation.

# Condition Context Keys

When a principal makes a request to AWS, AWS gathers the request information into a
request context. You can use the Condition element of a JSON policy to compare the
request context with values that you specify in your policy. To learn more about
the circumstances under which a global key is included in the request context, see
the Availability information for each global condition key.

From [AWS Global Condition Context Keys](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_condition-keys.html).

*/

/// Use this key to compare the date and time of the request with the date and time
///  that you specify in the policy.
pub const AWS_CURRENT_TIME: &str = "aws:CurrentTime";

/// Use this key to compare the date and time of the request in epoch or Unix time
/// with the value that you specify in the policy. This key also accepts the number
/// of seconds since January 1, 1970.
pub const AWS_EPOCH_TIME: &str = "aws:EpochTime";

/// Use this key to compare the number of seconds since the requesting principal
/// was authorized using MFA with the number that you specify in the policy.
pub const AWS_MFA_AGE: &str = "aws:MultiFactorAuthAge";

/// Use this key to check whether multi-factor authentication (MFA) was used to
/// validate the temporary security credentials that made the request.
pub const AWS_MFA_PRESENT: &str = "aws:MultiFactorAuthPresent";

/// Use this key to compare the account to which the requesting principal belongs
/// with the account identifier that you specify in the policy.
pub const AWS_PRINCIPAL_ACCOUNT: &str = "aws:PrincipalAccount";

/// Use this key to compare the Amazon Resource Name (ARN) of the principal that
/// made the request with the ARN that you specify in the policy. For IAM roles,
/// the request context returns the ARN of the role, not the ARN of the user that
/// assumed the role.
pub const AWS_PRINCIPAL_ARN: &str = "aws:PrincipalArn";

/// Use this key to compare the identifier of the organization in AWS Organizations
/// to which the requesting principal belongs with the identifier specified in
/// the policy.
pub const AWS_PRINCIPAL_ORG_ID: &str = "aws:PrincipalOrgID";

/// Use this key to compare the tag attached to the principal making the request
/// with the tag that you specify in the policy. If the principal has more than
/// one tag attached, the request context includes one aws:PrincipalTag key for
/// each attached tag key.
pub const AWS_PRINCIPAL_TAG: &str = "aws:PrincipalTag/";

/// Use this key to compare the type of principal making the request with the
/// principal type that you specify in the policy.
pub const AWS_PRINCIPAL_TYPE: &str = "aws:PrincipalType";

/// Use this key to compare who referred the request in the client browser with
/// the referer that you specify in the policy. The aws:referer request context
/// value is provided by the caller in an HTTP header.
pub const AWS_REFERER: &str = "aws:Referer";

/// Use this key to compare the AWS Region that was called in the request with
/// the region that you specify in the policy. You can use this global condition
/// key to control which Regions can be requested.
pub const AWS_REQUESTED_REGION: &str = "aws:RequestedRegion";

/// Use this key to compare the tag key-value pair that was passed in the request
/// with the tag pair that you specify in the policy. For example, you could check
/// whether the request includes the tag key "Dept" and that it has the value
/// "Accounting".
pub const AWS_REQUEST_TAG: &str = "aws:RequestTag/";

/// Use this key to compare the tag key-value pair that you specify in the policy
/// with the key-value pair that is attached to the resource. For example, you
/// could require that access to a resource is allowed only if the resource has
/// the attached tag key "Dept" with the value "Marketing".
pub const AWS_RESOURCE_TAG: &str = "aws:ResourceTag/";

/// Use this key to check whether the request was sent using SSL. The request
/// context returns true or false. In a policy, you can allow specific actions
/// only if the request is sent using SSL.
pub const AWS_SECURE_TRANSPORT: &str = "aws:SecureTransport";

/// Use this key to compare the source of the request with the account ID that
/// you specify in the policy.
///
/// For example, assume that you have an Amazon S3 bucket in your account that
/// is configured to deliver object creation events to an Amazon SNS topic. In
/// that case, you could use this condition key to check that Amazon S3 is not being
/// used as a confused deputy. Amazon S3 tells Amazon SNS the account that the
/// bucket belongs to.
pub const AWS_SOURCE_ACCOUNT: &str = "aws:SourceAccount";

/// Use this key to compare the source of the request with the Amazon Resource
/// Name (ARN) that you specify in the policy.
///
/// For example, when an Amazon S3 bucket update triggers an Amazon SNS topic
/// post, the Amazon S3 service invokes the sns:Publish API operation. The bucket
/// is considered the source of the SNS request and the value of the key is the
/// bucket's ARN. This key does not work with the ARN of the principal making
/// the request. Instead, use aws:PrincipalArn.
pub const AWS_SOURCE_ARN: &str = "aws:SourceArn";

/// Use this key to compare the requester's IP address with the IP address that
/// you specify in the policy.
pub const AWS_SOURCE_IP: &str = "aws:SourceIp";

/// Use this key to check whether the request comes from the VPC that you specify
/// in the policy. In a policy, you can use this key to allow access to only
/// a specific VPC.
pub const AWS_SOURCE_VPC: &str = "aws:SourceVpc";

/// .Use this key to compare the VPC endpoint identifier of the request with the
/// endpoint ID that you specify in the policy. In a policy, you can use this
/// key to restrict access to a specific VPC endpoint
pub const AWS_SOURCE_VPCE: &str = "aws:SourceVpce";

/// Use this key to compare the tag keys in a request with the keys that you
/// specify in the policy. As a best practice when you use policies to control
/// access using tags, use the aws:TagKeys condition key to define what tag
/// keys are allowed.
pub const AWS_TAG_KEYS: &str = "aws:TagKeys";

/// Use this key to compare the date and time that temporary security credentials
/// were issued with the date and time that you specify in the policy.
pub const AWS_TOKEN_ISSUE_TIME: &str = "aws:TokenIssueTime";

/// Use this key to compare the requester's client application with the
/// application that you specify in the policy.
pub const AWS_USER_AGENT: &str = "aws:UserAgent";

/// Use this key to compare the requester's principal identifier with the ID that
/// you specify in the policy. For IAM users, the request context value is the
/// user ID. For IAM roles, this value format can vary.
pub const AWS_USER_ID: &str = "aws:userid";

/// Use this key to compare the requester's user name with the user name that you
/// specify in the policy.
pub const AWS_USER_NAME: &str = "aws:username";

/// Use this key to compare the IP address from which a request was made with the
/// IP address that you specify in the policy. In a policy, the key matches only
/// if the request originates from the specified IP address and it goes through
/// a VPC endpoint.
pub const AWS_VPC_SOURCE_ID: &str = "aws:VpcSourceIp";
