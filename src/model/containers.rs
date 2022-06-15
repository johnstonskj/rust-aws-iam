/*!
Provides some basic container enums that are used by the Policy model.
*/

use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::iter::FromIterator;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// A container used by a number of elements where the JSON serialization may be  a single
/// string,  or an array of string values.
///
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OneOrAll<T = String> {
    /// A single statement.
    One(T),
    /// A vector of statements.
    All(Vec<T>),
}

///
/// A container used by a number of elements where the JSON serialization may be a wild-card
/// value, a single string,  or an array of string values.
///
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OneOrAny<T = String> {
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

impl<T> From<T> for OneOrAll<T> {
    fn from(v: T) -> Self {
        Self::One(v)
    }
}

impl<T> From<Vec<T>> for OneOrAll<T> {
    fn from(vs: Vec<T>) -> Self {
        Self::All(vs)
    }
}

impl<T> FromIterator<T> for OneOrAll<T> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> OneOrAll<T> {
        Self::All(Vec::from_iter(iter.into_iter()))
    }
}

impl<T: Clone> Clone for OneOrAll<T> {
    #[inline]
    fn clone(&self) -> Self {
        match self {
            Self::One(v) => Self::One(v.clone()),
            Self::All(vs) => Self::All(vs.clone()),
        }
    }
}

impl<T: PartialEq> PartialEq for OneOrAll<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::One(lhs), Self::One(rhs)) => lhs.eq(rhs),
            (Self::All(lhs), Self::All(rhs)) => lhs.eq(rhs),
            _ => false,
        }
    }
}

impl<T: Eq> Eq for OneOrAll<T> {}

impl<T: PartialOrd> PartialOrd for OneOrAll<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Self::One(lhs), Self::One(rhs)) => lhs.partial_cmp(rhs),
            (Self::All(lhs), Self::All(rhs)) => lhs.partial_cmp(rhs),
            (Self::One(lhs), Self::All(rhs)) => {
                vec![lhs].partial_cmp(&rhs.iter().collect::<Vec<&T>>())
            }
            (Self::All(lhs), Self::One(rhs)) => {
                lhs.iter().collect::<Vec<&T>>().partial_cmp(&vec![rhs])
            }
        }
    }
}

impl<T: Ord> Ord for OneOrAll<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::One(lhs), Self::One(rhs)) => lhs.cmp(rhs),
            (Self::All(lhs), Self::All(rhs)) => lhs.cmp(rhs),
            (Self::One(lhs), Self::All(rhs)) => vec![lhs].cmp(&rhs.iter().collect::<Vec<&T>>()),
            (Self::All(lhs), Self::One(rhs)) => lhs.iter().collect::<Vec<&T>>().cmp(&vec![rhs]),
        }
    }
}

impl<T: Hash> Hash for OneOrAll<T> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::One(v) => Hash::hash(v, state),
            Self::All(vs) => Hash::hash(vs, state),
        }
    }
}

impl<T> OneOrAll<T> {
    ///
    /// Returns `true` if the option is an `One` value.
    ///
    pub fn is_one(&self) -> bool {
        matches!(self, OneOrAll::One(_))
    }

    ///
    /// Returns `true` if the option is an `All` value.
    ///
    pub fn is_all(&self) -> bool {
        matches!(self, OneOrAll::All(_))
    }

    ///
    /// Converts from OneOrAll<T> to Option<T>.
    ///
    /// Converts `self` into an `Option<T>`, consuming `self`, and discarding `All` values.
    ///
    pub fn one(self) -> Option<T> {
        match self {
            OneOrAll::One(value) => Some(value),
            _ => None,
        }
    }

    ///
    /// Converts from OneOrAll<T> to Option<T>.
    ///
    /// Converts `self` into an `Option<T>`, consuming `self`, and `One` values.
    ///
    pub fn all(self) -> Option<Vec<T>> {
        match self {
            OneOrAll::All(values) => Some(values),
            _ => None,
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl<T> From<T> for OneOrAny<T> {
    fn from(v: T) -> Self {
        Self::One(v)
    }
}

impl<T> From<Vec<T>> for OneOrAny<T> {
    fn from(vs: Vec<T>) -> Self {
        Self::AnyOf(vs)
    }
}

impl<T> FromIterator<T> for OneOrAny<T> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> OneOrAny<T> {
        Self::AnyOf(Vec::from_iter(iter.into_iter()))
    }
}

impl<T: Clone> Clone for OneOrAny<T> {
    #[inline]
    fn clone(&self) -> Self {
        match self {
            Self::Any => self.clone(),
            Self::One(v) => Self::One(v.clone()),
            Self::AnyOf(vs) => Self::AnyOf(vs.clone()),
        }
    }
}

impl<T: PartialEq> PartialEq for OneOrAny<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Any, Self::Any) => true,
            (Self::One(lhs), Self::One(rhs)) => lhs.eq(rhs),
            (Self::AnyOf(lhs), Self::AnyOf(rhs)) => lhs.eq(rhs),
            _ => false,
        }
    }
}

impl<T: Eq> Eq for OneOrAny<T> {}

//
// **Note**: There is no implementation of PartialOrd or Ord here as it's
// really not clear how you would order the value `Any` against the inner
// type.
//

impl<T: Hash> Hash for OneOrAny<T> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Any => Hash::hash(self, state),
            Self::One(v) => Hash::hash(v, state),
            Self::AnyOf(vs) => Hash::hash(vs, state),
        }
    }
}

impl<T> OneOrAny<T> {
    ///
    /// Returns `true` if the option is an `Any` value.
    ///
    pub fn is_any(&self) -> bool {
        matches!(self, OneOrAny::Any)
    }

    ///
    /// Returns `true` if the option is an `One` value.
    ///
    pub fn is_one(&self) -> bool {
        matches!(self, OneOrAny::One(_))
    }

    ///
    /// Returns `true` if the option is an `AnyOf` value.
    ///
    pub fn is_any_of(&self) -> bool {
        matches!(self, OneOrAny::AnyOf(_))
    }

    ///
    /// Converts from OneOrAny<T> to Option<T>.
    ///
    /// Converts `self` into an `Option<T>`, consuming `self`, and discarding either `Any` or
    /// `AnyOf` values.
    ///
    pub fn one(self) -> Option<T> {
        match self {
            OneOrAny::One(value) => Some(value),
            _ => None,
        }
    }

    ///
    /// Converts from OneOrAny<T> to Option<T>.
    ///
    /// Converts `self` into an `Option<T>`, consuming `self`, and discarding either `Any` or
    /// `One` values.
    ///
    pub fn any_of(self) -> Option<Vec<T>> {
        match self {
            OneOrAny::AnyOf(values) => Some(values),
            _ => None,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_or_all_from_one() {
        let v: OneOrAll<i32> = OneOrAll::from(10);
        assert!(v.is_one());
        assert_eq!(v.one(), Some(10));
    }

    #[test]
    fn test_one_or_all_from_all() {
        let v: OneOrAll<i32> = OneOrAll::from(vec![10, 9, 8]);
        assert!(v.is_all());
        assert_eq!(v.all(), Some(vec![10, 9, 8]));
    }

    #[test]
    fn test_one_or_all_se_one() {
        let v: OneOrAll<i32> = OneOrAll::from(10);
        let s = serde_json::to_string(&v).expect("Could not serialize");
        assert_eq!(s, "10");
    }

    #[test]
    fn test_one_or_all_se_all() {
        let v: OneOrAll<i32> = OneOrAll::from(vec![10, 9, 8]);
        let s = serde_json::to_string(&v).expect("Could not serialize");
        assert_eq!(s, "[10,9,8]");
    }

    // --------------------------------------------------------------------------------------------

    #[test]
    fn test_one_or_any_any() {
        let v: OneOrAny<i32> = OneOrAny::Any;
        assert!(v.is_any());
    }

    #[test]
    fn test_one_or_any_from_one() {
        let v: OneOrAny<i32> = OneOrAny::from(10);
        assert!(v.is_one());
        assert_eq!(v.one(), Some(10));
    }

    #[test]
    fn test_one_or_any_from_all() {
        let v: OneOrAny<i32> = OneOrAny::from(vec![10, 9, 8]);
        assert!(v.is_any_of());
        assert_eq!(v.any_of(), Some(vec![10, 9, 8]));
    }

    #[test]
    #[ignore]
    fn test_one_or_any_se_any() {
        let v: OneOrAny<i32> = OneOrAny::Any;
        let s = serde_json::to_string(&v).expect("Could not serialize");
        assert_eq!(s, "*");
    }

    #[test]
    fn test_one_or_any_se_one() {
        let v: OneOrAny<i32> = OneOrAny::from(10);
        let s = serde_json::to_string(&v).expect("Could not serialize");
        assert_eq!(s, "10");
    }

    #[test]
    fn test_one_or_any_se_all() {
        let v: OneOrAny<i32> = OneOrAny::from(vec![10, 9, 8]);
        let s = serde_json::to_string(&v).expect("Could not serialize");
        assert_eq!(s, "[10,9,8]");
    }
}
