/*!
One-line description.
More detailed description, with
# Example
 */

use thiserror::Error;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Error)]
pub enum IamError {
    #[error(transparent)]
    Format(#[from] IamFormatError),

    #[error(transparent)]
    Json(#[from] serde_json::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}

#[derive(Debug, Error)]
pub enum IamFormatError {
    #[error("A required property `{name}` was not found")]
    MissingProperty { name: String },

    #[error("Unexpected properties found for type `{type_name}`")]
    UnexpectedProperties { type_name: String },

    #[error("An unexpected value `{value}` for property named `{name}` was found")]
    UnexpectedValue { name: String, value: String },

    #[error("An unexpected value `{value}` for type `{type_name}` was found")]
    UnexpectedTypeValue { type_name: String, value: String },

    #[error("Invalid type for property `{name}`; expecting a `{expecting}` but found a `{found}`")]
    TypeMismatch {
        name: String,
        expecting: String,
        found: String,
    },

    #[error("The vector property `{name}` was found to be empty, it is required to have at least one value")]
    EmptyVector { name: String },

    #[error("Could not serialize a value to JSON")]
    CouldNotSerialize,

    #[error("Could not parse an ARN")]
    Arn(
        #[from]
        #[source]
        aws_arn::ArnError,
    ),

    #[error("Could not expand a variable in the value `{value}`")]
    InvalidVariable { value: String },
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn missing_property<S>(name: S) -> IamFormatError
where
    S: Into<String>,
{
    IamFormatError::MissingProperty { name: name.into() }
}

pub fn unexpected_properties<S>(type_name: S) -> IamFormatError
where
    S: Into<String>,
{
    IamFormatError::UnexpectedProperties {
        type_name: type_name.into(),
    }
}

pub fn unexpected_value_for_property<S1, S2>(name: S1, value: S2) -> IamFormatError
where
    S1: Into<String>,
    S2: Into<String>,
{
    IamFormatError::UnexpectedValue {
        name: name.into(),
        value: value.into(),
    }
}

pub fn unexpected_value_for_type<S1, S2>(type_name: S1, value: S2) -> IamFormatError
where
    S1: Into<String>,
    S2: Into<String>,
{
    IamFormatError::UnexpectedTypeValue {
        type_name: type_name.into(),
        value: value.into(),
    }
}

pub fn type_mismatch<S1, S2, S3>(name: S1, expecting: S2, found: S3) -> IamFormatError
where
    S1: Into<String>,
    S2: Into<String>,
    S3: Into<String>,
{
    IamFormatError::TypeMismatch {
        name: name.into(),
        expecting: expecting.into(),
        found: found.into(),
    }
}

pub fn empty_vector_property<S>(name: S) -> IamFormatError
where
    S: Into<String>,
{
    IamFormatError::EmptyVector { name: name.into() }
}

pub fn could_not_serialize() -> IamFormatError {
    IamFormatError::CouldNotSerialize
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl<T> From<IamError> for Result<T, IamError> {
    fn from(e: IamError) -> Self {
        Err(e)
    }
}

impl<T> From<IamFormatError> for Result<T, IamFormatError> {
    fn from(e: IamFormatError) -> Self {
        Err(e)
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
