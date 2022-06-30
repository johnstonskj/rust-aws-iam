/*!
Provides a Serde-enabled model for AWS Identity and Access Management (IAM) policies.

This implementation only provides a convenient API to construct and consume IAM
policy resources using [Serde](https://serde.rs/) to serialize and deserialize into
the AWS-defined JSON representation.

# Policy Grammar

The following is taken from the latest AWS [IAM User
Guide](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_grammar.html).
Note that this is a logical grammar, the serialization in JSON has some specifics
documented in the notes below.

```plain,ignore
policy  = {
     <version_block?>
     <id_block?>
     <statement_block>
}

<version_block> = "Version" : ("2008-10-17" | "2012-10-17")

<id_block> = "Id" : <policy_id_string>

<statement_block> = "Statement" : [ <statement>, <statement>, ... ]

<statement> = {
    <sid_block?>,
    <principal_block?>,
    <effect_block>,
    <action_block>,
    <resource_block>,
    <condition_block?>
}

<sid_block> = "Sid" : <sid_string>

<effect_block> = "Effect" : ("Allow" | "Deny")

<principal_block> = ("Principal" | "NotPrincipal") : ("*" | <principal_map>)

<principal_map> = { <principal_map_entry>, <principal_map_entry>, ... }

<principal_map_entry> = ("AWS" | "Federated" | "Service" | "CanonicalUser") :
    [<principal_id_string>, <principal_id_string>, ...]

<action_block> = ("Action" | "NotAction") :
    ("*" | [<action_string>, <action_string>, ...])

<resource_block> = ("Resource" | "NotResource") :
    ("*" | [<resource_string>, <resource_string>, ...])

<condition_block> = "Condition" : { <condition_map> }
<condition_map> = {
  <condition_type_string> : { <condition_key_string> : <condition_value_list> },
  <condition_type_string> : { <condition_key_string> : <condition_value_list> }, ...
}
<condition_value_list> = [<condition_value>, <condition_value>, ...]
<condition_value> = ("string" | "number" | "Boolean")
```

## Grammar Notes

1. For those blocks that appear to take a list of strings, i.e. `principal_map_entry`
   contains a list of `principal_id_string`, `action_block`, `resource_block`, and
   `condition_value_list` these may be serialized as a JSON array of values, or as simply
   a single string if there is only one value. This is implemented by using enums
   that construct a `One` variant or `All`/`AnyOf` variant.
1. For thse blocks which accept a wild card, `principal_block`, `action_block`, and
   `resource_block` the `Qualified` enum has an `Any` variant.
1. The grammar for `condition_map` appears to suggest that there is only one value
   for `condition_key_string`, this is not the case, the right-hand side of the
   `condition_map` is itself a map.
1. The constraint that _The `id_block` is allowed in resource-based policies, but
   not in identity-based policies.` is ignored in this implementation.
1. The constraint that _For IAM policies, basic alphanumeric characters (A-Z,a-z,0-9)
   are the only allowed characters in the `Sid` value. Other AWS services that support
   resource policies may have other requirements for the `Sid` value._ is ignored in
   this implementation.
1. The value of `principal_id_string` **must** be an [Amazon Resource
   Name (ARN)](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_identifiers.html#identifiers-arns),
   and the value of `resource_string` is **most likely** an ARN. This is not validated
   in this implementation.
1. While most values for `condition_type_string` defined in [IAM JSON Policy Elements:
   Condition Operators](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_elements_condition_operators.html)
   are provided, the prefixes `ForAllValues` and `ForAnyValue` are not supported.
1. The value of `condition_key_string` is in effect an open-set enumeration, and
   while some values are described within [AWS Global Condition Context
   Keys](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_condition-keys.html)
   these are not validated in this implementation.

# Example

The example below implements a simple policy described in the IAM User Guide
[Access Policies](https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies.html#access_policies-json)
section.

```rust,ignore
use aws_iam::model::*;

let policy = Policy {
    version: Some(Version::V2012),
    id: Some("test_simple_access_policy".to_string()),
    statement: OneOrAll::One(Statement {
        sid: None,
        principal: None,
        effect: Effect::Allow,
        action: Action::Action(OneOrAny::One("s3:ListBucket".parse().unwrap())),
        resource: Resource::this("arn:aws:s3:::example_bucket".to_string()),
        condition: None,
    }),
};
let json = serde_json::to_string(&policy);
assert!(json.is_ok());
println!("JSON: {:#?}", json);
```

Alternatively using the `builder` module we can accomplish the same result with the following.

```rust,ignore
use aws_iam::model::*;
use aws_iam::io::to_string;

let policy: Policy = Policy::named(
    "test_simple_access_policy"
    vec![
        Statement::unnamed()
            .allows()
            .may_perform_action("s3:ListBucket")
            .on_resource("arn:aws:s3:::example_bucket")
    ]);
let json = io::to_string(&policy);
assert!(json.is_ok());
println!("JSON: {:#?}", json);
```

# Mapping from AWS Names

A JSON policy document includes these elements:

* Optional policy-wide information at the top of the document
* One or more individual statements

Each statement includes information about a single permission. If a policy includes multiple
statements, AWS applies a logical OR across the statements when evaluating them. If multiple
policies apply to a request, AWS applies a logical OR across all of those policies when
evaluating them. The information in a statement is contained within a series of elements.

* **Version** – Specify the version of the policy language that you want to use. As a best
  practice, use the latest 2012-10-17 version.
* **Statement** – Use this main policy element as a container for the following elements. You
  can include more than one statement in a policy.
* **Sid** (Optional) – Include an optional statement ID to differentiate between your statements.
* **Effect** – Use Allow or Deny to indicate whether the policy allows or denies access.
* **Principal** (Required in only some circumstances) – If you create a resource-based policy,
  you must indicate the account, user, role, or federated user to which you would like to allow
  or deny access. If you are creating an IAM permissions policy to attach to a user or role, you
  cannot include this element. The principal is implied as that user or role.
* **Action** – Include a list of actions that the policy allows or denies.
* **Resource** (Required in only some circumstances) – If you create an IAM permissions policy,
  you must specify a list of resources to which the actions apply. If you create a resource-based
  policy, this element is optional. If you do not include this element, then the resource to which
  the action applies is the resource to which the policy is attached.
* **Condition** (Optional) – Specify the circumstances under which the policy grants permission.

From [Overview of JSON Policies](https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies.html#access_policies-json).

*/

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub enum OrAny<T> {
    Any,
    Some(T),
}

pub trait MaybeAny<T> {
    fn new_any() -> Self
    where
        Self: Sized;

    fn new_none() -> Self
    where
        Self: Sized;

    fn is_negative(&self) -> bool;

    fn is_any(&self) -> bool {
        matches!(self.inner(), OrAny::Any)
    }

    fn is_some(&self) -> bool {
        matches!(self.inner(), OrAny::Some(_))
    }

    fn some(&self) -> Option<&T> {
        if let OrAny::Some(v) = self.inner() {
            Some(v)
        } else {
            None
        }
    }

    fn inner(&self) -> &OrAny<T>;
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

impl<T> From<T> for OrAny<T> {
    fn from(v: T) -> Self {
        Self::Some(v)
    }
}

impl<T: Clone> Clone for OrAny<T> {
    #[inline]
    fn clone(&self) -> Self {
        match self {
            Self::Any => Self::Any,
            Self::Some(vs) => Self::Some(vs.clone()),
        }
    }
}

impl<T: PartialEq> PartialEq for OrAny<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Any, Self::Any) => true,
            (Self::Some(lhs), Self::Some(rhs)) => lhs.eq(rhs),
            _ => false,
        }
    }
}

impl<T: Eq> Eq for OrAny<T> {}

impl<T> OrAny<T> {
    pub fn is_any(&self) -> bool {
        matches!(self, Self::Any)
    }

    pub fn is_some(&self) -> bool {
        matches!(self, Self::Some(_))
    }

    pub fn some(&self) -> Option<&T> {
        if let Self::Some(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn map<U, F>(self, f: F) -> OrAny<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            OrAny::Any => OrAny::Any,
            OrAny::Some(v) => OrAny::Some(f(v)),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod policy;
pub use policy::Policy;

pub mod version;
pub use version::Version;

pub mod statement;
pub use statement::Statement;

pub mod effect;
pub use effect::Effect;

pub mod principal;
pub use principal::Principal;

pub mod action;
pub use action::Action;

pub mod resource;
pub use resource::Resource;

pub mod condition;
pub use condition::{Condition, ConditionValue, GlobalOperator, Match, Operator, Quantifier};

pub mod naming;
pub use naming::{CanonicalUserId, QualifiedName, ServiceName};
