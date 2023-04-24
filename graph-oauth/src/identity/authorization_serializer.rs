use crate::identity::AzureAuthorityHost;
use graph_error::AuthorizationResult;
use std::collections::HashMap;
use url::Url;

pub trait AuthorizationSerializer {
    fn uri(&mut self, azure_authority_host: &AzureAuthorityHost) -> AuthorizationResult<Url>;
    fn form(&mut self) -> AuthorizationResult<HashMap<String, String>>;
}
