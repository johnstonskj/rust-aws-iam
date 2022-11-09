/*!
Provides a convenient and fluent builder interface for constructing policies.

# Example

```rust
use aws_iam::model::*;
use aws_iam::model::builder::*;
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
                "arn:aws:s3:::confidential-data/_*",
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
*/

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

#[doc(hidden)]
mod policy;
pub use policy::PolicyBuilder;

#[doc(hidden)]
mod statement;
pub use statement::StatementBuilder;

#[doc(hidden)]
mod principal;
pub use principal::PrincipalBuilder;

#[doc(hidden)]
mod action;
pub use action::ActionBuilder;

#[doc(hidden)]
mod resource;
pub use resource::ResourceBuilder;

#[doc(hidden)]
mod condition;
pub use condition::{ConditionBuilder, MatchBuilder};

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::io::write_to_writer;
    use std::io::stdout;

    #[test]
    fn test_simple_builder() {
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
        write_to_writer(stdout(), &policy).expect("well that was unexpected");
    }
}
