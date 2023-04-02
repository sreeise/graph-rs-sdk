use graph_core::resource::ResourceIdentity;
use graph_http::api_impl::{GraphUrl, ResourceConfig};
use handlebars::{Context, Handlebars, Helper, HelperResult, Output, RenderContext};
use url::Url;

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
        url: Url,
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

#[allow(unused_imports)]
mod tests {
    use super::{GraphUrl, ResourceIdentity, ResourceProvisioner};
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
