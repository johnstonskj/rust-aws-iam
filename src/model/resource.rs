/*!
One-line description.
More detailed description, with
# Example
 */

use std::str::FromStr;

use crate::error::{missing_property, type_mismatch, unexpected_properties, IamFormatError};
use crate::model::{MaybeAny, OrAny};
use crate::syntax::{
    display_vec_to_json, from_json_str, json_type_name, IamProperty, IamValue,
    JSON_TYPE_NAME_STRING, POLICY_WILDCARD_VALUE, RESOURCE_NAME, RESOURCE_VALUE_NOT_RESOURCE,
    RESOURCE_VALUE_RESOURCE,
};
use aws_arn::ARN;
use serde_json::{Map, Value};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The Resource element specifies the object or objects that the statement covers. Statements
/// must include either a Resource or a NotResource element. You specify a resource using an ARN.
///
/// From [IAM JSON Policy Elements: Resource](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_elements_resource.html)
/// and [IAM JSON Policy Elements: NotResource](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_elements_notresource.html).
///
/// ## resource_string
///
/// In most cases, consists of an Amazon Resource Name (ARN).
///
/// ```text
/// "Resource":"arn:aws:iam::123456789012:user/Bob"
/// "Resource":"arn:aws:s3:::examplebucket/*"
/// ```
///
#[derive(Debug, Clone, PartialEq)]
pub enum Resource {
    /// Asserts that the resource in the request **must** match one of the specified ones.
    Resource(OrAny<Vec<ARN>>),
    /// Asserts that the resource in the request **must not** match one of the specified ones.
    NotResource(OrAny<Vec<ARN>>),
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for Resource {
    fn default() -> Self {
        Self::Resource(OrAny::Any)
    }
}

impl IamProperty for Resource {
    fn into_json_object(
        &self,
        object: &mut serde_json::Map<String, Value>,
    ) -> Result<(), IamFormatError> {
        let _ = match &self {
            Self::Resource(values) => {
                object.insert(RESOURCE_VALUE_RESOURCE.to_string(), values.to_json()?)
            }
            Self::NotResource(values) => {
                object.insert(RESOURCE_VALUE_NOT_RESOURCE.to_string(), values.to_json()?)
            }
        };
        Ok(())
    }

    fn from_json_object(value: &Map<String, Value>) -> Result<Self, IamFormatError>
    where
        Self: Sized,
    {
        match (
            value.get(RESOURCE_VALUE_RESOURCE),
            value.get(RESOURCE_VALUE_NOT_RESOURCE),
        ) {
            (Some(v), None) => Ok(Resource::Resource(OrAny::<Vec<ARN>>::from_json(v)?)),
            (None, Some(v)) => Ok(Resource::NotResource(OrAny::<Vec<ARN>>::from_json(v)?)),
            (None, None) => missing_property(RESOURCE_NAME).into(),
            (Some(_), Some(_)) => unexpected_properties(RESOURCE_NAME).into(),
        }
    }
}

impl Resource {
    pub fn any_resource() -> Self {
        Self::Resource(OrAny::Any)
    }

    pub fn this_resource(name: ARN) -> Self {
        Self::Resource(OrAny::Some(vec![name]))
    }

    pub fn these_resources(names: Vec<ARN>) -> Self {
        Self::Resource(OrAny::Some(names))
    }

    pub fn no_resource() -> Self {
        Self::NotResource(OrAny::Any)
    }

    pub fn not_this_resource(name: ARN) -> Self {
        Self::NotResource(OrAny::Some(vec![name]))
    }

    pub fn not_these_resources(names: Vec<ARN>) -> Self {
        Self::NotResource(OrAny::Some(names))
    }

    fn inner(&self) -> &OrAny<Vec<ARN>> {
        match self {
            Resource::Resource(v) => v,
            Resource::NotResource(v) => v,
        }
    }

    pub fn is_negative(&self) -> bool {
        matches!(self, Resource::NotResource(_))
    }

    pub fn is_any(&self) -> bool {
        matches!(self.inner(), OrAny::Any)
    }

    pub fn is_some(&self) -> bool {
        matches!(self.inner(), OrAny::Some(_))
    }

    pub fn some(&self) -> Option<&Vec<ARN>> {
        if let OrAny::Some(v) = self.inner() {
            Some(v)
        } else {
            None
        }
    }
}

impl MaybeAny<Vec<ARN>> for Resource {
    fn new_any() -> Self
    where
        Self: Sized,
    {
        Self::Resource(OrAny::Any)
    }

    fn new_none() -> Self
    where
        Self: Sized,
    {
        Self::NotResource(OrAny::Any)
    }

    fn inner(&self) -> &OrAny<Vec<ARN>> {
        match self {
            Resource::Resource(v) => v,
            Resource::NotResource(v) => v,
        }
    }

    fn is_negative(&self) -> bool {
        matches!(self, Resource::NotResource(_))
    }
}

// ------------------------------------------------------------------------------------------------

impl IamValue for OrAny<Vec<ARN>> {
    fn to_json(&self) -> Result<Value, IamFormatError> {
        Ok(if let OrAny::Some(values) = self {
            display_vec_to_json(values)?
        } else {
            Value::String(POLICY_WILDCARD_VALUE.to_string())
        })
    }

    fn from_json(value: &Value) -> Result<Self, IamFormatError>
    where
        Self: Sized,
    {
        if let Value::String(s) = value {
            if s == POLICY_WILDCARD_VALUE {
                Ok(OrAny::Any)
            } else {
                Ok(OrAny::Some(vec![ARN::from_str(s)?]))
            }
        } else if let Value::Array(arr) = value {
            let results: Result<Vec<ARN>, IamFormatError> = arr
                .iter()
                .map(|v| from_json_str(v, RESOURCE_NAME))
                .collect();
            Ok(OrAny::Some(results?))
        } else {
            type_mismatch(RESOURCE_NAME, JSON_TYPE_NAME_STRING, json_type_name(value)).into()
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::model::{OrAny, Resource};
    use crate::syntax::IamProperty;
    use aws_arn::ARN;
    use serde_json::{Map, Value};
    use std::str::FromStr;

    #[test]
    fn test_any_resource_into_json() {
        let mut statement = Map::default();

        let resource = Resource::any_resource();
        resource.into_json_object(&mut statement).unwrap();
        assert_eq!(
            format!("{:?}", statement),
            r##"{"Resource": String("*")}"##.to_string()
        );
    }

    #[test]
    fn test_this_resource_into_json() {
        let mut statement = Map::default();

        let resource =
            Resource::this_resource(ARN::from_str("arn:aws:s3:::examplebucket/*").unwrap());
        resource.into_json_object(&mut statement).unwrap();
        assert_eq!(
            format!("{:?}", statement),
            r##"{"Resource": String("arn:aws:s3:::examplebucket/*")}"##.to_string()
        );
    }

    #[test]
    fn test_these_resources_into_json() {
        let mut statement = Map::default();

        let resource = Resource::these_resources(vec![
            ARN::from_str("arn:aws:s3:::examplebucket/*").unwrap(),
            ARN::from_str("arn:aws:iam::123456789012:user/Bob").unwrap(),
        ]);
        resource.into_json_object(&mut statement).unwrap();
        assert_eq!(
            format!("{:?}", statement),
            r##"{"Resource": Array([String("arn:aws:s3:::examplebucket/*"), String("arn:aws:iam::123456789012:user/Bob")])}"##.to_string()
        );
    }

    #[test]
    fn test_no_resource_into_json() {
        let mut statement = Map::default();

        let resource = Resource::no_resource();
        resource.into_json_object(&mut statement).unwrap();
        assert_eq!(
            format!("{:?}", statement),
            r##"{"NotResource": String("*")}"##.to_string()
        );
    }

    #[test]
    fn test_not_this_resource_into_json() {
        let mut statement = Map::default();

        let resource =
            Resource::not_this_resource(ARN::from_str("arn:aws:s3:::examplebucket/*").unwrap());
        resource.into_json_object(&mut statement).unwrap();
        assert_eq!(
            format!("{:?}", statement),
            r##"{"NotResource": String("arn:aws:s3:::examplebucket/*")}"##.to_string()
        );
    }

    #[test]
    fn test_not_these_resources_into_json() {
        let mut statement = Map::default();

        let resource = Resource::not_these_resources(vec![
            ARN::from_str("arn:aws:s3:::examplebucket/*").unwrap(),
            ARN::from_str("arn:aws:iam::123456789012:user/Bob").unwrap(),
        ]);
        resource.into_json_object(&mut statement).unwrap();
        assert_eq!(
            format!("{:?}", statement),
            r##"{"NotResource": Array([String("arn:aws:s3:::examplebucket/*"), String("arn:aws:iam::123456789012:user/Bob")])}"##.to_string()
        );
    }

    #[test]
    fn test_wildcard_from_json() {
        let resource = Value::String("*".to_string());
        let mut container = Map::default();
        container.insert("Resource".to_string(), resource);

        let result = Resource::from_json_object(&container).unwrap();

        assert_eq!(result, Resource::Resource(OrAny::Any));
    }

    #[test]
    fn test_not_wildcard_from_json() {
        let resource = Value::String("*".to_string());
        let mut container = Map::default();
        container.insert("NotResource".to_string(), resource);

        let result = Resource::from_json_object(&container).unwrap();

        assert_eq!(result, Resource::NotResource(OrAny::Any));
    }

    #[test]
    #[should_panic]
    fn test_from_json_missing() {
        let value = Map::default();
        Resource::from_json_object(&value).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_from_json_both_keys() {
        let resource = Value::String("*".to_string());
        let mut container = Map::default();
        container.insert("Resource".to_string(), resource.clone());
        container.insert("NotResource".to_string(), resource);

        Resource::from_json_object(&container).unwrap();
    }

    #[test]
    fn test_one_name_from_json() {
        let resource = Value::String("arn:aws:s3:::examplebucket/*".to_string());
        let mut container = Map::default();
        container.insert("Resource".to_string(), resource);

        let result = Resource::from_json_object(&container).unwrap();

        assert_eq!(
            result,
            Resource::Resource(OrAny::Some(vec![ARN::from_str(
                "arn:aws:s3:::examplebucket/*"
            )
            .unwrap()]))
        );
    }

    #[test]
    fn test_name_vec_from_json() {
        let resource_1 = Value::String("arn:aws:s3:::examplebucket/*".to_string());
        let resource_2 = Value::String("arn:aws:iam::123456789012:user/Bob".to_string());
        let mut container = Map::default();
        container.insert(
            "Resource".to_string(),
            Value::Array(vec![resource_1, resource_2]),
        );

        let result = Resource::from_json_object(&container).unwrap();

        assert_eq!(
            result,
            Resource::Resource(OrAny::Some(vec![
                ARN::from_str("arn:aws:s3:::examplebucket/*").unwrap(),
                ARN::from_str("arn:aws:iam::123456789012:user/Bob").unwrap()
            ]))
        );
    }
}
