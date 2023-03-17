use handlebars::{Context, Handlebars, Helper, HelperResult, Output, RenderContext};

use graph_core::resource::ResourceIdentity;
use graph_http::url::GraphUrl;
use graph_http::ResourceConfig;

pub(crate) struct ResourceProvisioner;

impl ResourceProvisioner {
    pub(crate) fn resource_config_with_url(
        url: GraphUrl,
        resource_identity: ResourceIdentity,
    ) -> ResourceConfig {
        ResourceConfig::new(resource_identity, url, None)
    }

    pub(crate) fn resource_config_with_id_and_url<ID: Into<String>>(
        id: ID,
        url: GraphUrl,
        resource_identity: ResourceIdentity,
    ) -> ResourceConfig {
        ResourceConfig::new(resource_identity, url, Some(id.into()))
    }

    pub(crate) fn registry_with_id<ID: ToString>(id: ID) -> Handlebars {
        let mut registry = Handlebars::new();
        let id_owned = id.to_string();
        registry.register_helper(
            "RID",
            Box::new(
                move |_: &Helper,
                      _: &Handlebars,
                      _: &Context,
                      _: &mut RenderContext,
                      out: &mut dyn Output|
                      -> HelperResult {
                    out.write(&id_owned)?;
                    Ok(())
                },
            ),
        );
        registry
    }

    pub(crate) fn config_and_registry_with_id_and_url<ID: ToString>(
        id: ID,
        url: GraphUrl,
        resource_identity: ResourceIdentity,
    ) -> (ResourceConfig, Handlebars) {
        (
            ResourceProvisioner::resource_config_with_id_and_url(
                id.to_string(),
                url,
                resource_identity,
            ),
            ResourceProvisioner::registry_with_id(id.to_string()),
        )
    }
}

mod tests {
    use super::*;
    use crate::{GRAPH_URL, GRAPH_URL_BETA};

    #[test]
    fn resource_provisioner_graph_url() {
        let rp = ResourceProvisioner::resource_config_with_url(
            GraphUrl::parse(GRAPH_URL_BETA).unwrap(),
            ResourceIdentity::Me,
        );
        assert_eq!(rp.url.as_str(), GRAPH_URL_BETA);
    }

    #[test]
    fn resource_provisioner_custom_endpoint() {
        let rp = ResourceProvisioner::resource_config_with_url(
            GraphUrl::parse("https://localhost.com").unwrap(),
            ResourceIdentity::Me,
        );
        assert_eq!(rp.url.as_str(), "https://localhost.com/");
    }
}
