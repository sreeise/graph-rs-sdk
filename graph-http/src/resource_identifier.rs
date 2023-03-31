use crate::api_impl::GraphUrl;
use graph_core::resource::ResourceIdentity;

pub trait ResourceIdentifier {
    fn resource_identifier() -> ResourceIdentity;
}

/// Provides components for storing resource id's and helps build the current request URL.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ResourceConfig {
    pub resource_identity: ResourceIdentity,
    pub url: GraphUrl,
    pub resource_identity_id: Option<String>,
}

impl ResourceConfig {
    pub fn new(
        resource_identity: ResourceIdentity,
        url: GraphUrl,
        resource_identity_id: Option<String>,
    ) -> ResourceConfig {
        ResourceConfig {
            resource_identity,
            url,
            resource_identity_id,
        }
    }
}

impl ResourceConfig {
    pub fn resource_identity(&self) -> ResourceIdentity {
        self.resource_identity
    }
}

impl AsRef<GraphUrl> for ResourceConfig {
    fn as_ref(&self) -> &GraphUrl {
        &self.url
    }
}

impl AsMut<GraphUrl> for ResourceConfig {
    fn as_mut(&mut self) -> &mut GraphUrl {
        &mut self.url
    }
}
