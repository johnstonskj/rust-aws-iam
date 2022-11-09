use crate::model::{OrAny, Resource};
use aws_arn::ARN;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// A `Resource` builder, used with `StatementBuilder::resource()`.
///
#[derive(Clone, Debug)]
pub struct ResourceBuilder {
    not_resource: bool,
    resources: OrAny<Vec<ARN>>,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for ResourceBuilder {
    fn default() -> Self {
        Self {
            not_resource: false,
            resources: OrAny::Any,
        }
    }
}

impl From<ResourceBuilder> for Resource {
    fn from(builder: ResourceBuilder) -> Self {
        if builder.not_resource {
            Resource::NotResource(builder.resources)
        } else {
            Resource::Resource(builder.resources)
        }
    }
}

impl ResourceBuilder {
    pub fn any() -> Self {
        Self {
            not_resource: false,
            resources: OrAny::Any,
        }
    }

    pub fn none() -> Self {
        Self {
            not_resource: true,
            resources: OrAny::Any,
        }
    }

    pub fn any_of() -> Self {
        Self {
            not_resource: false,
            resources: OrAny::Any,
        }
    }

    pub fn none_of() -> Self {
        Self {
            not_resource: true,
            resources: OrAny::Any,
        }
    }

    /// Sets the action of this statement to be only this value.
    pub fn this(self, resource: ARN) -> Self {
        self.these(vec![resource]);
        self
    }

    /// Sets the action of this statement to be any of these values.
    pub fn these(self, resources: Vec<ARN>) -> Self {
        if let OrAny::Some(resource_vec) = self.resources {
            resource_vec.extend(resources);
        }
        self
    }
}
