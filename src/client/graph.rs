use crate::admin::AdminApiClient;
use crate::agreement_acceptances::{
    AgreementAcceptancesApiClient, AgreementAcceptancesIdApiClient,
};
use crate::agreements::{AgreementsApiClient, AgreementsIdApiClient};
use crate::api_default_imports::*;
use crate::app_catalogs::AppCatalogsApiClient;
use crate::applications::{ApplicationsApiClient, ApplicationsIdApiClient};
use crate::audit_logs::AuditLogsApiClient;
use crate::authentication_method_configurations::{
    AuthenticationMethodConfigurationsApiClient, AuthenticationMethodConfigurationsIdApiClient,
};
use crate::authentication_methods_policy::AuthenticationMethodsPolicyApiClient;
use crate::batch::BatchApiClient;
use crate::branding::BrandingApiClient;
use crate::certificate_based_auth_configuration::{
    CertificateBasedAuthConfigurationApiClient, CertificateBasedAuthConfigurationIdApiClient,
};
use crate::chats::{ChatsApiClient, ChatsIdApiClient};
use crate::communications::CommunicationsApiClient;
use crate::contracts::{ContractsApiClient, ContractsIdApiClient};
use crate::data_policy_operations::DataPolicyOperationsApiClient;
use crate::default_drive::DefaultDriveApiClient;
use crate::device_app_management::DeviceAppManagementApiClient;
use crate::device_management::DeviceManagementApiClient;
use crate::devices::{DevicesApiClient, DevicesIdApiClient};
use crate::directory::DirectoryApiClient;
use crate::directory_objects::{DirectoryObjectsApiClient, DirectoryObjectsIdApiClient};
use crate::directory_role_templates::{
    DirectoryRoleTemplatesApiClient, DirectoryRoleTemplatesIdApiClient,
};
use crate::directory_roles::{DirectoryRolesApiClient, DirectoryRolesIdApiClient};
use crate::domain_dns_records::{DomainDnsRecordsApiClient, DomainDnsRecordsIdApiClient};
use crate::domains::{DomainsApiClient, DomainsIdApiClient};
use crate::drives::{DrivesApiClient, DrivesIdApiClient};
use crate::education::EducationApiClient;
use crate::group_lifecycle_policies::{
    GroupLifecyclePoliciesApiClient, GroupLifecyclePoliciesIdApiClient,
};
use crate::groups::{GroupsApiClient, GroupsIdApiClient};
use crate::identity::{
    AllowedHostValidator, AuthorizationCodeAssertionCredential,
    AuthorizationCodeCertificateCredential, AuthorizationCodeCredential, BearerTokenCredential,
    ClientAssertionCredential, ClientCertificateCredential, ClientSecretCredential,
    ConfidentialClientApplication, DeviceCodeCredential, HostIs, OpenIdCredential,
    PublicClientApplication, ResourceOwnerPasswordCredential, Token,
};
use crate::identity_access::IdentityApiClient;
use crate::identity_governance::IdentityGovernanceApiClient;
use crate::identity_providers::{IdentityProvidersApiClient, IdentityProvidersIdApiClient};
use crate::invitations::InvitationsApiClient;
use crate::me::MeApiClient;
use crate::oauth2_permission_grants::{
    Oauth2PermissionGrantsApiClient, Oauth2PermissionGrantsIdApiClient,
};
use crate::organization::{OrganizationApiClient, OrganizationIdApiClient};
use crate::permission_grants::{PermissionGrantsApiClient, PermissionGrantsIdApiClient};
use crate::places::PlacesApiClient;
use crate::planner::PlannerApiClient;
use crate::policies::PoliciesApiClient;
use crate::reports::ReportsApiClient;
use crate::schema_extensions::{SchemaExtensionsApiClient, SchemaExtensionsIdApiClient};
use crate::service_principals::{ServicePrincipalsApiClient, ServicePrincipalsIdApiClient};
use crate::sites::{SitesApiClient, SitesIdApiClient};
use crate::solutions::SolutionsApiClient;
use crate::subscribed_skus::SubscribedSkusApiClient;
use crate::subscriptions::{SubscriptionsApiClient, SubscriptionsIdApiClient};
use crate::teams::{TeamsApiClient, TeamsIdApiClient};
use crate::teams_templates::{TeamsTemplatesApiClient, TeamsTemplatesIdApiClient};
use crate::teamwork::TeamworkApiClient;
use crate::users::{UsersApiClient, UsersIdApiClient};
use crate::{GRAPH_URL, GRAPH_URL_BETA};
use graph_core::identity::ForceTokenRefresh;
use lazy_static::lazy_static;

lazy_static! {
    static ref PARSED_GRAPH_URL: Url = Url::parse(GRAPH_URL).expect("Unable to set v1 endpoint");
    static ref PARSED_GRAPH_URL_BETA: Url =
        Url::parse(GRAPH_URL_BETA).expect("Unable to set beta endpoint");
}

// For backwards compatibility.
pub type Graph = GraphClient;

#[derive(Debug, Clone)]
pub struct GraphClient {
    client: Client,
    endpoint: Url,
    allowed_host_validator: AllowedHostValidator,
}

impl GraphClient {
    pub fn new<AT: ToString>(access_token: AT) -> GraphClient {
        GraphClient {
            client: Client::new(BearerTokenCredential::from(access_token.to_string())),
            endpoint: PARSED_GRAPH_URL.clone(),
            allowed_host_validator: AllowedHostValidator::default(),
        }
    }

    pub fn from_client_app<CA: ClientApplication + 'static>(client_app: CA) -> GraphClient {
        GraphClient {
            client: Client::new(client_app),
            endpoint: PARSED_GRAPH_URL.clone(),
            allowed_host_validator: AllowedHostValidator::default(),
        }
    }

    /// Use the v1 endpoint for the Microsoft Graph API. This is the default
    /// endpoint used by the client.
    ///
    /// # Example
    /// ```rust,ignore
    /// # use graph_rs_sdk::GRAPH_URL;
    /// use graph_rs_sdk::Graph;
    ///
    /// let mut client = Graph::new("ACCESS_TOKEN");
    ///
    /// client.v1()
    ///     .me()
    ///     .get_user()
    ///     .send()
    ///     .await?;
    /// ```
    pub fn v1(&mut self) -> &mut GraphClient {
        self.endpoint = PARSED_GRAPH_URL.clone();
        self
    }

    /// Use the v1 endpoint for the Microsoft Graph API. Same as calling
    /// `v1()` but takes a mutable reference to self and does not return
    /// self.
    ///
    /// # Example
    /// ```rust
    /// # use graph_rs_sdk::GRAPH_URL;
    /// use graph_rs_sdk::Graph;
    ///
    /// let mut client = Graph::new("ACCESS_TOKEN");
    /// client.use_v1();
    ///
    /// assert_eq!(client.url().to_string(), GRAPH_URL.to_string())
    /// ```
    pub fn use_v1(&mut self) {
        self.endpoint = PARSED_GRAPH_URL.clone();
    }

    /// Use the beta endpoint for the Microsoft Graph API
    ///
    /// # Example
    /// ```rust,ignore
    /// # use graph_rs_sdk::GRAPH_URL_BETA;
    /// use graph_rs_sdk::Graph;
    ///
    /// let mut client = Graph::new("ACCESS_TOKEN");
    ///
    /// client.beta()
    ///     .me()
    ///     .get_user()
    ///     .send()
    ///     .await?;
    /// ```
    pub fn beta(&mut self) -> &mut GraphClient {
        self.endpoint = PARSED_GRAPH_URL_BETA.clone();
        self
    }

    /// Use the beta endpoint for the Microsoft Graph API. Same as calling
    /// `beta()` but takes a mutable reference to self and does not return
    /// self.
    ///
    /// Example
    /// ```rust
    /// # use graph_rs_sdk::GRAPH_URL_BETA;
    /// use graph_rs_sdk::Graph;
    ///
    /// let mut client = Graph::new("ACCESS_TOKEN");
    /// client.use_beta();
    ///
    /// assert_eq!(client.url().to_string(), GRAPH_URL_BETA.to_string())
    /// ```
    pub fn use_beta(&mut self) {
        self.endpoint = PARSED_GRAPH_URL_BETA.clone();
    }

    pub fn url(&self) -> &Url {
        &self.endpoint
    }

    pub fn with_force_token_refresh(
        &mut self,
        force_token_refresh: ForceTokenRefresh,
    ) -> &mut Self {
        self.client.with_force_token_refresh(force_token_refresh);
        self
    }

    pub fn use_force_token_refresh(&mut self, force_token_refresh: ForceTokenRefresh) {
        self.client.with_force_token_refresh(force_token_refresh);
    }

    /// Set a custom endpoint for the Microsoft Graph API. Provide the scheme and host with an
    /// optional path. The path is not set by the sdk when using a custom endpoint.
    ///
    /// The scheme must be https:// and any other provided scheme will cause a panic.
    /// See [Microsoft Graph Service Root Endpoints](https://learn.microsoft.com/en-us/graph/deployments#microsoft-graph-and-graph-explorer-service-root-endpoints)
    ///
    /// Attempting to use an invalid host will cause the client to panic. This is done
    /// for increased security.
    ///
    /// Do not use a government host endpoint without authorization and any necessary clearances.
    /// Using any government host endpoint means you should expect every API call will be monitored
    /// and recorded.
    ///
    /// You should also assume China's Graph API operated by 21Vianet is being monitored
    /// by the Chinese government who is well known for the control it has over Chinese companies
    /// and for its surveillance state of Chinese citizens.
    /// And, according to Microsoft, **These services are subject to Chinese laws**. See
    /// [Microsoft 365 operated by 21Vianet](https://learn.microsoft.com/en-us/office365/servicedescriptions/office-365-platform-service-description/microsoft-365-operated-by-21vianet)
    ///
    /// Valid Hosts:
    /// * graph.microsoft.com (Default public endpoint worldwide)
    /// * graph.microsoft.us (U.S. Government)
    /// * dod-graph.microsoft.us (U.S. Department Of Defense)
    /// * graph.microsoft.de
    /// * microsoftgraph.chinacloudapi.cn (operated by 21Vianet)
    /// * canary.graph.microsoft.com
    ///
    /// Example
    /// ```rust,ignore
    /// use graph_rs_sdk::Graph;
    ///
    /// let mut client = Graph::new("ACCESS_TOKEN");
    ///
    /// client.custom_endpoint("https://graph.microsoft.com/v1.0")
    ///     .me()
    ///     .get_user()
    ///     .send()
    ///     .await?;
    /// ```
    pub fn custom_endpoint(&mut self, url: &Url) -> &mut GraphClient {
        self.use_endpoint(url);
        self
    }

    /// Set a custom endpoint for the Microsoft Graph API. Provide the scheme and host with an
    /// optional path. The path is not set by the sdk when using a custom endpoint.
    ///
    /// The scheme must be https:// and any other provided scheme will cause a panic.
    /// See [Microsoft Graph Service Root Endpoints](https://learn.microsoft.com/en-us/graph/deployments#microsoft-graph-and-graph-explorer-service-root-endpoints)
    ///
    /// Attempting to use an invalid host will cause the client to panic. This is done
    /// for increased security.
    ///
    /// Do not use a government host endpoint without authorization and any necessary clearances.
    /// Using any government host endpoint means you should expect every API call will be monitored
    /// and recorded.
    ///
    /// You should also assume China's Graph API operated by 21Vianet is being monitored
    /// by the Chinese government who is well known for the control it has over Chinese companies
    /// and for its surveillance state of Chinese citizens.
    /// And, according to Microsoft, **These services are subject to Chinese laws**. See
    /// [Microsoft 365 operated by 21Vianet](https://learn.microsoft.com/en-us/office365/servicedescriptions/office-365-platform-service-description/microsoft-365-operated-by-21vianet)
    ///
    /// Valid Hosts:
    /// * graph.microsoft.com (Default public endpoint worldwide)
    /// * graph.microsoft.us (U.S. Government)
    /// * dod-graph.microsoft.us (U.S. Department Of Defense)
    /// * graph.microsoft.de
    /// * microsoftgraph.chinacloudapi.cn (operated by 21Vianet)
    /// * canary.graph.microsoft.com
    ///
    /// Example
    /// ```rust
    /// use url::Url;
    /// use graph_rs_sdk::Graph;
    ///
    /// let mut client = Graph::new("ACCESS_TOKEN");
    /// client.use_endpoint(&Url::parse("https://graph.microsoft.com/v1.0").unwrap());
    ///
    /// assert_eq!(client.url().to_string(), "https://graph.microsoft.com/v1.0".to_string())
    /// ```
    pub fn use_endpoint(&mut self, url: &Url) {
        if url.query().is_some() {
            panic!(
                "Invalid query - provide only the scheme, host, and optional path of the Uri such as https://graph.microsoft.com/v1.0"
            );
        }

        match self.allowed_host_validator.validate_url(url) {
            HostIs::Valid => {
                self.endpoint = url.clone();
            }
            HostIs::Invalid => panic!("Invalid host"),
        }
    }

    #[cfg(feature = "test-util")]
    pub fn use_test_endpoint(&mut self, url: &Url) {
        self.endpoint = url.clone();
    }

    api_client_impl!(admin, AdminApiClient);

    api_client_impl!(app_catalogs, AppCatalogsApiClient);

    api_client_impl!(
        agreement_acceptances,
        AgreementAcceptancesApiClient,
        agreement_acceptance,
        AgreementAcceptancesIdApiClient
    );

    api_client_impl!(
        agreements,
        AgreementsApiClient,
        agreement,
        AgreementsIdApiClient
    );

    api_client_impl!(
        applications,
        ApplicationsApiClient,
        application,
        ApplicationsIdApiClient
    );

    api_client_impl!(audit_logs, AuditLogsApiClient);

    api_client_impl!(
        authentication_method_configurations,
        AuthenticationMethodConfigurationsApiClient,
        authentication_method_configuration,
        AuthenticationMethodConfigurationsIdApiClient
    );

    api_client_impl!(
        authentication_methods_policy,
        AuthenticationMethodsPolicyApiClient
    );

    api_client_impl!(branding, BrandingApiClient);

    api_client_impl!(
        certificate_based_auth_configurations,
        CertificateBasedAuthConfigurationApiClient,
        certificate_based_auth_configuration,
        CertificateBasedAuthConfigurationIdApiClient
    );

    api_client_impl!(chats, ChatsApiClient, chat, ChatsIdApiClient);

    api_client_impl!(communications, CommunicationsApiClient);

    api_client_impl!(
        contracts,
        ContractsApiClient,
        contract,
        ContractsIdApiClient
    );

    api_client_impl!(data_policy_operations, DataPolicyOperationsApiClient);

    api_client_impl!(device_app_management, DeviceAppManagementApiClient);

    api_client_impl!(device_management, DeviceManagementApiClient);

    api_client_impl!(devices, DevicesApiClient, device, DevicesIdApiClient);

    api_client_impl!(directory, DirectoryApiClient);

    api_client_impl!(
        directory_objects,
        DirectoryObjectsApiClient,
        directory_object,
        DirectoryObjectsIdApiClient
    );

    api_client_impl!(
        directory_role_templates,
        DirectoryRoleTemplatesApiClient,
        directory_role_template,
        DirectoryRoleTemplatesIdApiClient
    );

    api_client_impl!(
        directory_roles,
        DirectoryRolesApiClient,
        directory_role,
        DirectoryRolesIdApiClient
    );

    api_client_impl!(
        domain_dns_records,
        DomainDnsRecordsApiClient,
        domain_dns_record,
        DomainDnsRecordsIdApiClient
    );

    api_client_impl!(domains, DomainsApiClient, domain, DomainsIdApiClient);

    api_client_impl!(drives, DrivesApiClient, drive, DrivesIdApiClient);

    api_client_impl_link!(default_drive, DefaultDriveApiClient);

    api_client_impl_link!(education, EducationApiClient);

    api_client_impl!(groups, GroupsApiClient, group, GroupsIdApiClient);

    api_client_impl!(
        group_lifecycle_policies,
        GroupLifecyclePoliciesApiClient,
        group_lifecycle_policy,
        GroupLifecyclePoliciesIdApiClient
    );

    api_client_impl_link!(identity, IdentityApiClient);

    api_client_impl!(identity_governance, IdentityGovernanceApiClient);

    api_client_impl!(
        identity_providers,
        IdentityProvidersApiClient,
        identity_provider,
        IdentityProvidersIdApiClient
    );

    api_client_impl!(invitations, InvitationsApiClient);

    api_client_impl_link!(me, MeApiClient);

    api_client_impl!(
        oauth2_permission_grants,
        Oauth2PermissionGrantsApiClient,
        oauth2_permission_grant,
        Oauth2PermissionGrantsIdApiClient
    );

    api_client_impl!(
        organizations,
        OrganizationApiClient,
        organization,
        OrganizationIdApiClient
    );

    api_client_impl!(places, PlacesApiClient);

    api_client_impl!(
        permission_grants,
        PermissionGrantsApiClient,
        permission_grant,
        PermissionGrantsIdApiClient
    );

    api_client_impl!(planner, PlannerApiClient);

    api_client_impl!(policies, PoliciesApiClient);

    api_client_impl!(reports, ReportsApiClient);

    api_client_impl!(
        schema_extensions,
        SchemaExtensionsApiClient,
        schema_extension,
        SchemaExtensionsIdApiClient
    );

    api_client_impl!(
        service_principals,
        ServicePrincipalsApiClient,
        service_principal,
        ServicePrincipalsIdApiClient
    );

    api_client_impl!(sites, SitesApiClient, site, SitesIdApiClient);

    api_client_impl!(solutions, SolutionsApiClient);

    api_client_impl!(
        subscribed_skus,
        SubscribedSkusApiClient,
        subscribed_sku,
        SubscriptionsIdApiClient
    );

    api_client_impl!(
        subscriptions,
        SubscriptionsApiClient,
        subscription,
        SubscriptionsIdApiClient
    );

    api_client_impl!(teams, TeamsApiClient, team, TeamsIdApiClient);

    api_client_impl!(
        teams_templates,
        TeamsTemplatesApiClient,
        teams_template,
        TeamsTemplatesIdApiClient
    );

    api_client_impl_link!(teamwork, TeamworkApiClient);

    api_client_impl!(users, UsersApiClient, user, UsersIdApiClient);

    pub fn custom(&self, method: Method, body: Option<BodyRead>) -> RequestHandler {
        let body_result = body.map(|body| body.into_body());
        if let Some(b) = body_result {
            if let Err(err) = b {
                return RequestHandler::new(
                    self.client.clone(),
                    RequestComponents::new(ResourceIdentity::Custom, self.endpoint.clone(), method),
                    Some(err),
                    None,
                );
            } else if let Ok(body_read) = b {
                return RequestHandler::new(
                    self.client.clone(),
                    RequestComponents::new(ResourceIdentity::Custom, self.endpoint.clone(), method),
                    None,
                    Some(body_read),
                );
            }
        }

        RequestHandler::new(
            self.client.clone(),
            RequestComponents::new(ResourceIdentity::Custom, self.endpoint.clone(), method),
            None,
            None,
        )
    }

    pub fn batch<B: serde::Serialize>(&self, batch: &B) -> RequestHandler {
        BatchApiClient::new(
            self.client.clone(),
            ResourceProvisioner::resource_config_with_url(
                self.endpoint.clone(),
                ResourceIdentity::Batch,
            ),
            Handlebars::new(),
        )
        .batch(batch)
    }
}

impl From<&str> for GraphClient {
    fn from(token: &str) -> Self {
        GraphClient::from_client_app(BearerTokenCredential::from(token.to_string()))
    }
}

impl From<String> for GraphClient {
    fn from(token: String) -> Self {
        GraphClient::from_client_app(BearerTokenCredential::from(token))
    }
}

impl From<&Token> for GraphClient {
    fn from(token: &Token) -> Self {
        GraphClient::from_client_app(BearerTokenCredential::from(token.access_token.clone()))
    }
}

impl From<GraphClientConfiguration> for GraphClient {
    fn from(graph_client_builder: GraphClientConfiguration) -> Self {
        GraphClient {
            client: Client::from(graph_client_builder),
            endpoint: PARSED_GRAPH_URL.clone(),
            allowed_host_validator: AllowedHostValidator::default(),
        }
    }
}

impl From<&ConfidentialClientApplication<AuthorizationCodeCredential>> for GraphClient {
    fn from(value: &ConfidentialClientApplication<AuthorizationCodeCredential>) -> Self {
        GraphClient::from_client_app(value.clone())
    }
}

impl From<&ConfidentialClientApplication<AuthorizationCodeAssertionCredential>> for GraphClient {
    fn from(value: &ConfidentialClientApplication<AuthorizationCodeAssertionCredential>) -> Self {
        GraphClient::from_client_app(value.clone())
    }
}

impl From<&ConfidentialClientApplication<AuthorizationCodeCertificateCredential>> for GraphClient {
    fn from(value: &ConfidentialClientApplication<AuthorizationCodeCertificateCredential>) -> Self {
        GraphClient::from_client_app(value.clone())
    }
}

impl From<&ConfidentialClientApplication<ClientSecretCredential>> for GraphClient {
    fn from(value: &ConfidentialClientApplication<ClientSecretCredential>) -> Self {
        GraphClient::from_client_app(value.clone())
    }
}

impl From<&ConfidentialClientApplication<ClientCertificateCredential>> for GraphClient {
    fn from(value: &ConfidentialClientApplication<ClientCertificateCredential>) -> Self {
        GraphClient::from_client_app(value.clone())
    }
}

impl From<&ConfidentialClientApplication<ClientAssertionCredential>> for GraphClient {
    fn from(value: &ConfidentialClientApplication<ClientAssertionCredential>) -> Self {
        GraphClient::from_client_app(value.clone())
    }
}

impl From<&ConfidentialClientApplication<OpenIdCredential>> for GraphClient {
    fn from(value: &ConfidentialClientApplication<OpenIdCredential>) -> Self {
        GraphClient::from_client_app(value.clone())
    }
}

impl From<&PublicClientApplication<DeviceCodeCredential>> for GraphClient {
    fn from(value: &PublicClientApplication<DeviceCodeCredential>) -> Self {
        GraphClient::from_client_app(value.clone())
    }
}

impl From<&PublicClientApplication<ResourceOwnerPasswordCredential>> for GraphClient {
    fn from(value: &PublicClientApplication<ResourceOwnerPasswordCredential>) -> Self {
        GraphClient::from_client_app(value.clone())
    }
}

impl From<&PublicClientApplication<AuthorizationCodeCredential>> for GraphClient {
    fn from(value: &PublicClientApplication<AuthorizationCodeCredential>) -> Self {
        GraphClient::from_client_app(value.clone())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic]
    fn try_invalid_host() {
        let mut client = GraphClient::new("token");
        client.custom_endpoint(&Url::parse("https://example.org").unwrap());
    }

    #[test]
    #[should_panic]
    fn try_invalid_http_scheme() {
        let mut client = GraphClient::new("token");
        client.custom_endpoint(&Url::parse("http://example.org").unwrap());
    }

    #[test]
    #[should_panic]
    fn try_invalid_query() {
        let mut client = GraphClient::new("token");
        client.custom_endpoint(&Url::parse("https://example.org?user=name").unwrap());
    }

    #[test]
    #[should_panic]
    fn try_invalid_path() {
        let mut client = GraphClient::new("token");
        client.custom_endpoint(&Url::parse("https://example.org/v1").unwrap());
    }

    #[test]
    #[should_panic]
    fn try_invalid_host2() {
        let mut client = GraphClient::new("token");
        client.use_endpoint(&Url::parse("https://example.org").unwrap());
    }

    #[test]
    #[should_panic]
    fn try_invalid_scheme2() {
        let mut client = GraphClient::new("token");
        client.use_endpoint(&Url::parse("http://example.org").unwrap());
    }

    #[test]
    #[should_panic]
    fn try_invalid_query2() {
        let mut client = GraphClient::new("token");
        client.use_endpoint(&Url::parse("https://example.org?user=name").unwrap());
    }

    #[test]
    #[should_panic]
    fn try_invalid_path2() {
        let mut client = GraphClient::new("token");
        client.use_endpoint(&Url::parse("https://example.org/v1").unwrap());
    }

    #[test]
    fn try_valid_hosts() {
        let urls = [
            "https://graph.microsoft.com/v1.0",
            "https://graph.microsoft.us",
            "https://dod-graph.microsoft.us",
            "https://graph.microsoft.de",
            "https://microsoftgraph.chinacloudapi.cn",
            "https://canary.graph.microsoft.com",
        ];

        let mut client = Graph::new("token");

        for url in urls.iter() {
            client.custom_endpoint(&Url::parse(url).unwrap());
            assert_eq!(client.url().clone(), Url::parse(url).unwrap());
        }
    }
}

#[cfg(test)]
#[cfg(feature = "test-util")]
mod test_util_feature {
    use crate::{http::Url, Graph, GraphClientConfiguration, ODataQuery};
    use wiremock::matchers::{bearer_token, method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    /// Tests the test-util feature and setting https-only to false.
    #[tokio::test]
    async fn can_set_test_endpoint() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/users"))
            .and(query_param("$top", "10"))
            .and(bearer_token("token"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let graph_client_configuration = GraphClientConfiguration::new()
            .access_token("token")
            .https_only(false);

        let mut client = Graph::from(graph_client_configuration);
        let uri = mock_server.uri();
        client.use_test_endpoint(&Url::parse(uri.as_str()).unwrap());

        let response = client.users().list_user().top("10").send().await.unwrap();
        let status = response.status();
        assert_eq!(status.as_u16(), 200);
    }

    #[tokio::test]
    #[should_panic]
    async fn test_util_feature_use_endpoint_panics() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/users"))
            .and(query_param("$top", "10"))
            .and(bearer_token("token"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let graph_client_configuration = GraphClientConfiguration::new()
            .access_token("token")
            .https_only(false);

        let mut client = Graph::from(graph_client_configuration);
        let uri = mock_server.uri();
        client.use_endpoint(&Url::parse(uri.as_str()).unwrap());

        let response = client.users().list_user().top("10").send().await.unwrap();
        let status = response.status();
        assert_eq!(status.as_u16(), 200);
    }
}
