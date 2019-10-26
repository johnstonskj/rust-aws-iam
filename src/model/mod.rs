/*!
* Provides a Serde-enabled model for AWS Identity and Access Management (IAM) policies.
*
* This implementation only provides a convenient API to construct and consume IAM
* policy resources using [Serde](https://serde.rs/) to serialize and deserialize into
* the AWS-defined JSON representation.
*
* # Policy Grammar
*
* The following is taken from the latest AWS [IAM User
* Guide](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_grammar.html).
* Note that this is a logical grammar, the serialization in JSON has some specifics
* documented in the notes below.
*
* ```plain,ignore
* policy  = {
*      <version_block?>
*      <id_block?>
*      <statement_block>
* }
*
* <version_block> = "Version" : ("2008-10-17" | "2012-10-17")
*
* <id_block> = "Id" : <policy_id_string>
*
* <statement_block> = "Statement" : [ <statement>, <statement>, ... ]
*
* <statement> = {
*     <sid_block?>,
*     <principal_block?>,
*     <effect_block>,
*     <action_block>,
*     <resource_block>,
*     <condition_block?>
* }
*
* <sid_block> = "Sid" : <sid_string>
*
* <effect_block> = "Effect" : ("Allow" | "Deny")
*
* <principal_block> = ("Principal" | "NotPrincipal") : ("*" | <principal_map>)
*
* <principal_map> = { <principal_map_entry>, <principal_map_entry>, ... }
*
* <principal_map_entry> = ("AWS" | "Federated" | "Service" | "CanonicalUser") :
*     [<principal_id_string>, <principal_id_string>, ...]
*
* <action_block> = ("Action" | "NotAction") :
*     ("*" | [<action_string>, <action_string>, ...])
*
* <resource_block> = ("Resource" | "NotResource") :
*     ("*" | [<resource_string>, <resource_string>, ...])
*
* <condition_block> = "Condition" : { <condition_map> }
* <condition_map> = {
*   <condition_type_string> : { <condition_key_string> : <condition_value_list> },
*   <condition_type_string> : { <condition_key_string> : <condition_value_list> }, ...
* }
* <condition_value_list> = [<condition_value>, <condition_value>, ...]
* <condition_value> = ("string" | "number" | "Boolean")
* ```
*
* ## Grammar Notes
*
* 1. For those blocks that appear to take a list of strings, i.e. `principal_map_entry`
*    contains a list of `principal_id_string`, `action_block`, `resource_block`, and
*    `condition_value_list` these may be serialized as a JSON array of values, or as simply
*    a single string if there is only one value. This is implemented by using enums
*    that construct a `One` variant or `All`/`AnyOf` variant.
* 1. For thse blocks which accept a wild card, `principal_block`, `action_block`, and
*    `resource_block` the `Qualified` enum has an `Any` variant.
* 1. The grammar for `condition_map` appears to suggest that there is only one value
*    for `condition_key_string`, this is not the case, the right-hand side of the
*    `condition_map` is itself a map.
* 1. The constraint that _The `id_block` is allowed in resource-based policies, but
*    not in identity-based policies.` is ignored in this implementation.
* 1. The constraint that _For IAM policies, basic alphanumeric characters (A-Z,a-z,0-9)
*    are the only allowed characters in the `Sid` value. Other AWS services that support
*    resource policies may have other requirements for the `Sid` value._ is ignored in
*    this implementation.
* 1. The value of `principal_id_string` **must** be an [Amazon Resource
*    Name (ARN)](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_identifiers.html#identifiers-arns),
*    and the value of `resource_string` is **most likely** an ARN. This is not validated
*    in this implementation.
* 1. While most values for `condition_type_string` defined in [IAM JSON Policy Elements:
*    Condition Operators](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_elements_condition_operators.html)
*    are provided, the prefixes `ForAllValues` and `ForAnyValue` are not supported.
* 1. The value of `condition_key_string` is in effect an open-set enumeration, and
*    while some values are described within [AWS Global Condition Context
*    Keys](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_condition-keys.html)
*    these are not validated in this implementation.
*
* # Example
*
* The example below implements a simple policy described in the IAM User Guide
* [Access Policies](https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies.html#access_policies-json)
* section.
*
* ```
* use aws_iam::model::*;
* use aws_iam::model::builder::*;
*
* let policy = Policy {
*     version: Some(Version::V2012),
*     id: Some("test_simple_access_policy".to_string()),
*     statement: Statements::One(Statement {
*         sid: None,
*         principal: None,
*         effect: Effect::Allow,
*         action: Action::Action(this("s3:ListBucket")),
*         resource: Resource::Resource(this("arn:aws:s3:::example_bucket")),
*         condition: None,
*     }),
* };
* let json = serde_json::to_string(&policy);
* assert!(json.is_ok());
* println!("JSON: {:#?}", json);
* ```
*/

pub mod builder;

pub mod qstring;

pub mod types;
pub use types::*;

pub mod impls;
pub use impls::*;
