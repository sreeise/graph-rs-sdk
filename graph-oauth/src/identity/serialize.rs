use crate::identity::AzureAuthorityHost;
use graph_error::{AuthorizationResult, GraphResult};
use std::collections::HashMap;
use url::Url;

pub trait AuthorizationSerializer {
    fn uri(&mut self, azure_authority_host: &AzureAuthorityHost) -> GraphResult<Url>;
    fn form(&mut self) -> AuthorizationResult<HashMap<String, String>>;
}
