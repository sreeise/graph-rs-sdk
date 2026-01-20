use graph_core::resource::ResourceIdentity;
use graph_http::api_impl::ResourceConfig;
use url::Url;

/// Simple path renderer that replaces `{{RID}}` with a resource ID
/// and `{{key}}` patterns with values from a JSON object.
#[derive(Clone, Default)]
pub struct PathRenderer {
    rid: Option<String>,
}

impl PathRenderer {
    pub fn new() -> Self {
        Self { rid: None }
    }

    pub fn with_rid<S: Into<String>>(rid: S) -> Self {
        Self {
            rid: Some(rid.into()),
        }
    }

    /// Render a path template by replacing `{{RID}}` with the stored resource ID
    /// and `{{key}}` patterns with values from the params JSON object.
    pub fn render(&self, template: &str, params: &serde_json::Value) -> String {
        let mut result = template.to_string();

        // Replace {{RID}} with the resource ID if present
        if let Some(ref rid) = self.rid {
            result = result.replace("{{RID}}", rid);
        }

        // Replace {{key}} patterns with values from params
        if let Some(obj) = params.as_object() {
            for (key, value) in obj {
                let placeholder = format!("{{{{{}}}}}", key);
                if let Some(s) = value.as_str() {
                    result = result.replace(&placeholder, s);
                }
            }
        }

        result
    }
}

pub(crate) struct ResourceProvisioner;

impl ResourceProvisioner {
    pub(crate) fn resource_config_with_url(
        url: Url,
        resource_identity: ResourceIdentity,
    ) -> ResourceConfig {
        ResourceConfig::new(resource_identity, url, None)
    }

    pub(crate) fn resource_config_with_id_and_url<ID: Into<String>>(
        id: ID,
        url: Url,
        resource_identity: ResourceIdentity,
    ) -> ResourceConfig {
        ResourceConfig::new(resource_identity, url, Some(id.into()))
    }

    pub(crate) fn path_renderer_with_id<ID: ToString>(id: ID) -> PathRenderer {
        PathRenderer::with_rid(id.to_string())
    }

    pub(crate) fn config_and_renderer_with_id_and_url<ID: ToString>(
        id: ID,
        url: Url,
        resource_identity: ResourceIdentity,
    ) -> (ResourceConfig, PathRenderer) {
        (
            ResourceProvisioner::resource_config_with_id_and_url(
                id.to_string(),
                url,
                resource_identity,
            ),
            ResourceProvisioner::path_renderer_with_id(id.to_string()),
        )
    }
}

#[allow(unused_imports)]
mod tests {
    use super::{ResourceIdentity, ResourceProvisioner};
    use url::Url;

    #[test]
    fn resource_provisioner_graph_url() {
        let rp = ResourceProvisioner::resource_config_with_url(
            Url::parse(crate::GRAPH_URL_BETA).unwrap(),
            ResourceIdentity::Me,
        );
        assert_eq!(rp.url.as_str(), crate::GRAPH_URL_BETA);
    }

    #[test]
    fn resource_provisioner_custom_endpoint() {
        let rp = ResourceProvisioner::resource_config_with_url(
            Url::parse("https://localhost.com").unwrap(),
            ResourceIdentity::Me,
        );
        assert_eq!(rp.url.as_str(), "https://localhost.com/");
    }
}
