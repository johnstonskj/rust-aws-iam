use aws_iam::model::{QualifiedName, ServiceName};
use std::str::FromStr;

#[test]
fn test_service_name_plain() {
    ServiceName::from_str("www.amazon.com").unwrap();
    ServiceName::from_str("ecs.amazonaws.com").unwrap();
}

#[test]
fn test_service_name_errors() {
    assert!(ServiceName::from_str("").is_err());
    assert!(ServiceName::from_str(".").is_err());
    assert!(ServiceName::from_str("*").is_err());
    assert!(ServiceName::from_str("amazon").is_err());
    assert!(ServiceName::from_str("ecs.amazon*aws.com").is_err());
}

#[test]
fn test_qname_plain() {
    QualifiedName::from_str("ns:name").unwrap();
    QualifiedName::from_str("ns1:name").unwrap();
    QualifiedName::from_str("aws:name99").unwrap();
    QualifiedName::from_str("aws:name-99").unwrap();
}

#[test]
fn test_qname_errors() {
    assert!(QualifiedName::from_str("").is_err());
    assert!(QualifiedName::from_str(":").is_err());
    assert!(QualifiedName::from_str(":name").is_err());
    assert!(QualifiedName::from_str("aws:").is_err());
    assert!(QualifiedName::from_str("aws:foo_bar").is_err());
    assert!(QualifiedName::from_str("a?s:valid").is_err());
}

#[test]
fn test_qname_tagged() {
    QualifiedName::from_str("ns:name/foo").unwrap();
    QualifiedName::from_str("ns:name/Foo").unwrap();
    QualifiedName::from_str("ns:name/f99").unwrap();
    QualifiedName::from_str("ns:name/f-99").unwrap();
}

#[test]
fn test_qname_wildcards() {
    QualifiedName::from_str("aws:name*").unwrap();
    QualifiedName::from_str("aws:*name").unwrap();
    QualifiedName::from_str("aws:name-v??").unwrap();
    QualifiedName::from_str("ns:name/?oo").unwrap();
    QualifiedName::from_str("ns:name/foo*").unwrap();
    QualifiedName::from_str("ns:name/?oo*").unwrap();
}

#[test]
fn test_qname_parts() {
    let qname = QualifiedName::from_str("aws:name").unwrap();
    assert_eq!(qname.namespace(), "aws");
    assert_eq!(qname.name(), "name");
    assert_eq!(qname.tag(), None);
    assert!(!qname.has_wildcard());

    let qname = QualifiedName::from_str("aws:name*/tag").unwrap();
    assert_eq!(qname.namespace(), "aws");
    assert_eq!(qname.name(), "name*");
    assert_eq!(qname.tag(), Some("tag"));
    assert!(qname.has_wildcard());
}

#[test]
fn test_qname_with_name() {
    let qname = QualifiedName::from_str("aws:name").unwrap();
    assert_eq!(
        qname.with_name("foo").unwrap(),
        QualifiedName::new_unchecked("aws:foo")
    );

    let qname = QualifiedName::from_str("aws:name").unwrap();
    assert_eq!(
        qname.with_name("foo/").unwrap(),
        QualifiedName::new_unchecked("aws:foo/")
    );

    let qname = QualifiedName::from_str("aws:name/bar").unwrap();
    assert_eq!(
        qname.with_name("foo").unwrap(),
        QualifiedName::new_unchecked("aws:foo")
    );

    let qname = QualifiedName::from_str("aws:name/bar").unwrap();
    assert_eq!(
        qname.with_name("foo/").unwrap(),
        QualifiedName::new_unchecked("aws:foo/bar")
    );
}
