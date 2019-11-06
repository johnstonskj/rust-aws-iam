/*!
Provides the ability to produce formatted, readable, versions of a policy document.

TBD

# Example Usage

```rust,ignore
let generator = MarkdownGenerator::default();
report::walk_policy(&policy, &generator, &mut stdout());
```

# Building a new Visitor

TBD

*/

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod markdown;
pub use markdown::*;

mod latex;
pub use latex::*;

mod visitor;
pub use visitor::*;