use aws_arn::ARN;
use aws_iam::model::naming::CanonicalUserId;
use aws_iam::model::Principal;
use aws_iam::{model::MaybeAny, syntax::IamProperty};
use serde_json::{json, Map, Value};
use std::str::FromStr;

#[test]
fn test_any_principal_to_json() {
    let principal = Principal::new_any();

    let mut object: Map<String, Value> = Map::default();
    principal.into_json_object(&mut object).unwrap();

    assert_eq!(
        Value::Object(object),
        json!({
            "Principal": "*"
        })
    );
}

#[test]
fn test_none_principal_to_json() {
    let principal = Principal::new_none();

    let mut object: Map<String, Value> = Map::default();
    principal.into_json_object(&mut object).unwrap();

    assert_eq!(
        Value::Object(object),
        json!({
            "NotPrincipal": "*"
        })
    );
}

#[test]
fn test_example_to_json() {
    let mut principal = Principal::these_aws(vec![
        ARN::from_str("arn:aws:iam::123456789012:root").unwrap(),
        ARN::from_str("arn:aws:iam::999999999999:root").unwrap(),
    ]);
    principal.insert_canonical_user(
        CanonicalUserId::from_str(
            "79a59df900b949e55d96a1e698fbacedfd6e09d98eacf8f8d5218e7cd47ef2be",
        )
        .unwrap(),
    );

    let mut object: Map<String, Value> = Map::default();
    principal.into_json_object(&mut object).unwrap();

    assert_eq!(
        Value::Object(object),
        json!({
            "Principal": {
                "AWS": [
                    "arn:aws:iam::123456789012:root",
                    "arn:aws:iam::999999999999:root"
                ],
                "CanonicalUser": "79a59df900b949e55d96a1e698fbacedfd6e09d98eacf8f8d5218e7cd47ef2be"
            }
        })
    );
}

#[test]
fn test_example_from_json() {
    let json = json!({
            "Principal": {
                "AWS": [
                    "arn:aws:iam::123456789012:root",
                    "arn:aws:iam::999999999999:root"
                ],
                "CanonicalUser": "79a59df900b949e55d96a1e698fbacedfd6e09d98eacf8f8d5218e7cd47ef2be"
            }
    });

    if let Value::Object(object) = json {
        let result = Principal::from_json_object_optional(&object).unwrap();
        println!("{:#?}", result);
    } else {
        panic!("What no object?");
    }
}
