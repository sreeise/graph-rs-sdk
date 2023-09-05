use crate::identity::{Authority, AzureCloudInstance};
use reqwest::header::HeaderMap;
use std::collections::HashMap;
use url::Url;
use uuid::Uuid;

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
    pub(crate) client_id: Uuid,
    pub(crate) authority: Authority,
    pub(crate) azure_cloud_instance: AzureCloudInstance,
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
            client_id: Uuid::default(),
            authority: Default::default(),
            azure_cloud_instance: Default::default(),
            extra_query_parameters: Default::default(),
            extra_header_parameters: Default::default(),
            redirect_uri: None,
        }
    }

    pub(crate) fn new_with_client_id(client_id: impl AsRef<str>) -> AppConfig {
        AppConfig {
            tenant_id: None,
            client_id: Uuid::try_parse(client_id.as_ref()).unwrap_or_default(),
            authority: Default::default(),
            azure_cloud_instance: Default::default(),
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
            client_id: Uuid::try_parse(client_id.as_ref()).unwrap_or_default(),
            authority: Authority::TenantId(tenant_id.as_ref().to_string()),
            azure_cloud_instance: Default::default(),
            extra_query_parameters: Default::default(),
            extra_header_parameters: Default::default(),
            redirect_uri: None,
        }
    }
}
