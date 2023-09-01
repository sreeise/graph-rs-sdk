macro_rules! credential_builder_base {
    ($name:ident) => {
        impl $name {
            pub fn with_client_id(&mut self, client_id: impl AsRef<str>) -> &mut Self {
                if self.credential.app_config.client_id.is_empty() {
                    self.credential
                        .app_config
                        .client_id
                        .push_str(client_id.as_ref());
                } else {
                    self.credential.app_config.client_id = client_id.as_ref().to_owned();
                }
                self
            }

            /// Convenience method. Same as calling [with_authority(Authority::TenantId("tenant_id"))]
            pub fn with_tenant(&mut self, tenant_id: impl AsRef<str>) -> &mut Self {
                self.credential.app_config.authority =
                    crate::identity::Authority::TenantId(tenant_id.as_ref().to_owned());
                self.credential.app_config.tenant_id = Some(tenant_id.as_ref().to_owned());
                self
            }

            pub fn with_authority<T: Into<crate::identity::Authority>>(
                &mut self,
                authority: T,
            ) -> &mut Self {
                self.credential.app_config.authority = authority.into();
                self
            }

            pub fn with_scope<T: ToString, I: IntoIterator<Item = T>>(
                &mut self,
                scope: I,
            ) -> &mut Self {
                self.credential.scope = scope.into_iter().map(|s| s.to_string()).collect();
                self
            }

            /// Extends the query parameters of both the default query params and user defined params.
            /// Does not overwrite default params.
            pub fn with_extra_query_param(&mut self, query_param: (String, String)) -> &mut Self {
                self.credential
                    .app_config
                    .extra_query_parameters
                    .insert(query_param.0, query_param.1);
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
                    .extra_query_parameters
                    .extend(query_parameters);
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
                    .extra_header_parameters
                    .insert(header_name.into(), header_value.into());
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
                    .extra_header_parameters
                    .extend(header_parameters);
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
