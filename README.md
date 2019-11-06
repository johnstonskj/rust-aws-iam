# Crate aws-iam

A Rust crate for dealing with [AWS IAM](https://docs.aws.amazon.com/IAM/latest/UserGuide/introduction.html) 
Policy resources.

![MIT License](https://img.shields.io/badge/license-mit-118811.svg)
![Minimum Rust Version](https://img.shields.io/badge/Min%20Rust-1.38-green.svg)
[![crates.io](https://img.shields.io/crates/v/quixotic.svg)](https://crates.io/crates/aws-iam)
[![docs.rs](https://docs.rs/quixotic/badge.svg)](https://docs.rs/aws-iam/)
[![GitHub stars](https://img.shields.io/github/stars/johnstonskj/rust-aws-iam.svg)](https://github.com/johnstonskj/rust-aws-iam/stargazers)

## Model

TBD

### Example

```rust
use aws_iam::model::*;

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
println!("{}", policy);
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

TBD

## Changes

**Version 0.1.0**

* First commit to Crates.io
* Completed markdown support for `policy` tool verification.
* Completed changes to the model to support `NotAction`, `NotPrincipal`, and `NotResource`.
* Filled obvious gaps in documentation.

**Version 0.1.0**

* Initial commit stream to Github from private project.
* Goal was to complete the existing model, documentation and add the `policy` tool.

## TODO

1. Add Latex output to `policy`.