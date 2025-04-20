macro_rules! credential_builder_base {
    ($name:ident) => {
        impl $name {
            pub fn with_client_id(&mut self, client_id: impl TryInto<uuid::Uuid>) -> &mut Self {
                self.credential.app_config.with_client_id(client_id);
                self
            }

            /// Convenience method. Same as calling [with_authority(Authority::TenantId("tenant_id"))]
            pub fn with_tenant(&mut self, tenant_id: impl AsRef<str>) -> &mut Self {
                self.credential.app_config.with_tenant(tenant_id);
                self
            }

            pub fn with_authority(
                &mut self,
                authority: impl Into<crate::identity::Authority>,
            ) -> &mut Self {
                self.credential.app_config.with_authority(authority.into());
                self
            }

            pub fn with_azure_cloud_instance(
                &mut self,
                azure_cloud_instance: crate::identity::AzureCloudInstance,
            ) -> &mut Self {
                self.credential
                    .app_config
                    .with_azure_cloud_instance(azure_cloud_instance);
                self
            }

            /// Extends the query parameters of both the default query params and user defined params.
            /// Does not overwrite default params.
            pub fn with_extra_query_param(&mut self, query_param: (String, String)) -> &mut Self {
                self.credential
                    .app_config
                    .with_extra_query_param(query_param);
                self
            }

            /// Extends the query parameters of both the default query params and user defined params.
            /// Does not overwrite default params.
            pub fn with_extra_query_parameters(
                &mut self,
                query_parameters: HashMap<String, String>,
            ) -> &mut Self {
                self.credential
                    .app_config
                    .with_extra_query_parameters(query_parameters);
                self
            }

            /// Extends the header parameters of both the default header params and user defined params.
            /// Does not overwrite default params.
            pub fn with_extra_header_param<K: Into<HeaderName>, V: Into<HeaderValue>>(
                &mut self,
                header_name: K,
                header_value: V,
            ) -> &mut Self {
                self.credential
                    .app_config
                    .with_extra_header_param(header_name, header_value);
                self
            }

            /// Extends the header parameters of both the default header params and user defined params.
            /// Does not overwrite default params.
            pub fn with_extra_header_parameters(
                &mut self,
                header_parameters: HeaderMap,
            ) -> &mut Self {
                self.credential
                    .app_config
                    .with_extra_header_parameters(header_parameters);
                self
            }

            pub fn with_scope<T: ToString, I: IntoIterator<Item = T>>(
                &mut self,
                scope: I,
            ) -> &mut Self {
                self.credential.app_config.with_scope(scope);
                self
            }

            pub fn with_config(
                &mut self,
                config: &graph_http::api_impl::GraphClientConfiguration,
            ) -> &mut Self {
                self.credential.app_config.with_config(config.clone());
                self
            }
        }
    };
}

macro_rules! credential_builder {
    ($name:ident, $client:ty) => {
        credential_builder_base!($name);

        impl $name {
            pub fn build(&self) -> $client {
                <$client>::new(self.credential.clone())
            }
        }
    };
}
