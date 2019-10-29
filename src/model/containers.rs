use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// A container used by a number of elements where the JSON serialization may be  a single
/// string,  or an array of string values.
///
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OneOrAll<T> {
    /// A single statement.
    One(T),
    /// A vector of statements.
    All(Vec<T>),
}

///
/// A container used by a number of elements where the JSON serialization may be a wild-card
/// value, a single string,  or an array of string values.
///
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OneOrAny<T: Clone = String>
where
    T: Clone,
{
    /// The wildcard value, may be Any or All depending on use.
    #[serde(rename = "*")]
    Any,
    /// One element of type `T`
    One(T),
    /// A JSON array with elements of type `T`.
    AnyOf(Vec<T>),
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl<T> OneOrAny<T>
where
    T: Clone,
{
    pub fn is_any(&self) -> bool {
        match self {
            OneOrAny::Any => true,
            _ => false,
        }
    }

    pub fn is_one(&self) -> bool {
        match self {
            OneOrAny::One(_) => true,
            _ => false,
        }
    }

    pub fn is_any_of(&self) -> bool {
        match self {
            OneOrAny::AnyOf(_) => true,
            _ => false,
        }
    }

    pub fn one(&self) -> Option<T> {
        match self {
            OneOrAny::One(value) => Some(value.clone()),
            _ => None,
        }
    }

    pub fn any_of(&self) -> Option<Vec<T>> {
        match self {
            OneOrAny::AnyOf(values) => Some(values.clone()),
            _ => None,
        }
    }
}
