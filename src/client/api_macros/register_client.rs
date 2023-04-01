macro_rules! resource_identifier_impl {
    ($api_client:ident, $resource_identity:expr) => {
        impl ResourceIdentifier for $api_client {
            fn resource_identifier() -> ResourceIdentity {
                $resource_identity
            }
        }
    };
}

macro_rules! resource_api_client {
    ($name:ident) => {
        pub struct $name {
            pub(crate) client: graph_http::api_impl::Client,
            pub(crate) resource_config: graph_http::api_impl::ResourceConfig,
            registry: handlebars::Handlebars,
        }

        impl $name {
            pub(crate) fn new(
                client: graph_http::api_impl::Client,
                resource_config: graph_http::api_impl::ResourceConfig,
                registry: handlebars::Handlebars,
            ) -> $name {
                $name {
                    client,
                    resource_config,
                    registry,
                }
            }
        }

        impl ApiClientImpl for $name {
            fn url(&self) -> GraphUrl {
                self.resource_config.url.clone()
            }

            fn render_path<S: AsRef<str>>(
                &self,
                path: S,
                path_params_map: &serde_json::Value,
            ) -> GraphResult<String> {
                self.registry
                    .render_template(path.as_ref(), path_params_map)
                    .map_err(GraphFailure::from)
            }
        }

        impl ODataQuery for $name {
            fn append_query_pair<KV: AsRef<str>>(mut self, key: KV, value: KV) -> Self {
                self.resource_config.url.append_query_pair(key, value);
                self
            }
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!($name))
                    .field("resource_config", &self.resource_config)
                    .finish()
            }
        }
    };

    ($name:ident, $resource_identity:expr) => {
        resource_api_client!($name);
        resource_identifier_impl!($name, $resource_identity);
    };

    ($name:ident, $name2:ident, $resource_identity:expr) => {
        resource_api_client!($name);
        resource_api_client!($name2);
        resource_identifier_impl!($name, $resource_identity);
        resource_identifier_impl!($name2, $resource_identity);

        impl $name {
            resource_id_tunnel!($name2);
        }
    };
}
