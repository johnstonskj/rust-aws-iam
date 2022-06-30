/*!
This crate provides a Rust typed model for creating and reading AWS IAM Policy documents.
Whreever possible this crate uses documentation directly from the AWS IAM User Guide.

From [AWS Identity and Access Management Documentation](https://docs.aws.amazon.com/iam/index.html):

> The access management portion of AWS Identity and Access Management (IAM) helps you define
> what a principal entity is allowed to do in an account. A principal entity is a person or
> application that is authenticated using an IAM entity (user or role). Access management is often
> referred to as authorization. You manage access in AWS by creating policies and attaching them
> to IAM identities (users, groups of users, or roles) or AWS resources. A policy is an object in
> AWS that, when associated with an identity or resource, defines their permissions. AWS evaluates
> these policies when a principal uses an IAM entity (user or role) to make a request. Permissions
> in the policies determine whether the request is allowed or denied. Most policies are stored in
> AWS as JSON documents.

# Overview

This crate provides a set of types that can be used to serialize and deserialize IAM Policy
documents. For a simpler experience creating documents a [`builder`](model/builder/index.html)
module provides a more _fluent_ method for construction. The [`io`](io/index.html) module
provides basic support for reading and writing JSON files.

# Usage

The example JSON below is taken from [Overview of JSON
Policies](https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies.html#access_policies-json).

```json
{
  "Version": "2012-10-17",
  "Statement": [
    ...
    {
      "Sid": "ThirdStatement",
      "Effect": "Allow",
      "Action": [
        "s3:List*",
        "s3:Get*"
      ],
      "Resource": [
        "arn:aws:s3:::confidential-data",
        "arn:aws:s3:::confidential-data/ *"
      ],
      "Condition": {"Bool": {"aws:MultiFactorAuthPresent": "true"}}
    }
  ]
}
```

This can be constructed with the following code.

```rust,ignore
use std::collections::HashMap;
use aws_iam::model::*;
use aws_iam::model::builder::*;
use std::str::FromStr;

let condition = ConditionBuilder::new(GlobalConditionOperator::Bool)
    .right_hand_str("aws:MultiFactorAuthPresent", "true")
    .build_as_condition();
let policy = Policy {
    version: Some(Version::V2012),
    id: Some("test_access_policy_with_condition".to_string()),
    statement: OneOrAll::All(vec![Statement {
        sid: Some("ThirdStatement".to_string()),
        principal: None,
        effect: Effect::Allow,
        action: Action::these(&mut vec![
            "s3:List*".parse().unwrap(),
            "s3:Get*".parse().unwrap(),
        ]),
        resource: Resource::these(&mut vec![
            "arn:aws:s3:::confidential-data".to_string(),
            "arn:aws:s3:::confidential-data/-*".to_string(),
        ]),
        condition: Some(condition),
    }]),
};
println!("{}", policy.to_string());
```

# Features

The following features are supported by this crate and can be included as needed. By default the
crate only provides an in-memory data model with Serde support for reading and writing to the
standard JSON representation.

* `command_line` - provides a command line tool, `policy` that can verify and evaluate existing
  policy documents and create new ones from a set of templates.
* `document` - produces formatted documentation for a policy. This can be in the form of Markdown
  or LaTeX and would usually be used to document a policy template for others to follow.
* `offline_eval` - provides a simple, by which we mean incomplete, evaluation of a policy  using
  a request object to match. This is useful but not sufficient for testing policies.
* `service_config` - adds to the verification of policies by storing service-specific configuration
  on actions, resource formats, and condition keys.

*/

// ------------------------------------------------------------------------------------------------
// Preamble
// ------------------------------------------------------------------------------------------------

#![warn(
    missing_debug_implementations,
//    missing_docs,
    unused_extern_crates,
    rust_2018_idioms
)]

#[macro_use]
extern crate lazy_static;

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod error;

pub mod syntax;

pub mod model;

pub mod context;

pub mod io;

#[cfg(feature = "document")]
pub mod document;

#[cfg(feature = "offline_eval")]
pub mod offline;

#[cfg(feature = "service_config")]
pub mod service;
