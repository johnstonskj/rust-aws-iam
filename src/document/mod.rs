/*!
* Provides the ability to produce formatted, readable, versions of a policy document.
*
* The intent of this module is to provide the ability to produce documentation that describes the
* behavior of a policy.
*
* # Example
*
* ```rust
* use aws_iam::{io, model::*, document};
* use std::path::PathBuf;
*
* let policy = io::read_from_file(
*         &PathBuf::from("tests/data/good/example-021.json")
*     ).expect("Error reading file");
*
* let mut generator = document::MarkdownGenerator::default();
*
* document::walk_policy(&policy, &mut generator);
* ```
*
* # Building a new Visitor
*
* TBD
*
*/

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

mod markdown;
pub use markdown::MarkdownGenerator;

mod latex;
pub use latex::LatexGenerator;

mod visitor;
pub use visitor::*;
