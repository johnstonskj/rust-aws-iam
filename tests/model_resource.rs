use aws_arn::ARN;
use aws_iam::model::{OrAny, Resource};
use aws_iam::syntax::IamProperty;
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

    let resource = Resource::this_resource(ARN::from_str("arn:aws:s3:::examplebucket/*").unwrap());
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
