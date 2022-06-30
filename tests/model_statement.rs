use aws_iam::model::Statement;
use aws_iam::syntax::IamValue;
use serde_json::Value;

#[test]
fn test_simple_statement_to_json() {
    let statement = Statement::unnamed();
    let object = statement.to_json().unwrap();
    let obj_str = format!("{:?}", object);

    assert!(obj_str.starts_with(r##"Object({"##));
    assert!(obj_str.contains(r##""Effect": String("Deny")"##));
    assert!(obj_str.contains(r##""Action": String("*")"##));
    assert!(obj_str.contains(r##""Resource": String("*")"##));
    assert!(obj_str.ends_with(r##"})"##));

    println!("{}", serde_json::to_string_pretty(&object).unwrap());
}

#[test]
fn test_named_statement_to_json() {
    let statement = Statement::named("sid-001");
    let object = statement.to_json().unwrap();
    let obj_str = format!("{:?}", object);

    assert!(obj_str.starts_with(r##"Object({"##));
    assert!(obj_str.contains(r##""Effect": String("Deny")"##));
    assert!(obj_str.contains(r##""Sid": String("sid-001")"##));
    assert!(obj_str.contains(r##""Action": String("*")"##));
    assert!(obj_str.contains(r##""Resource": String("*")"##));
    assert!(obj_str.ends_with(r##"})"##));

    println!("{}", serde_json::to_string_pretty(&object).unwrap());
}

#[test]
fn test_from_json_str() {
    const JSON: &str = r##"{
  "Effect": "Allow",
  "Action": [
    "s3:ListAllMyBuckets",
    "s3:GetBucketLocation"
  ],
  "Resource": "arn:aws:s3:::*"
}"##;
    let value: Value = serde_json::from_str(JSON).unwrap();
    let statement = Statement::from_json(&value).unwrap();
    println!("{:?}", statement);
}
