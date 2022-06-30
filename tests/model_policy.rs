use aws_iam::model::{Policy, Statement, Version};
use aws_iam::syntax::IamValue;
use serde_json::json;

#[test]
fn test_simple_policy_to_json() {
    let policy = Policy::unnamed(vec![Statement::unnamed()]).unwrap();
    let object = policy.to_json().unwrap();

    assert_eq!(
        object,
        json!({
          "Statement": [
            {
              "Action": "*",
              "Effect": "Deny",
              "Resource": "*"
            }
          ]
        })
    );
}

#[test]
fn test_named_policy_to_json() {
    let policy = Policy::named("SomePolicyName", vec![Statement::unnamed()])
        .unwrap()
        .for_version(Version::V2012);
    let object = policy.to_json().unwrap();

    assert_eq!(
        object,
        json!({
          "Id": "SomePolicyName",
          "Statement": [
            {
              "Action": "*",
              "Effect": "Deny",
              "Resource": "*"
            }
          ],
          "Version": "2012-10-17"
        })
    );
}

#[test]
fn test_example_policy_from_json() {
    let json = json!({
    "Version": "2012-10-17",
    "Statement": [
      {
        "Sid": "UsePrincipalArnInsteadOfNotPrincipalWithDeny",
        "Effect": "Deny",
        "Action": "s3:*",
        "Principal": "*",
        "Resource": [
          "arn:aws:s3:::BUCKETNAME/*",
          "arn:aws:s3:::BUCKETNAME"
        ],
        "Condition": {
          "ArnNotEquals": {
            "aws:PrincipalArn": "arn:aws:iam::444455556666:user/user-name"
          }
        }
      }
    ]
          });

    let policy = Policy::from_json(&json);

    println!("{:#?}", policy);
}
