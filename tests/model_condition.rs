use aws_iam::context::keys::AWS_RESOURCE_TAG;
use aws_iam::model::{Condition, Operator, QualifiedName};
use aws_iam::syntax::IamProperty;
use serde_json::Map;
use std::str::FromStr;

#[test]
fn condition_operator_to_string() {
    let c = Operator::from_str("StringEquals").unwrap();
    assert_eq!(c, Operator::string_equals());

    let c = Operator::from_str("StringEqualsIfExists").unwrap();
    let mut c2 = Operator::string_equals();
    c2.set_if_exists();
    assert_eq!(c, c2);

    let c = Operator::from_str("ForAllValues:StringEquals").unwrap();
    let mut c2 = Operator::string_equals();
    c2.set_for_all();
    assert_eq!(c, c2);
}

#[test]
fn condition_operator_from_str() {
    let c = Operator::from_str("StringEquals").unwrap();
    assert_eq!(c, Operator::string_equals());

    let c = Operator::from_str("StringEqualsIfExists").unwrap();
    let mut c2 = Operator::string_equals();
    c2.set_if_exists();
    assert_eq!(c, c2);

    let c = Operator::from_str("ForAllValues:StringEquals").unwrap();
    let mut c2 = Operator::string_equals();
    c2.set_for_all();
    assert_eq!(c, c2);
}

#[test]
fn condition_to_json() {
    let c = Condition::new_one(
        Operator::string_equals(),
        QualifiedName::from_str(AWS_RESOURCE_TAG).unwrap(),
        "test",
    );
    println!("1: {:?}", c);

    let mut json = Map::default();
    let _ = c.into_json_object(&mut json);
    println!("2: {:?}", json);
}
