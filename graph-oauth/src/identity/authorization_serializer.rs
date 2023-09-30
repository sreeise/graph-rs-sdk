use crate::identity::AzureCloudInstance;
use graph_error::IdentityResult;
use std::collections::HashMap;
use url::Url;

pub trait AuthorizationSerializer {
    fn uri(&mut self, azure_cloud_instance: &AzureCloudInstance) -> IdentityResult<Url>;
    fn form_urlencode(&mut self) -> IdentityResult<HashMap<String, String>>;
    fn basic_auth(&self) -> Option<(String, String)> {
        None
    }
}

pub trait AuthorizationUrl {
    fn redirect_uri(&self) -> Option<&Url>;
    fn authorization_url(&self) -> IdentityResult<Url>;
    fn authorization_url_with_host(
        &self,
        azure_cloud_instance: &AzureCloudInstance,
    ) -> IdentityResult<Url>;
}
