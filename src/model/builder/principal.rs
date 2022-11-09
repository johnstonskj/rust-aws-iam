use aws_arn::ARN;

use crate::model::{CanonicalUserId, HostName, OrAny, Principal, PrincipalMap, ServiceName};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// A `Principal` builder, used with `StatementBuilder::principal()`.
///
#[derive(Clone, Debug)]
pub struct PrincipalBuilder {
    not_principal: bool,
    principals: OrAny<PrincipalMap>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for PrincipalBuilder {
    fn default() -> Self {
        Self {
            not_principal: false,
            principals: OrAny::Any,
        }
    }
}

impl From<PrincipalBuilder> for Principal {
    fn from(builder: PrincipalBuilder) -> Self {
        if builder.not_principal {
            Principal::NotPrincipal(builder.principals)
        } else {
            Principal::Principal(builder.principals)
        }
    }
}

impl PrincipalBuilder {
    pub fn any() -> Self {
        Self {
            not_principal: false,
            principals: OrAny::Any,
        }
    }

    pub fn any_of() -> Self {
        Self {
            not_principal: false,
            principals: OrAny::Some(Default::default()),
        }
    }

    pub fn none() -> Self {
        Self {
            not_principal: true,
            principals: OrAny::Any,
        }
    }

    pub fn none_of() -> Self {
        Self {
            not_principal: true,
            principals: OrAny::Some(Default::default()),
        }
    }

    /// Sets the **AWS** principal of this statement to be only this value.
    pub fn this_aws(self, principal: ARN) -> Self {
        self.these_aws(vec![principal])
    }

    /// Sets the **AWS** principal of this statement to be any of these values.
    pub fn these_aws(self, principals: Vec<ARN>) -> Self {
        if let OrAny::Some(principal_map) = self.principals {
            principal_map.extend_aws(principals)
        }
        self
    }

    /// Sets the **Federated** principal of this statement to be only this value.
    pub fn this_federated(self, principal: HostName) -> Self {
        self.these_federated(vec![principal])
    }

    /// Sets the **Federated** principal of this statement to be any of these values.
    pub fn these_federated(self, principals: Vec<HostName>) -> Self {
        if let OrAny::Some(principal_map) = self.principals {
            principal_map.extend_federated(principals)
        }
        self
    }

    /// Sets the **Service** principal of this statement to be only this value.
    pub fn this_service(self, principal: ServiceName) -> Self {
        self.these_service(vec![principal])
    }

    /// Sets the **Service** principal of this statement to be any of these values.
    pub fn these_service(self, principals: Vec<ServiceName>) -> Self {
        if let OrAny::Some(principal_map) = self.principals {
            principal_map.extend_services(principals)
        }
        self
    }

    /// Sets the **Canonical User** principal of this statement to be only this value.
    pub fn this_canonical_user(self, principal: CanonicalUserId) -> Self {
        self.these_canonical_user(vec![principal])
    }

    /// Sets the **Canonical User** principal of this statement to be any of these values.
    pub fn these_canonical_user(self, principals: Vec<CanonicalUserId>) -> Self {
        if let OrAny::Some(principal_map) = self.principals {
            principal_map.extend_canonical_users(principals)
        }
        self
    }
}
