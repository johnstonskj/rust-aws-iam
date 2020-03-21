# Crate aws-iam

A Rust crate for dealing with [AWS IAM](https://docs.aws.amazon.com/IAM/latest/UserGuide/introduction.html) 
Policy resources.

![MIT License](https://img.shields.io/badge/license-mit-118811.svg)
![Minimum Rust Version](https://img.shields.io/badge/Min%20Rust-1.40-green.svg)
[![crates.io](https://img.shields.io/crates/v/aws-iam.svg)](https://crates.io/crates/aws-iam)
[![docs.rs](https://docs.rs/aws-iam/badge.svg)](https://docs.rs/aws-iam)
[![travis.ci](https://travis-ci.org/johnstonskj/rust-aws-iam.svg?branch=master)](https://travis-ci.org/johnstonskj/rust-aws-iam)
[![GitHub stars](https://img.shields.io/github/stars/johnstonskj/rust-aws-iam.svg)](https://github.com/johnstonskj/rust-aws-iam/stargazers)

## Model

For the most part importing `aws_iam::model` provides the core types necessary to programmatically create
Policy documents. You can also import `aws_iam::model::builder` to use a more _fluent_ interface to construct
Policies. The `aws_iam::io` module provides simple read and write functions, the write functions producing
_pretty printed_ JSON output.

The `aws_iam::report` module provides a set of traits that allow for visiting a Policy model, and implementations
of these that write formatted versions of a Policy as documentation.

### Example

```rust
use aws_iam::model::*;
use aws_iam::io::write_to_writer;
use std::io::stdout;

let policy: Policy = PolicyBuilder::new()
    .named("confidential-data-access")
    .evaluate_statement(
        StatementBuilder::new()
            .auto_named()
            .allows()
            .unspecified_principals()
            .may_perform_actions(vec!["s3:List*", "s3:Get*"])
            .on_resources(vec![
                "arn:aws:s3:::confidential-data",
                "arn:aws:s3:::confidential-data/*",
            ])
            .if_condition(
                ConditionBuilder::new_bool()
                    .right_hand_bool("aws:MultiFactorAuthPresent", true)
                    .if_exists(),
            ),
    )
    .into();
write_to_writer(stdout(), &policy);
```

Results in the following JSON.

```json
{
  "Id": "confidential-data-access",
  "Statement": {
    "Sid": "sid_e4d7f2d3-cfed-4346-9c5e-a8e9e38ef44f",
    "Effect": "Allow",
    "Action": [
      "s3:List*",
      "s3:Get*"
    ],
    "Resource": [
      "arn:aws:s3:::confidential-data",
      "arn:aws:s3:::confidential-data/*"
    ],
    "Condition": {
      "BoolIfExists": {
        "aws:MultiFactorAuthPresent": "true"
      }
    }
  }
}
```

## policy Command-Line Tool

The `policy` tool provides some very basic policy resource operations. The most valuable of these is `verify` which
will read a file, parse it and produce a formatted output. This output can be a documentation form which is useful 
for describing common policies. 

```bash
 $ policy -h
policy 0.2.0

USAGE:
    policy [FLAGS] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    The level of logging to perform, from off to trace

SUBCOMMANDS:
    help      Prints this message or the help of the given subcommand(s)
    new       Create a new default policy document
    verify    Verify an existing policy document
```

For example, given the following JSON policy:

```json
{
  "Version": "2012-10-17",
  "Statement": [{
    "Sid": "DenyAllUsersNotUsingMFA",
    "Effect": "Deny",
    "NotAction": "iam:*",
    "Resource": "*",
    "Condition": {"BoolIfExists": {"aws:MultiFactorAuthPresent": "false"}}
  }]
}
```

the command `policy verify -f markdown` will produce the output between the following lines.

-----
# Policy

> IAM Policy Version: 2012-10-17

## Statement

> Statement ID: DenyAllUsersNotUsingMFA

**DENY** IF

* `Action `**`NOT`**` = "iam:*"`
* `Resource  = "*"`
* `Condition `**`IF EXISTS`**` `*`aws:MultiFactorAuthPresent`*` `**`THEN`**
   * *`aws:MultiFactorAuthPresent`*` `**`Bool`**` "false"`
-----

## Changes

**Version 0.2.1**

* Fixing `missing_docs` warnings.
* Removed `any_of()`, `condition_one()`, and `one()` from builder, replaced with functions on Action, Principal, and Resource.

**Version 0.2.0**

* First commit to Crates.io.
* Completed markdown support for `policy` tool verification.
* Completed changes to the model to support `NotAction`, `NotPrincipal`, and `NotResource`.
* Filled obvious gaps in documentation.

**Version 0.1.0**

* Initial commit stream to Github from private project.
* Goal was to complete the existing model, documentation and add the `policy` tool.

## TODO

1. Add Latex output to `policy`.
