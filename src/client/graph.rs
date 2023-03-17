use std::convert::TryFrom;

use crate::api_default_imports::*;

use graph_core::resource::ResourceIdentity;
use graph_error::GraphFailure;
use graph_http::client::{Client, GraphClientBuilder};
use graph_http::url::GraphUrl;
use graph_http::ResponseHandler;
use graph_oauth::oauth::{AccessToken, OAuth};

use crate::admin::AdminApiClient;
use crate::agreement_acceptances::{
    AgreementAcceptancesApiClient, AgreementAcceptancesIdApiClient,
};
use crate::agreements::{AgreementsApiClient, AgreementsIdApiClient};
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
use crate::client::ResourceProvisioner;
use crate::communications::CommunicationsApiClient;
use crate::contracts::{ContractsApiClient, ContractsIdApiClient};
use crate::data_policy_operations::DataPolicyOperationsApiClient;
use crate::device_app_management::DeviceAppManagementApiClient;
use crate::device_management::DeviceManagementApiClient;
use crate::directory::DirectoryApiClient;
use crate::directory_objects::{DirectoryObjectsApiClient, DirectoryObjectsIdApiClient};
use crate::directory_role_templates::{
    DirectoryRoleTemplatesApiClient, DirectoryRoleTemplatesIdApiClient,
};
use crate::directory_roles::{DirectoryRolesApiClient, DirectoryRolesIdApiClient};
use crate::domain_dns_records::{DomainDnsRecordsApiClient, DomainDnsRecordsIdApiClient};
use crate::domains::{DomainsApiClient, DomainsIdApiClient};
use crate::drive::DefaultDriveApiClient;
use crate::drives::{DrivesApiClient, DrivesIdApiClient};
use crate::education::EducationApiClient;
use crate::group_lifecycle_policies::{
    GroupLifecyclePoliciesApiClient, GroupLifecyclePoliciesIdApiClient,
};
use crate::groups::{GroupsApiClient, GroupsIdApiClient};
use crate::identity::IdentityApiClient;
use crate::identity_governance::IdentityGovernanceApiClient;
use crate::identity_providers::{IdentityProvidersApiClient, IdentityProvidersIdApiClient};
use crate::invitations::InvitationsApiClient;
use crate::me::MeApiClient;
use crate::oauth2_permission_grants::{
    Oauth2PermissionGrantsApiClient, Oauth2PermissionGrantsIdApiClient,
};
use crate::organization::{OrganizationApiClient, OrganizationIdApiClient};
use crate::places::PlacesApiClient;
use crate::planner::PlannerApiClient;
use crate::policies::PoliciesApiClient;
use crate::reports::ReportsApiClient;
use crate::schema_extensions::{SchemaExtensionsApiClient, SchemaExtensionsIdApiClient};
use crate::service_principals::{ServicePrincipalsApiClient, ServicePrincipalsIdApiClient};
use crate::sites::{SitesApiClient, SitesIdApiClient};
use crate::subscribed_skus::SubscribedSkusApiClient;
use crate::subscriptions::{SubscriptionsApiClient, SubscriptionsIdApiClient};
use crate::teams::{TeamsApiClient, TeamsIdApiClient};
use crate::teams_templates::{TeamsTemplatesApiClient, TeamsTemplatesIdApiClient};
use crate::teamwork::TeamworkApiClient;
use crate::users::{UsersApiClient, UsersIdApiClient};
use crate::{GRAPH_URL, GRAPH_URL_BETA};

pub struct Graph {
    client: Client,
    endpoint: GraphUrl,
}

impl Graph {
    pub fn new(access_token: &str) -> Graph {
        Graph {
            client: Client::new(access_token),
            endpoint: GraphUrl::parse(GRAPH_URL).unwrap(),
        }
    }

    /// Use the v1 endpoint for the Microsoft Graph API. This is the default
    /// endpoint used by the client.
    ///
    /// Example
    /// ```rust
    /// # use graph_rs_sdk::GRAPH_URL;
    /// use graph_rs_sdk::client::Graph;
    ///
    /// let client = Graph::new("ACCESS_TOKEN").v1();
    ///
    /// assert_eq!(client.url().to_string(), GRAPH_URL.to_string())
    /// ```
    pub fn v1(mut self) -> Graph {
        self.endpoint
            .replace(GRAPH_URL)
            .expect("Unable to set v1 endpoint");
        self
    }

    /// Use the v1 endpoint for the Microsoft Graph API. Same as calling
    /// `v1()` but takes a mutable reference to self and does not return
    /// self.
    ///
    /// Example
    /// ```rust
    /// # use graph_rs_sdk::GRAPH_URL;
    /// use graph_rs_sdk::client::Graph;
    ///
    /// let mut client = Graph::new("ACCESS_TOKEN");
    /// client.use_v1();
    ///
    /// assert_eq!(client.url().to_string(), GRAPH_URL.to_string())
    /// ```
    pub fn use_v1(&mut self) {
        self.endpoint
            .replace(GRAPH_URL)
            .expect("Unable to set v1 endpoint");
    }

    /// Use the beta endpoint for the Microsoft Graph API
    ///
    /// Example
    /// ```rust
    /// # use graph_rs_sdk::GRAPH_URL_BETA;
    /// use graph_rs_sdk::client::Graph;
    ///
    /// let client = Graph::new("ACCESS_TOKEN").beta();
    ///
    /// assert_eq!(client.url().to_string(), GRAPH_URL_BETA.to_string())
    /// ```
    pub fn beta(mut self) -> Graph {
        self.endpoint
            .replace(GRAPH_URL_BETA)
            .expect("Unable to set beta endpoint");
        self
    }

    /// Use the beta endpoint for the Microsoft Graph API. Same as calling
    /// `beta()` but takes a mutable reference to self and does not return
    /// self.
    ///
    /// Example
    /// ```rust
    /// # use graph_rs_sdk::GRAPH_URL_BETA;
    /// use graph_rs_sdk::client::Graph;
    ///
    /// let mut client = Graph::new("ACCESS_TOKEN");
    /// client.use_beta();
    ///
    /// assert_eq!(client.url().to_string(), GRAPH_URL_BETA.to_string())
    /// ```
    pub fn use_beta(&mut self) {
        self.endpoint
            .replace(GRAPH_URL_BETA)
            .expect("Unable to set beta endpoint");
    }

    pub fn url(&self) -> reqwest::Url {
        self.endpoint.to_reqwest_url()
    }

    /// Set a custom endpoint for the Microsoft Graph API
    /// # See [microsoft-graph-and-graph-explorer-service-root-endpoints](https://learn.microsoft.com/en-us/graph/deployments#microsoft-graph-and-graph-explorer-service-root-endpoints)
    ///
    /// Example
    /// ```rust
    /// use graph_rs_sdk::client::Graph;
    ///
    /// let client = Graph::new("ACCESS_TOKEN").custom_endpoint("https://api.microsoft.com/api");
    ///
    /// assert_eq!(client.url().to_string(), "https://api.microsoft.com/api".to_string())
    /// ```
    pub fn custom_endpoint(mut self, custom_endpoint: &str) -> Graph {
        self.endpoint
            .replace(custom_endpoint)
            .expect("Unable to set custom endpoint");
        self
    }

    pub fn use_endpoint(&mut self, custom_endpoint: &str) {
        self.endpoint
            .replace(custom_endpoint)
            .expect("Unable to set custom endpoint");
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

    pub fn batch<B: serde::Serialize>(&self, batch: &B) -> ResponseHandler {
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

impl From<&str> for Graph {
    fn from(token: &str) -> Self {
        Graph::new(token)
    }
}

impl From<String> for Graph {
    fn from(token: String) -> Self {
        Graph::new(token.as_str())
    }
}

impl From<&AccessToken> for Graph {
    fn from(token: &AccessToken) -> Self {
        Graph::new(token.bearer_token())
    }
}

impl TryFrom<&OAuth> for Graph {
    type Error = GraphFailure;

    fn try_from(oauth: &OAuth) -> Result<Self, Self::Error> {
        let access_token = oauth
            .get_access_token()
            .ok_or_else(|| GraphFailure::not_found("no access token"))?;
        Ok(Graph::from(&access_token))
    }
}

impl From<GraphClientBuilder> for Graph {
    fn from(graph_client_builder: GraphClientBuilder) -> Self {
        Graph {
            client: graph_client_builder.build(),
            endpoint: GraphUrl::parse(GRAPH_URL).unwrap(),
        }
    }
}

mod test {
    use crate::client::Graph;

    #[test]
    fn test_host() {
        let mut client = Graph::new("ACCESS_TOKEN").beta();
        assert_eq!(
            "graph.microsoft.com".to_string(),
            client.url().host().expect("dead").to_string()
        );
        assert_eq!("/beta".to_string(), client.url().path());

        client.use_v1();
        assert_eq!(
            "graph.microsoft.com".to_string(),
            client.url().host().expect("dead").to_string()
        );
        assert_eq!("/v1.0".to_string(), client.url().path());

        client = client.custom_endpoint("https://localhost/api");
        assert_eq!(
            "localhost".to_string(),
            client.url().host().expect("dead").to_string()
        );
        assert_eq!("/api".to_string(), client.url().path());
    }
}
