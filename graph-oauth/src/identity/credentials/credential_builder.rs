macro_rules! credential_builder_impl {
    ($name:ident, $credential:ty) => {
        impl $name {
            pub fn with_client_id(&mut self, client_id: impl AsRef<str>) -> &mut Self {
                if self.credential.client_id.is_empty() {
                    self.credential.client_id.push_str(client_id.as_ref());
                } else {
                    self.credential.client_id = client_id.as_ref().to_owned();
                }
                self
            }

            /// Convenience method. Same as calling [with_authority(Authority::TenantId("tenant_id"))]
            pub fn with_tenant(&mut self, tenant: impl AsRef<str>) -> &mut Self {
                self.credential.authority = crate::identity::Authority::TenantId(tenant.as_ref().to_owned());
                self
            }

            pub fn with_authority<T: Into<crate::identity::Authority>>(&mut self, authority: T) -> &mut Self {
                self.credential.authority = authority.into();
                self
            }

            pub fn with_scope<T: ToString, I: IntoIterator<Item = T>>(
                &mut self,
                scope: I,
            ) -> &mut Self {
                self.credential.scope = scope.into_iter().map(|s| s.to_string()).collect();
                self
            }

            pub fn with_token_credential_options(
                &mut self,
                options: crate::identity::TokenCredentialOptions,
            ) -> &mut Self {
                self.credential.token_credential_options = options;
                self
            }

            pub fn build(&self) -> $credential {
                self.credential.clone()
            }
        }
    }
}
