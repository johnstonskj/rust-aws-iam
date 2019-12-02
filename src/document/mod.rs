/*!
Provides the ability to produce formatted, readable, versions of a policy document.

The intent of this module is to provide the ability to produce documentation that describes the
behavior of a policy. The intent was to provide documentation that uses a precise method to
describe the conditions under which a policy will either allow or deny an action. This is useful
when describing how policies may be used within a particular service, or describing template
policies.

# Example

```rust
use aws_iam::{io, model::*, document};
use std::path::PathBuf;

let policy = io::read_from_file(
        &PathBuf::from("tests/data/good/example-021.json")
    ).expect("Error reading file");

let mut generator = document::MarkdownGenerator::default();

document::visitor::walk_policy(&policy, &mut generator);
```

# Building a new Visitor

To build a new documentation tool, ot any tool that wishes to inspect the structure of a policy,
you can implement the traits within the [`visitor`](visitor/index.html) module and call them with
the [`walk_policy`](document/fn.walk_policy.html) function as in the example above. All of the
visitor traits have default implementations for their members and so  only those events you care
to handle need be implemented.

*/

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod markdown;
pub use markdown::MarkdownGenerator;

mod latex;
pub use latex::LatexGenerator;

pub mod visitor;
