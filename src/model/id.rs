/// https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_iam-quotas.html
/// The external ID value that a third party uses to assume a role must
/// have a minimum of 2 characters and a maximum of 1,224 characters. The
/// value must be alphanumeric without white space. It can also include the
/// following symbols: plus (+), equal (=), comma (,), period (.), at (@),
/// colon (:), forward slash (/), and hyphen (-). For more information
/// about the external ID, see How to use an external ID when granting
/// access to your AWS resources to a third party.
#[inline]
pub fn is_valid_external_id<S>(s: S) -> bool
where
    S: Into<String>,
{
    let s = s.into();
    s.len() >= 2
        && s.len() <= 1224
        && s.chars().any(|c| {
            c.is_ascii_alphanumeric() || ['+', '=', ',', '.', '@', ':', '/', '-'].contains(&c)
        })
}

#[inline]
pub fn new_external_id() -> String {
    uuid::Uuid::new_v4().to_string()
}
