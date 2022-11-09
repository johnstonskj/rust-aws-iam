use aws_iam::model::Version;
use aws_iam::syntax::IamValue;
use std::str::FromStr;

#[test]
fn test_version_display() {
    assert_eq!(Version::V2012.to_string(), "2012-10-17".to_string());
    assert_eq!(Version::V2008.to_string(), "2008-10-17".to_string());
}

#[test]
fn test_version_from_str_ok() {
    assert_eq!(Version::from_str("2012-10-17").unwrap(), Version::V2012);
    assert_eq!(Version::from_str("2008-10-17").unwrap(), Version::V2008);
}

#[test]
fn test_version_from_str_err() {
    if let Err(e) = Version::from_str("2022-06-27") {
        assert_eq!(
            e.to_string(),
            "An unexpected value `2022-06-27` for property named `Version` was found".to_string()
        );
    } else {
        panic!("should have failed");
    }
}

#[test]
fn test_version_to_json() {
    assert_eq!(Version::V2012.to_json().unwrap(), "2012-10-17".to_string());
    assert_eq!(Version::V2008.to_json().unwrap(), "2008-10-17".to_string());
}
