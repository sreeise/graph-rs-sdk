use crate::identity::{Authority, TokenCredentialOptions};

pub trait CredentialBuilder {
    type Credential;

    fn with_client_id(&mut self, client_id: impl AsRef<str>) -> &mut Self;
    fn with_tenant(&mut self, tenant: impl AsRef<str>) -> &mut Self;
    fn with_authority<T: Into<Authority>>(&mut self, authority: T) -> &mut Self;
    fn with_scope<T: ToString, I: IntoIterator<Item = T>>(&mut self, scopes: I) -> &mut Self;
    fn with_token_credential_options(&mut self, options: TokenCredentialOptions) -> &mut Self;
    fn build(&self) -> Self::Credential;
}

macro_rules! credential_builder_impl {
    ($name:ident, $credential:ty) => {
        impl CredentialBuilder for $name {
            type Credential = $credential;

            fn with_client_id(&mut self, client_id: impl AsRef<str>) -> &mut Self {
                if self.credential.client_id.is_empty() {
                    self.credential.client_id.push_str(client_id.as_ref());
                } else {
                    self.credential.client_id = client_id.as_ref().to_owned();
                }
                self
            }

            /// Convenience method. Same as calling [with_authority(Authority::TenantId("tenant_id"))]
            fn with_tenant(&mut self, tenant: impl AsRef<str>) -> &mut Self {
                self.credential.authority = Authority::TenantId(tenant.as_ref().to_owned());
                self
            }

            fn with_authority<T: Into<Authority>>(&mut self, authority: T) -> &mut Self {
                self.credential.authority = authority.into();
                self
            }

            fn with_scope<T: ToString, I: IntoIterator<Item = T>>(
                &mut self,
                scope: I,
            ) -> &mut Self {
                self.credential.scope = scope.into_iter().map(|s| s.to_string()).collect();
                self
            }

            fn with_token_credential_options(
                &mut self,
                options: TokenCredentialOptions,
            ) -> &mut Self {
                self.credential.token_credential_options = options;
                self
            }

            fn build(&self) -> $credential {
                self.credential.clone()
            }
        }
    };
}
