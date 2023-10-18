use base64::Engine;
use std::collections::HashMap;

use reqwest::header::HeaderMap;
use url::Url;
use uuid::Uuid;

use crate::identity::{Authority, AzureCloudInstance};
use crate::oauth::ForceTokenRefresh;

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
    /// Specifies which Microsoft accounts can be used for sign-in with a given application.
    /// See https://aka.ms/msal-net-application-configuration
    pub(crate) authority: Authority,
    /// STS instance (for instance https://login.microsoftonline.com for the Azure public cloud).
    /// Maps to the instance url string.
    pub(crate) azure_cloud_instance: AzureCloudInstance,
    pub(crate) extra_query_parameters: HashMap<String, String>,
    pub(crate) extra_header_parameters: HeaderMap,
    /// Optional -  Some flows may require the redirect URI
    /// The redirect_uri of your app, where authentication responses can be sent and received
    /// by your app. It must exactly match one of the redirect_uris you registered in the portal,
    /// except it must be URL-encoded.
    pub(crate) redirect_uri: Option<Url>,
    /// Cache id used in a token cache store.
    pub(crate) cache_id: String,
    pub(crate) force_token_refresh: ForceTokenRefresh,
}

impl AppConfig {
    pub(crate) fn new() -> AppConfig {
        let client_id = Uuid::default();
        let cache_id =
            base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(client_id.to_string());

        AppConfig {
            tenant_id: None,
            client_id,
            authority: Default::default(),
            azure_cloud_instance: Default::default(),
            extra_query_parameters: Default::default(),
            extra_header_parameters: Default::default(),
            redirect_uri: None,
            cache_id,
            force_token_refresh: Default::default(),
        }
    }

    pub(crate) fn new_init(
        client_id: Uuid,
        tenant: Option<impl AsRef<str>>,
        redirect_uri: Option<Url>,
    ) -> AppConfig {
        let tenant_id: Option<String> = tenant.map(|value| value.as_ref().to_string());
        let cache_id = {
            if let Some(tenant_id) = tenant_id.as_ref() {
                base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(format!(
                    "{},{}",
                    tenant_id,
                    client_id.to_string()
                ))
            } else {
                base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(client_id.to_string())
            }
        };

        AppConfig {
            tenant_id,
            client_id,
            authority: Default::default(),
            azure_cloud_instance: Default::default(),
            extra_query_parameters: Default::default(),
            extra_header_parameters: Default::default(),
            redirect_uri,
            cache_id,
            force_token_refresh: Default::default(),
        }
    }

    pub(crate) fn new_with_client_id(client_id: impl AsRef<str>) -> AppConfig {
        let client_id = Uuid::try_parse(client_id.as_ref()).unwrap_or_default();
        let cache_id =
            base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(client_id.to_string());

        AppConfig {
            tenant_id: None,
            client_id,
            authority: Default::default(),
            azure_cloud_instance: Default::default(),
            extra_query_parameters: Default::default(),
            extra_header_parameters: Default::default(),
            redirect_uri: None,
            cache_id,
            force_token_refresh: Default::default(),
        }
    }

    pub(crate) fn new_with_tenant_and_client_id(
        tenant_id: impl AsRef<str>,
        client_id: impl AsRef<str>,
    ) -> AppConfig {
        let client_id = Uuid::try_parse(client_id.as_ref()).unwrap_or_default();
        let tenant_id = tenant_id.as_ref();
        let cache_id = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(format!(
            "{},{}",
            tenant_id,
            client_id.to_string()
        ));

        AppConfig {
            tenant_id: Some(tenant_id.to_string()),
            client_id,
            authority: Authority::TenantId(tenant_id.to_string()),
            azure_cloud_instance: Default::default(),
            extra_query_parameters: Default::default(),
            extra_header_parameters: Default::default(),
            redirect_uri: None,
            cache_id,
            force_token_refresh: Default::default(),
        }
    }
}
