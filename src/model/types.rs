use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------
///
/// An IAM policy resource.
///
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Policy {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The IAM version of the policy grammar used in this resource
    pub version: Option<Version>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The identifier of this policy, if any
    pub id: Option<String>,
    /// One or more policy statements
    pub statement: Statements,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Statements {
    One(Statement), // TODO this makes a large enum, consider One(Box<Statement>)
    All(Vec<Statement>),
}

///
/// The Statement element is the main element for a policy. This element is required. It can include multiple elements (see the subsequent sections in this page). The Statement element contains an array of individual statements. Each individual statement is a JSON block enclosed in braces { }.
///
/// From [IAM JSON Policy Elements: Statement](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_elements_statement.html).
///
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Statement {
    ///
    /// The Sid (statement ID) is an optional identifier that you provide for the policy statement. You can assign a Sid value to each statement in a statement array. In services that let you specify an ID element, such as SQS and SNS, the Sid value is just a sub-ID of the policy document's ID. In IAM, the Sid value must be unique within a JSON policy
    ///
    /// In IAM, the Sid is not exposed in the IAM API. You can't retrieve a particular statement based on this ID.
    ///
    /// From [IAM JSON Policy Elements: Sid](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_elements_sid.html).
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal: Option<Principal>,
    pub effect: Effect,
    pub action: Action,
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<HashMap<ConditionOperator, HashMap<String, ConditionValues>>>,
}

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
#[derive(Debug, Serialize, Deserialize)]
pub enum Version {
    #[serde(rename = "2008-10-17")]
    /// This is the current version of the policy language, and you should always
    /// include a Version element and set it to 2012-10-17. Otherwise, you cannot
    /// use features such as policy variables that were introduced with this version.
    V2008,
    #[serde(rename = "2012-10-17")]
    /// This was an earlier version of the policy language. You might see this
    /// version on older existing policies. Do not use this version for any new
    /// policies or when you update any existing policies.
    V2012,
}

///
/// The Effect element is required and specifies whether the statement results in an allow or an explicit deny. Valid values for Effect are Allow and Deny.
///
/// From [IAM JSON Policy Elements: Effect](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_elements_effect.html).
///
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Effect {
    Allow,
    Deny,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Qualified<T = String> {
    #[serde(rename = "*")]
    Any,
    One(T),
    AnyOf(Vec<T>),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct QString {
    pub(crate) qualifier: Option<String>,
    pub(crate) value: String,
}

///
/// The Action element describes the specific action or actions that will be allowed or denied. Statements must include either an Action or NotAction element. Each AWS service has its own set of actions that describe tasks that you can perform with that service.
///
/// You specify a value using a service namespace as an action prefix (iam, ec2 sqs, sns, s3, etc.) followed by the name of the action to allow or deny. The name must match an action that is supported by the service. The prefix and the action name are case insensitive. For example, iam:ListAccessKeys is the same as IAM:listaccesskeys.
///
/// From [IAM JSON Policy Elements: Action](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_elements_action.html)
/// and [IAM JSON Policy Elements: NotAction](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_elements_notaction.html).
///
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Action {
    Action(Qualified<QString>),
    NotAction(Qualified<QString>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PrincipalType {
    AWS,
    Federated,
    Service,
    CanonicalUser,
}

///
/// Use the Principal element to specify the IAM user, federated user, IAM role, AWS account, AWS service, or other principal entity that is allowed or denied access to a resource. You cannot use the Principal element in an IAM identity-based policy. You can use it in the trust policies for IAM roles and in resource-based policies. Resource-based policies are policies that you embed directly in an IAM resource.
///
/// From [AWS JSON Policy Elements: Principal](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_elements_principal.html)
/// and [AWS JSON Policy Elements: NotPrincipal](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_elements_notprincipal.html).
///
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Principal {
    Principal(HashMap<PrincipalType, Qualified>),
    NotPrincipal(HashMap<PrincipalType, Qualified>),
}

///
/// The Resource element specifies the object or objects that the statement covers. Statements must include either a Resource or a NotResource element. You specify a resource using an ARN.
///
/// From [IAM JSON Policy Elements: Resource](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_elements_resource.html)
/// and [IAM JSON Policy Elements: NotResource](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_elements_notresource.html).
///
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Resource {
    Resource(Qualified),
    NotResource(Qualified),
}

///
/// You can use the Condition element of a policy to test multiple keys or multiple
/// values for a single key in a request. You can use condition keys to test the
/// values of the matching keys in the request. For example, you can use a condition
/// key to control access to specific attributes of a DynamoDB table or to an Amazon
/// EC2 instance based on tags.
///
/// From [Creating a Condition with Multiple Keys or Values](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_multi-value-conditions.html).
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConditionOperatorQuantifier {
    ForAllValues,
    ForAnyValue,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConditionOperator {
    /// Used to test multiple keys or multiple values for a single key in a request.
    pub quantifier: Option<ConditionOperatorQuantifier>,
    /// The condition operator you choose to use.
    pub base_type: GlobalConditionOperator,
    /// You use this to say "If the policy key is present in the context of the
    /// request, process the key as specified in the policy. If the key is not
    /// present, evaluate the condition element as true." Other condition elements
    /// in the statement can still result in a nonmatch, but not a missing key
    /// when checked with ...`IfExists`.
    pub only_if_exists: bool,
}
///
/// Use condition operators in the `Condition` element to match the condition
/// key and value in the policy against values in the request context.
///
/// The condition operator that you can use in a policy depends on the condition
/// key you choose. You can choose a global condition key or a service-specific
/// condition key.
///
/// From [IAM JSON Policy Elements: Condition Operators](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_elements_condition_operators.html).
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GlobalConditionOperator {
    // ----- String Condition Operators
    /// Exact matching, case sensitive
    StringEquals,
    /// Negated matching
    StringNotEquals,
    /// Exact matching, ignoring case
    StringEqualsIgnoreCase,
    /// Negated matching, ignoring case
    StringNotEqualsIgnoreCase,
    /// Case-sensitive matching. The values can include a multi-character
    /// match wildcard (*) or a single-character match wildcard (?) anywhere
    /// in the string.
    StringLike,
    /// Negated case-sensitive matching. The values can include a multi-character
    /// match wildcard (*) or a single-character match wildcard (?) anywhere
    /// in the string.
    StringNotLike,
    // ----- Numeric Condition Operators
    NumericEquals,
    /// Matching
    NumericNotEquals,
    /// Negated matching
    NumericLessThan,
    /// "Less than" matching
    NumericLessThanEquals,
    /// "Less than or equals" matching
    NumericGreaterThan,
    /// "Greater than" matching
    NumericGreaterThanEquals,
    // ----- Date Condition Operators
    /// Matching a specific date
    DateEquals,
    /// Negated matching
    DateNotEquals,
    /// Matching before a specific date and time
    DateLessThan,
    /// Matching at or before a specific date and time
    DateLessThanEquals,
    /// Matching after a specific a date and time
    DateGreaterThan,
    /// Matching at or after a specific date and time
    DateGreaterThanEquals,
    // ----- Boolean Condition Operators
    /// Boolean matching
    Bool,
    // ----- Binary Condition Operators
    /// The BinaryEquals condition operator let you construct Condition
    /// elements that test key values that are in binary format. It compares
    /// the value of the specified key byte for byte against a base-64
    /// encoded representation of the binary value in the policy.
    BinaryEquals,
    // ----- IP Address Condition Operators
    /// The specified IP address or range
    IpAddress,
    /// ll IP addresses except the specified IP address or range
    NotIpAddress,
    // ----- ARN Condition Operators
    /// Case-sensitive matching of the ARN. Each of the six colon-delimited
    /// components of the ARN is checked separately and each can include a
    /// multi-character match wildcard (*) or a single-character match
    /// wildcard (?).
    ArnEquals,
    /// Case-sensitive matching of the ARN. Each of the six colon-delimited
    /// components of the ARN is checked separately and each can include a
    /// multi-character match wildcard (*) or a single-character match
    /// wildcard (?).
    ArnLike,
    /// Negated matching for ARN.
    ArnNotEquals,
    /// Negated matching for ARN.
    ArnNotLike,
    // ------ Check Existence of Condition Keys
    /// Use a Null condition operator to check if a condition key is
    /// present at the time of authorization. In the policy statement, use
    /// either true (the key doesn't exist — it is null) or false (the key
    /// exists and its value is not null).
    Null,
    // ----- Custom Condition Operator
    /// The name of a custom condition
    Other(QString),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConditionValue {
    String(String),
    Integer(i64),
    Float(f64),
    Bool(bool),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConditionValues {
    One(ConditionValue),
    All(Vec<ConditionValue>),
}
