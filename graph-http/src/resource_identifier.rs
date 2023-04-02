use graph_core::resource::ResourceIdentity;
use url::Url;

pub trait ResourceIdentifier {
    fn resource_identifier() -> ResourceIdentity;
}

/// Provides components for storing resource id's and helps build the current request URL.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResourceConfig {
    pub resource_identity: ResourceIdentity,
    pub url: Url,
    pub resource_identity_id: Option<String>,
}

impl ResourceConfig {
    pub fn new(
        resource_identity: ResourceIdentity,
        url: Url,
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

    pub fn extend_path<I: AsRef<str>>(&mut self, path: &[I]) {
        if let Ok(mut p) = self.url.path_segments_mut() {
            p.extend(path);
        }
    }
}

impl AsRef<Url> for ResourceConfig {
    fn as_ref(&self) -> &Url {
        &self.url
    }
}

impl AsMut<Url> for ResourceConfig {
    fn as_mut(&mut self) -> &mut Url {
        &mut self.url
    }
}
