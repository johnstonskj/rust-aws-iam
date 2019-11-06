use std::collections::HashMap;

pub fn all_templates() -> HashMap<String, String> {
    [
        (
            "s3",
            r#"{
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
    }"#,
        ),
        (
            "mfa",
            r#"{
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
    }"#,
        ),
        (
            "iam",
            r#"{
  "Version": "2012-10-17",
  "Statement": [ {
    "Effect": "Allow",
    "Action": [
      "iam:GenerateCredentialReport",
      "iam:Get*",
      "iam:List*"
    ],
    "Resource": "*"
  } ]
}"#,
        ),
    ]
    .iter()
    .map(|(k, v)| (k.to_string(), v.to_string()))
    .collect()
}
