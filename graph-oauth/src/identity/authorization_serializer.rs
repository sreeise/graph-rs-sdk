use crate::identity::AzureCloudInstance;
use graph_error::AuthorizationResult;
use std::collections::HashMap;
use url::Url;

pub trait AuthorizationSerializer {
    fn uri(&mut self, azure_authority_host: &AzureCloudInstance) -> AuthorizationResult<Url>;
    fn form_urlencode(&mut self) -> AuthorizationResult<HashMap<String, String>>;
    fn basic_auth(&self) -> Option<(String, String)> {
        None
    }
}

pub trait AuthorizationUrl {
    fn redirect_uri(&self) -> AuthorizationResult<Url>;
    fn authorization_url(&self) -> AuthorizationResult<Url>;
    fn authorization_url_with_host(
        &self,
        azure_authority_host: &AzureCloudInstance,
    ) -> AuthorizationResult<Url>;
}
