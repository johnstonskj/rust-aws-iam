use super::StatementBuilder;
use crate::model::{Policy, Statement, Version};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The top-level `Policy` builder.
///
#[derive(Debug, Default)]
pub struct PolicyBuilder {
    version: Option<Version>,
    id: Option<String>,
    statements: Vec<Statement>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl From<PolicyBuilder> for Policy {
    fn from(builder: PolicyBuilder) -> Self {
        match (builder.id, builder.version) {
            (None, None) => Policy::unnamed(builder.statements),
            (None, Some(version)) => Policy::unnamed_with_version(builder.statements, version),
            (Some(id), None) => Policy::named(id, builder.statements),
            (Some(id), Some(version)) => {
                Policy::named_with_version(id, builder.statements, version)
            }
        }
        .expect("Could not create new Policy")
    }
}

impl PolicyBuilder {
    /// Set the version of this policy.
    pub fn for_version(self, version: Version) -> Self {
        self.version = Some(version);
        self
    }

    /// Set the id of this policy
    pub fn named<S>(self, id: S) -> Self
    where
        S: Into<String>,
    {
        self.id = Some(id.into());
        self
    }

    /// Set the id of this policy to a randomly generate value.
    pub fn auto_name(self) -> Self {
        self.id = Some(random_id());
        self
    }

    /// Add a statement to this policy.
    pub fn evaluate(self, statement: StatementBuilder) -> Self {
        self.statements.push(statement.into());
        self
    }

    /// Add a list of statements to this policy.
    pub fn evaluate_all(self, statements: Vec<StatementBuilder>) -> Self {
        let statements: Vec<Statement> = statements.into_iter().map(|sb| sb.into()).collect();
        self.statements.extend(statements);
        self
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

#[inline]
fn random_id() -> String {
    let id = uuid::Uuid::new_v4();
    id.to_string()
}
