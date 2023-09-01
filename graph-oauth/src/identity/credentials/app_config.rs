use crate::identity::credentials::application_builder::AuthorityHost;
use crate::identity::Authority;
use reqwest::header::HeaderMap;
use std::collections::HashMap;
use url::Url;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct AppConfig {
    /// The directory tenant that you want to request permission from.
    /// This can be in GUID or friendly name format.
    /// If you don't know which tenant the user belongs to
    /// and you want to let them sign in with any tenant, use common.
    pub(crate) tenant_id: Option<String>,
    /// Required.
    /// The Application (client) ID that the Azure portal - App registrations page assigned
    /// to your app
    pub(crate) client_id: String,
    pub(crate) authority: Authority,
    pub(crate) authority_url: AuthorityHost,
    pub(crate) extra_query_parameters: HashMap<String, String>,
    pub(crate) extra_header_parameters: HeaderMap,
    /// Optional -  Some flows may require the redirect URI
    /// The redirect_uri of your app, where authentication responses can be sent and received
    /// by your app. It must exactly match one of the redirect_uris you registered in the portal,
    /// except it must be URL-encoded.
    pub(crate) redirect_uri: Option<Url>,
}

impl AppConfig {
    pub fn new() -> AppConfig {
        AppConfig {
            tenant_id: None,
            client_id: String::with_capacity(32),
            authority: Default::default(),
            authority_url: Default::default(),
            extra_query_parameters: Default::default(),
            extra_header_parameters: Default::default(),
            redirect_uri: None,
        }
    }

    pub(crate) fn new_with_client_id(client_id: impl AsRef<str>) -> AppConfig {
        AppConfig {
            tenant_id: None,
            client_id: client_id.as_ref().to_string(),
            authority: Default::default(),
            authority_url: Default::default(),
            extra_query_parameters: Default::default(),
            extra_header_parameters: Default::default(),
            redirect_uri: None,
        }
    }

    pub(crate) fn new_with_tenant_and_client_id(
        tenant_id: impl AsRef<str>,
        client_id: impl AsRef<str>,
    ) -> AppConfig {
        AppConfig {
            tenant_id: Some(tenant_id.as_ref().to_string()),
            client_id: client_id.as_ref().to_string(),
            authority: Authority::TenantId(tenant_id.as_ref().to_string()),
            authority_url: Default::default(),
            extra_query_parameters: Default::default(),
            extra_header_parameters: Default::default(),
            redirect_uri: None,
        }
    }
}
