use crate::identity::AzureCloudInstance;
use graph_error::IdentityResult;
use url::Url;

pub trait AuthorizationUrl {
    fn redirect_uri(&self) -> Option<&Url>;
    fn authorization_url(&self) -> IdentityResult<Url>;
    fn authorization_url_with_host(
        &self,
        azure_cloud_instance: &AzureCloudInstance,
    ) -> IdentityResult<Url>;
}
