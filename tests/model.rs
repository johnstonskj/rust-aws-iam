use aws_iam::model::builder::*;
use aws_iam::model::*;
use std::collections::HashMap;

fn serialize_and_check(policy: &Policy, expected: &str) {
    println!("Policy: {:#?}", policy);
    let json = serde_json::to_string(&policy).unwrap();
    println!("{:#?}", json);
    let expected = expected.to_string();
    assert_eq!(json, expected);
    let parsed: Result<Policy, serde_json::Error> = serde_json::from_str(&json);
    println!("Parsed: {:#?}", parsed);
    assert!(parsed.is_ok());
}

#[test]
fn test_simple_access_policy() {
    /* From https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies.html#access_policies-json
    {
      "Version": "2012-10-17",
      "Statement": {
        "Effect": "Allow",
        "Action": "s3:ListBucket",
        "Resource": "arn:aws:s3:::example_bucket"
      }
    }
    */
    let expected = "{\"Version\":\"2012-10-17\",\"Id\":\"test_simple_access_policy\",\"Statement\":{\"Effect\":\"Allow\",\"Action\":\"s3:ListBucket\",\"Resource\":\"arn:aws:s3:::example_bucket\"}}";
    let policy = Policy {
        version: Some(Version::V2012),
        id: Some("test_simple_access_policy".to_string()),
        statement: Statements::One(Statement::new(
            Effect::Allow,
            Action::Action(this("s3:ListBucket")),
            Resource::Resource(this("arn:aws:s3:::example_bucket")),
        )),
    };
    serialize_and_check(&policy, expected);
}

#[test]
fn test_access_policy_with_statements() {
    /* From https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies.html#access_policies-json
    {
      "Version": "2012-10-17",
      "Statement": {
        "Effect": "Allow",
        "Action": "s3:ListBucket",
        "Resource": "arn:aws:s3:::example_bucket"
      }
    }
    */
    let expected = "{\"Version\":\"2012-10-17\",\"Id\":\"test_access_policy_with_statements\",\"Statement\":[{\"Effect\":\"Allow\",\"Action\":\"s3:ListBucket\",\"Resource\":\"arn:aws:s3:::example_bucket\"},{\"Effect\":\"Allow\",\"Action\":\"s3:SomethingElse\",\"Resource\":\"arn:aws:s3:::example_bucket_2\"}]}";
    let policy = Policy {
        version: Some(Version::V2012),
        id: Some("test_access_policy_with_statements".to_string()),
        statement: Statements::All(vec![
            Statement {
                sid: None,
                principal: None,
                effect: Effect::Allow,
                action: Action::Action(this("s3:ListBucket")),
                resource: Resource::Resource(this("arn:aws:s3:::example_bucket")),
                condition: None,
            },
            Statement {
                sid: None,
                principal: None,
                effect: Effect::Allow,
                action: Action::Action(this("s3:SomethingElse")),
                resource: Resource::Resource(this("arn:aws:s3:::example_bucket_2")),
                condition: None,
            },
        ]),
    };
    serialize_and_check(&policy, expected);
}

#[test]
fn test_access_policy_with_principal() {
    /* From https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies.html#access_policies-json
    {
      "Version": "2012-10-17",
      "Id": "S3-Account-Permissions",
      "Statement": [{
        "Sid": "1",
        "Effect": "Allow",
        "Principal": {"AWS": ["arn:aws:iam::ACCOUNT-ID-WITHOUT-HYPHENS:root"]},
        "Action": "s3:*",
        "Resource": [
          "arn:aws:s3:::mybucket",
          "arn:aws:s3:::mybucket/ *"
        ]
      }]
    }
    */
    let expected = "{\"Version\":\"2012-10-17\",\"Id\":\"test_access_policy_with_principal\",\"Statement\":[{\"Sid\":\"1\",\"Principal\":{\"AWS\":[\"arn:aws:iam::ACCOUNT-ID-WITHOUT-HYPHENS:root\"]},\"Effect\":\"Allow\",\"Action\":\"s3:*\",\"Resource\":[\"arn:aws:s3:::mybucket\",\"arn:aws:s3:::mybucket/*\"]}]}";
    let principal: HashMap<PrincipalType, Qualified> = vec![(
        PrincipalType::AWS,
        any_of(vec!["arn:aws:iam::ACCOUNT-ID-WITHOUT-HYPHENS:root"]),
    )]
    .iter()
    .cloned()
    .collect();
    let policy = Policy {
        version: Some(Version::V2012),
        id: Some("test_access_policy_with_principal".to_string()),
        statement: Statements::All(vec![Statement {
            sid: Some("1".to_string()),
            principal: Some(Principal::Principal(principal)),
            effect: Effect::Allow,
            action: Action::Action(this("s3:*")),
            resource: Resource::Resource(any_of(vec![
                "arn:aws:s3:::mybucket",
                "arn:aws:s3:::mybucket/*",
            ])),
            condition: None,
        }]),
    };
    serialize_and_check(&policy, expected);
}

#[test]
fn test_access_policy_with_condition() {
    /* From https://docs.aws.amazon.com/IAM/latest/UserGuide/access_policies.html#access_policies-json
    {
      "Version": "2012-10-17",
      "Statement": [
        ...
        {
          "Sid": "ThirdStatement",
          "Effect": "Allow",
          "Action": [
            "s3:List*",
            "s3:Get*"
          ],
          "Resource": [
            "arn:aws:s3:::confidential-data",
            "arn:aws:s3:::confidential-data/ *"
          ],
          "Condition": {"Bool": {"aws:MultiFactorAuthPresent": "true"}}
        }
      ]
    }
    */
    let expected = "{\"Version\":\"2012-10-17\",\"Id\":\"test_access_policy_with_condition\",\"Statement\":[{\"Sid\":\"ThirdStatement\",\"Effect\":\"Allow\",\"Action\":[\"s3:List*\",\"s3:Get*\"],\"Resource\":[\"arn:aws:s3:::confidential-data\",\"arn:aws:s3:::confidential-data/*\"],\"Condition\":{\"Bool\":{\"aws:MultiFactorAuthPresent\":\"true\"}}}]}";
    let mut condition: HashMap<ConditionType, HashMap<String, ConditionValues>> = HashMap::new();
    condition_one(
        &mut condition,
        ConditionType::new(BaseConditionType::Bool),
        "aws:MultiFactorAuthPresent".to_string(),
        "true".to_string(),
    );
    let policy = Policy {
        version: Some(Version::V2012),
        id: Some("test_access_policy_with_condition".to_string()),
        statement: Statements::All(vec![Statement {
            sid: Some("ThirdStatement".to_string()),
            principal: None,
            effect: Effect::Allow,
            action: Action::Action(any_of(vec!["s3:List*", "s3:Get*"])),
            resource: Resource::Resource(any_of(vec![
                "arn:aws:s3:::confidential-data",
                "arn:aws:s3:::confidential-data/*",
            ])),
            condition: Some(condition),
        }]),
    };
    serialize_and_check(&policy, expected);
}

#[test]
fn test_deserialize() {
    let raw_string = r#"{
    "Version": "2012-10-17",
    "Statement": [
        {
            "Effect": "Allow",
            "Action": [
                "aws-portal:ViewBilling",
                "aws-portal:ViewPaymentMethods",
                "aws-portal:ModifyPaymentMethods",
                "aws-portal:ViewAccount",
                "aws-portal:ModifyAccount",
                "aws-portal:ViewUsage"
            ],
            "Resource": [
                "*"
            ],
            "Condition": {
                "IpAddress": {
                    "aws:SourceIp": "203.0.113.0/24"
                }
            }
        },
        {
            "Effect": "Deny",
            "Action": [
                "s3:*"
            ],
            "Resource": [
                "arn:aws:s3:::customer",
                "arn:aws:s3:::customer/*"
            ]
        },
        {
            "Effect": "Allow",
            "Action": [
                "ec2:GetConsoleScreenshots"
            ],
            "Resource": [
                "*"
            ]
        },
        {
            "Effect": "Allow",
            "Action": [
                "codedploy:*",
                "codecommit:*"
            ],
            "Resource": [
                "arn:aws:codedeploy:us-west-2:123456789012:deploymentgroup:*",
                "arn:aws:codebuild:us-east-1:123456789012:project/my-demo-project"
            ]
        },
        {
            "Effect": "Allow",
            "Action": [
                "s3:ListAllMyBuckets",
                "s3:GetObject",
                "s3:DeletObject",
                "s3:PutObject",
                "s3:PutObjectAcl"
            ],
            "Resource": [
                "arn:aws:s3:::developer_bucket",
                "arn:aws:s3:::developer_bucket/*",
                "arn:aws:autoscling:us-east-2:123456789012:autoscalgrp"
            ],
            "Condition": {
                "StringEquals": {
                    "s3:x-amz-acl": [
                        "public-read"
                    ],
                    "s3:prefix": [
                        "custom",
                        "other"
                    ]
                }
            }
        }
    ]
}"#;
    let policy: Result<Policy, serde_json::Error> = serde_json::from_str(raw_string);
    println!("Policy: {:#?}", policy);
    assert!(policy.is_ok());
}
