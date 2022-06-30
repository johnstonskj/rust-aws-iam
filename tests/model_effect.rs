use aws_iam::model::Effect;
use aws_iam::syntax::IamValue;
use serde_json::Value;
use std::str::FromStr;

#[test]
fn effect_display() {
    assert_eq!(Effect::Allow.to_string(), "Allow".to_string());
    assert_eq!(Effect::Deny.to_string(), "Deny".to_string());
}

#[test]
fn effect_from_str_ok() {
    assert_eq!(Effect::from_str("Allow").unwrap(), Effect::Allow);
    assert_eq!(Effect::from_str("Deny").unwrap(), Effect::Deny);
}

#[test]
fn effect_from_str_err() {
    if let Err(e) = Effect::from_str("allow") {
        assert_eq!(
            e.to_string(),
            "An unexpected value `allow` for property named `Effect` was found".to_string()
        );
    } else {
        panic!("should have failed");
    }
}

#[test]
fn effect_to_json() {
    assert_eq!(
        Effect::Allow.to_json().unwrap(),
        Value::String("Allow".to_string())
    );
    assert_eq!(
        Effect::Deny.to_json().unwrap(),
        Value::String("Deny".to_string())
    );
}
