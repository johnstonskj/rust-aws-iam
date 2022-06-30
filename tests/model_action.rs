use aws_iam::model::{Action, MaybeAny, OrAny, QualifiedName};
use aws_iam::syntax::IamProperty;
use serde_json::{json, Map, Value};
use std::str::FromStr;

#[test]
fn test_any_action_into_json() {
    let mut statement = Map::default();

    let action = Action::new_any();
    action.into_json_object(&mut statement).unwrap();

    assert_eq!(
        Value::Object(statement),
        json!({
            "Action": "*"
        })
    );
}

#[test]
fn test_this_action_into_json() {
    let mut statement = Map::default();

    let action = Action::this_action(QualifiedName::from_str("s3:Get*").unwrap());
    action.into_json_object(&mut statement).unwrap();

    assert_eq!(
        Value::Object(statement),
        json!({
            "Action": "s3:Get*"
        })
    );
}

#[test]
fn test_these_actions_into_json() {
    let mut statement = Map::default();

    let action = Action::these_actions(vec![
        QualifiedName::from_str("s3:Get*").unwrap(),
        QualifiedName::from_str("s3:Put*").unwrap(),
    ]);
    action.into_json_object(&mut statement).unwrap();

    assert_eq!(
        Value::Object(statement),
        json!({
            "Action": [
                "s3:Get*",
                "s3:Put*"
            ]
        })
    );
}

#[test]
fn test_no_action_into_json() {
    let mut statement = Map::default();

    let action = Action::new_none();
    action.into_json_object(&mut statement).unwrap();

    assert_eq!(
        Value::Object(statement),
        json!({
            "NotAction": "*"
        })
    );
}

#[test]
fn test_not_this_action_into_json() {
    let mut statement = Map::default();

    let action = Action::not_this_action(QualifiedName::from_str("s3:Get*").unwrap());
    action.into_json_object(&mut statement).unwrap();

    assert_eq!(
        Value::Object(statement),
        json!({
            "NotAction": "s3:Get*"
        })
    );
}

#[test]
fn test_not_these_actions_into_json() {
    let mut statement = Map::default();

    let action = Action::not_these_actions(vec![
        QualifiedName::from_str("s3:Get*").unwrap(),
        QualifiedName::from_str("s3:Put*").unwrap(),
    ]);
    action.into_json_object(&mut statement).unwrap();

    assert_eq!(
        Value::Object(statement),
        json!({
            "NotAction": [
                "s3:Get*",
                "s3:Put*"
            ]
        })
    );
}

#[test]
fn test_wildcard_from_json() {
    let action = Value::String("*".to_string());
    let mut container = Map::default();
    container.insert("Action".to_string(), action);

    let result = Action::from_json_object(&container).unwrap();

    assert_eq!(result, Action::Action(OrAny::Any));
}

#[test]
fn test_not_wildcard_from_json() {
    let action = Value::String("*".to_string());
    let mut container = Map::default();
    container.insert("NotAction".to_string(), action);

    let result = Action::from_json_object(&container).unwrap();

    assert_eq!(result, Action::NotAction(OrAny::Any));
}

#[test]
#[should_panic]
fn test_from_json_missing() {
    let value = Map::default();
    Action::from_json_object(&value).unwrap();
}

#[test]
#[should_panic]
fn test_from_json_both_keys() {
    let action = Value::String("*".to_string());
    let mut container = Map::default();
    container.insert("Action".to_string(), action.clone());
    container.insert("NotAction".to_string(), action);

    Action::from_json_object(&container).unwrap();
}

#[test]
fn test_one_name_from_json() {
    let action = Value::String("ec2:StartInstances".to_string());
    let mut container = Map::default();
    container.insert("Action".to_string(), action);

    let result = Action::from_json_object(&container).unwrap();

    assert_eq!(
        result,
        Action::Action(OrAny::Some(vec![QualifiedName::from_str(
            "ec2:StartInstances"
        )
        .unwrap()]))
    );
}

#[test]
fn test_name_vec_from_json() {
    let action_1 = Value::String("ec2:StartInstances".to_string());
    let action_2 = Value::String("ec2:StopInstances".to_string());
    let mut container = Map::default();
    container.insert("Action".to_string(), Value::Array(vec![action_1, action_2]));

    let result = Action::from_json_object(&container).unwrap();

    assert_eq!(
        result,
        Action::Action(OrAny::Some(vec![
            QualifiedName::from_str("ec2:StartInstances").unwrap(),
            QualifiedName::from_str("ec2:StopInstances").unwrap()
        ]))
    );
}
