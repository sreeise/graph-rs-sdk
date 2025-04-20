use base64::Engine;
use http::{HeaderName, HeaderValue};
use std::collections::{BTreeSet, HashMap};
use std::fmt::{Debug, Formatter};

use crate::identity::{Authority, AzureCloudInstance, IdToken};
use crate::ApplicationOptions;
use graph_core::identity::ForceTokenRefresh;
use graph_error::AF;
use graph_http::api_impl::GraphClientConfiguration;
use reqwest::header::HeaderMap;
use url::Url;
use uuid::Uuid;

#[derive(Clone, Default)]
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
    /// Required -
    /// A space-separated list of scopes. You might also include
    /// other scopes in this request for requesting consent.
    ///
    /// For OpenID Connect, it must include the scope openid, which translates to the Sign you in
    /// permission in the consent UI. Openid scope is already included for [OpenIdCredential](crate::identity::OpenIdCredential)
    /// and for [OpenIdAuthorizationUrlParameters](crate::identity::OpenIdAuthorizationUrlParameters)
    ///
    /// For Client Credentials, The value passed for the scope parameter in this request should
    /// be the resource identifier (application ID URI) of the resource you want, affixed with
    /// the .default suffix. All scopes included must be for a single resource.
    /// Including scopes for multiple resources will result in an error.
    ///
    /// For the Microsoft Graph example, the value is https://graph.microsoft.com/.default.
    /// This value tells the Microsoft identity platform that of all the direct application
    /// permissions you have configured for your app, the endpoint should issue a token for the
    /// ones associated with the resource you want to use. To learn more about the /.default scope,
    /// see the [consent documentation](https://learn.microsoft.com/en-us/entra/identity-platform/permissions-consent-overview#the-default-scope).
    ///
    /// This https://graph.microsoft.com/.default scope is automatically set for
    /// [ClientCredentialsAuthorizationUrlParameters](crate::identity::ClientCredentialsAuthorizationUrlParameters),
    /// [ClientSecretCredential](crate::identity::ClientSecretCredential),
    /// [ClientCertificateCredential](crate::identity::ClientCertificateCredential),
    /// and [ClientAssertionCredential](crate::identity::ClientAssertionCredential).
    pub(crate) scope: BTreeSet<String>,
    /// Optional -  Some flows may require the redirect URI
    /// The redirect_uri of your app, where authentication responses can be sent and received
    /// by your app. It must exactly match one of the redirect_uris you registered in the portal,
    /// except it must be URL-encoded.
    pub(crate) redirect_uri: Option<Url>,
    /// Cache id used in a token cache store.
    pub(crate) cache_id: String,
    pub(crate) force_token_refresh: ForceTokenRefresh,
    pub(crate) id_token: Option<IdToken>,
    pub(crate) log_pii: bool,
    pub(crate) config: GraphClientConfiguration,
}

impl TryFrom<ApplicationOptions> for AppConfig {
    type Error = AF;

    fn try_from(value: ApplicationOptions) -> Result<Self, Self::Error> {
        let client_id = Uuid::try_parse(&value.client_id.to_string()).unwrap_or_default();
        let cache_id = AppConfig::generate_cache_id(client_id, value.tenant_id.as_ref());
        Ok(AppConfig {
            tenant_id: value.tenant_id,
            client_id: Uuid::try_parse(&value.client_id.to_string())?,
            authority: value
                .aad_authority_audience
                .map(Authority::from)
                .unwrap_or_default(),
            azure_cloud_instance: value.azure_cloud_instance.unwrap_or_default(),
            extra_query_parameters: Default::default(),
            extra_header_parameters: Default::default(),
            scope: Default::default(),
            redirect_uri: Some(
                Url::parse("http://localhost")
                    .map_err(|_| AF::msg_internal_err("redirect_uri"))
                    .unwrap(),
            ),
            cache_id,
            force_token_refresh: Default::default(),
            id_token: Default::default(),
            log_pii: false,
            config: Default::default(),
        })
    }
}

impl Debug for AppConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.log_pii {
            f.debug_struct("AppConfig")
                .field("tenant_id", &self.tenant_id)
                .field("client_id", &self.client_id)
                .field("authority", &self.authority)
                .field("azure_cloud_instance", &self.azure_cloud_instance)
                .field("extra_query_parameters", &self.extra_query_parameters)
                .field("extra_header_parameters", &self.extra_header_parameters)
                .field("scope", &self.scope)
                .field("force_token_refresh", &self.force_token_refresh)
                .finish()
        } else {
            f.debug_struct("AppConfig")
                .field(
                    "tenant_id",
                    &"[REDACTED] - call enable_pii_logging(true) to log value",
                )
                .field(
                    "client_id",
                    &"[REDACTED] - call enable_pii_logging(true) to log value",
                )
                .field("authority", &self.authority)
                .field("azure_cloud_instance", &self.azure_cloud_instance)
                .field("extra_query_parameters", &self.extra_query_parameters)
                .field(
                    "extra_header_parameters",
                    &"[REDACTED] - call enable_pii_logging(true) to log value",
                )
                .field("scope", &self.scope)
                .field("force_token_refresh", &self.force_token_refresh)
                .finish()
        }
    }
}

impl AppConfig {
    fn generate_cache_id(client_id: Uuid, tenant_id: Option<&String>) -> String {
        if let Some(tenant_id) = tenant_id.as_ref() {
            base64::engine::general_purpose::URL_SAFE_NO_PAD
                .encode(format!("{},{}", tenant_id, client_id))
        } else {
            base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(client_id.to_string())
        }
    }

    pub(crate) fn builder(client_id: impl TryInto<Uuid>) -> AppConfigBuilder {
        AppConfigBuilder::new(client_id)
    }

    pub(crate) fn new(client_id: impl TryInto<Uuid>) -> AppConfig {
        let client_id = client_id.try_into().unwrap_or_default();
        let cache_id = AppConfig::generate_cache_id(client_id, None);

        AppConfig {
            tenant_id: None,
            client_id,
            authority: Default::default(),
            azure_cloud_instance: Default::default(),
            extra_query_parameters: Default::default(),
            extra_header_parameters: Default::default(),
            scope: Default::default(),
            redirect_uri: Some(
                Url::parse("http://localhost")
                    .map_err(|_| AF::msg_internal_err("redirect_uri"))
                    .unwrap(),
            ),
            cache_id,
            force_token_refresh: Default::default(),
            id_token: Default::default(),
            log_pii: Default::default(),
            config: Default::default(),
        }
    }

    pub fn enable_pii_logging(&mut self, log_pii: bool) {
        self.log_pii = log_pii;
    }

    pub(crate) fn with_client_id(&mut self, client_id: impl TryInto<Uuid>) {
        self.client_id = client_id.try_into().unwrap_or_default();
    }

    pub(crate) fn with_authority(&mut self, authority: Authority) {
        if let Authority::TenantId(tenant_id) = &authority {
            self.tenant_id = Some(tenant_id.clone());
        }
        self.authority = authority;
    }

    pub(crate) fn with_azure_cloud_instance(&mut self, azure_cloud_instance: AzureCloudInstance) {
        self.azure_cloud_instance = azure_cloud_instance;
    }

    pub(crate) fn with_tenant(&mut self, tenant_id: impl AsRef<str>) {
        let tenant = tenant_id.as_ref().to_string();
        self.tenant_id = Some(tenant.clone());
        self.authority = Authority::TenantId(tenant);
    }

    /// Extends the query parameters of both the default query params and user defined params.
    /// Does not overwrite default params.
    pub(crate) fn with_extra_query_param(&mut self, query_param: (String, String)) {
        self.extra_query_parameters
            .insert(query_param.0, query_param.1);
    }

    /// Extends the query parameters of both the default query params and user defined params.
    /// Does not overwrite default params.
    pub(crate) fn with_extra_query_parameters(
        &mut self,
        query_parameters: HashMap<String, String>,
    ) {
        self.extra_query_parameters.extend(query_parameters);
    }

    /// Extends the header parameters of both the default header params and user defined params.
    /// Does not overwrite default params.
    pub(crate) fn with_extra_header_param<K: Into<HeaderName>, V: Into<HeaderValue>>(
        &mut self,
        header_name: K,
        header_value: V,
    ) {
        self.extra_header_parameters
            .insert(header_name.into(), header_value.into());
    }

    /// Extends the header parameters of both the default header params and user defined params.
    /// Does not overwrite default params.
    pub(crate) fn with_extra_header_parameters(&mut self, header_parameters: HeaderMap) {
        self.extra_header_parameters.extend(header_parameters);
    }

    pub(crate) fn with_scope<T: ToString, I: IntoIterator<Item = T>>(&mut self, scope: I) {
        self.scope = scope.into_iter().map(|s| s.to_string()).collect();
    }

    pub(crate) fn with_id_token(&mut self, id_token: IdToken) {
        self.id_token = Some(id_token);
    }

    pub(crate) fn with_config(&mut self, config: GraphClientConfiguration) {
        self.config = config;
    }
}

#[derive(Clone, Default)]
pub struct AppConfigBuilder {
    app_config: AppConfig,
}

impl AppConfigBuilder {
    pub fn new(client_id: impl TryInto<Uuid>) -> AppConfigBuilder {
        AppConfigBuilder {
            app_config: AppConfig::new(client_id),
        }
    }

    pub fn tenant(mut self, tenant: impl Into<String>) -> Self {
        let tenant_id = tenant.into();
        self.app_config.tenant_id = Some(tenant_id.clone());
        self.authority(Authority::TenantId(tenant_id))
    }

    pub fn redirect_uri(mut self, redirect_uri: Url) -> Self {
        self.app_config.redirect_uri = Some(redirect_uri);
        self
    }

    pub fn redirect_uri_option(mut self, redirect_uri: Option<Url>) -> Self {
        self.app_config.redirect_uri = redirect_uri;
        self
    }

    pub fn authority(mut self, authority: Authority) -> Self {
        self.app_config.authority = authority;
        self
    }

    pub fn scope<T: ToString, I: IntoIterator<Item = T>>(mut self, scope: I) -> Self {
        self.app_config.scope = scope.into_iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn graph_client_configuration(mut self, config: GraphClientConfiguration) -> Self {
        self.app_config.config = config;
        self
    }

    pub fn build(mut self) -> AppConfig {
        if self.app_config.redirect_uri.is_none() {
            self.app_config.redirect_uri = Some(
                Url::parse("http://localhost")
                    .map_err(|_| AF::msg_internal_err("redirect_uri"))
                    .unwrap(),
            );
        }
        self.app_config
    }
}
