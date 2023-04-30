use crate::identity::{
    AuthorizationSerializer, ResourceOwnerPasswordCredential, TokenCredentialOptions,
};
use reqwest::tls::Version;
use reqwest::ClientBuilder;

/// Clients incapable of maintaining the confidentiality of their credentials
/// (e.g., clients executing on the device used by the resource owner, such as an
/// installed native application or a web browser-based application), and incapable of
/// secure client authentication via any other means.
pub struct PublicClientApplication {
    http_client: reqwest::Client,
    token_credential_options: TokenCredentialOptions,
    credential: Box<dyn AuthorizationSerializer + Send>,
}

impl PublicClientApplication {
    pub fn new<T>(
        credential: T,
        options: TokenCredentialOptions,
    ) -> anyhow::Result<PublicClientApplication>
    where
        T: Into<PublicClientApplication>,
    {
        let mut public_client_application = credential.into();
        public_client_application.token_credential_options = options;
        Ok(public_client_application)
    }
}

impl From<ResourceOwnerPasswordCredential> for PublicClientApplication {
    fn from(value: ResourceOwnerPasswordCredential) -> Self {
        PublicClientApplication {
            http_client: ClientBuilder::new()
                .min_tls_version(Version::TLS_1_2)
                .https_only(true)
                .build()
                .unwrap(),
            token_credential_options: value.token_credential_options.clone(),
            credential: Box::new(value),
        }
    }
}
