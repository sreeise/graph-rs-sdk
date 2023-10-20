#![allow(dead_code)]

use from_as::*;
use graph_core::resource::ResourceIdentity;
use graph_rs_sdk::oauth::{
    ClientSecretCredential, ConfidentialClientApplication, ResourceOwnerPasswordCredential, Token,
    TokenCredentialExecutor,
};
use graph_rs_sdk::Graph;
use std::collections::{BTreeMap, HashMap};
use std::convert::TryFrom;
use std::env;
use std::io::{Read, Write};

use graph_core::identity::ClientApplication;
use std::sync::Mutex;

// static mutex's that are used for preventing test failures
// due to too many concurrent requests (throttling) for Microsoft Graph.
lazy_static! {
    pub static ref THROTTLE_MUTEX: Mutex<()> = Mutex::new(());
    pub static ref DRIVE_THROTTLE_MUTEX: Mutex<()> = Mutex::new(());
    pub static ref ASYNC_THROTTLE_MUTEX: tokio::sync::Mutex<()> = tokio::sync::Mutex::new(());
    pub static ref DRIVE_ASYNC_THROTTLE_MUTEX: tokio::sync::Mutex<()> = tokio::sync::Mutex::new(());
}

//pub const APPLICATIONS_CLIENT: Mutex<Option<(String, Graph)>> = Mutex::new(OAuthTestClient::graph_by_rid(ResourceIdentity::Applications));

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, AsFile, FromFile, Default)]
pub enum TestEnv {
    AppVeyor,
    GitHub,
    #[default]
    Local,
}

impl TestEnv {
    pub fn is_env_set(&self) -> bool {
        match self {
            TestEnv::AppVeyor => Environment::is_appveyor(),
            TestEnv::GitHub => Environment::is_github(),
            TestEnv::Local => Environment::is_local(),
        }
    }
}

// For local testing create a env.toml file in the root of the
// repository and set the oauth credentials with the same names as the
// OAuth2 struct. The file will be read into OAuth2 so the names must
// be the same.

pub struct Environment;

impl Environment {
    pub fn is_local() -> bool {
        env::var("GRAPH_TEST_ENV") == Ok("true".to_string())
    }

    pub fn is_travis() -> bool {
        Environment::is_test_env_set()
            && env::var("TRAVIS") == Ok("true".to_string())
            && env::var("TRAVIS_SECURE_ENV_VARS").is_ok()
    }

    pub fn is_appveyor() -> bool {
        Environment::is_test_env_set() && env::var("APPVEYOR") == Ok("True".to_string())
    }

    pub fn is_github() -> bool {
        env::var("GITHUB_ACTIONS") == Ok("true".to_string())
    }

    pub fn is_test_env_set() -> bool {
        env::var("TEST_APP_TENANT").is_ok()
            && env::var("TEST_APP_ID").is_ok()
            && env::var("TEST_APP_SECRET").is_ok()
            && env::var("TEST_APP_USER_NAME").is_ok()
            && env::var("TEST_APP_PASSWORD").is_ok()
            && env::var("TEST_APP_USER_ID").is_ok()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Hash, AsFile, FromFile)]
pub struct OAuthTestCredentials {
    client_id: String,
    client_secret: String,
    username: String,
    password: String,
    tenant: String,
    #[serde(skip)]
    scope: Vec<String>,
    user_id: Option<String>,
}

impl OAuthTestCredentials {
    pub fn new(
        client_id: &str,
        client_secret: &str,
        tenant: &str,
        username: &str,
        password: &str,
        user_id: Option<&str>,
    ) -> OAuthTestCredentials {
        OAuthTestCredentials {
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            username: username.into(),
            password: password.into(),
            tenant: tenant.into(),
            scope: vec!["https://graph.microsoft.com/.default".into()],
            user_id: user_id.map(|t| t.to_string()),
        }
    }

    pub fn new_env() -> OAuthTestCredentials {
        OAuthTestCredentials {
            client_id: env::var("TEST_APP_ID").expect("Missing env TEST_APP_ID"),
            client_secret: env::var("TEST_APP_SECRET").expect("Missing env TEST_APP_SECRET"),
            username: env::var("TEST_APP_USER_NAME").expect("Missing env TEST_APP_USER_NAME"),
            password: env::var("TEST_APP_PASSWORD").expect("Missing env TEST_APP_PASSWORD"),
            tenant: env::var("TEST_APP_TENANT").expect("Missing env TEST_APP_TENANT"),
            scope: vec!["https://graph.microsoft.com/.default".into()],
            user_id: Some(env::var("TEST_APP_USER_ID").expect("Missing env TEST_APP_USER_ID")),
        }
    }

    fn client_credentials(self) -> ConfidentialClientApplication<ClientSecretCredential> {
        ConfidentialClientApplication::builder(self.client_id.as_str())
            .with_client_secret(self.client_secret.as_str())
            .with_tenant(self.tenant.as_str())
            .build()
    }

    fn resource_owner_password_credential(self) -> ResourceOwnerPasswordCredential {
        ResourceOwnerPasswordCredential::builder(self.client_id.as_str())
            .with_tenant(self.tenant.as_str())
            .with_client_id(self.client_id.as_str())
            .with_username(self.username.as_str())
            .with_password(self.password.as_str())
            .with_scope(vec!["https://graph.microsoft.com/.default"])
            .build()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Hash, AsFile, FromFile)]
pub enum OAuthTestClient {
    ClientCredentials,
    ResourceOwnerPasswordCredentials,
    AuthorizationCodeCredential,
}

impl OAuthTestClient {
    fn get_access_token(&self, creds: OAuthTestCredentials) -> Option<(String, Token)> {
        let user_id = creds.user_id.clone()?;
        match self {
            OAuthTestClient::ClientCredentials => {
                let mut credential = creds.client_credentials();
                if let Ok(response) = credential.execute() {
                    let token: Token = response.json().unwrap();
                    Some((user_id, token))
                } else {
                    None
                }
            }
            OAuthTestClient::ResourceOwnerPasswordCredentials => {
                let mut credential = creds.resource_owner_password_credential();
                if let Ok(response) = credential.execute() {
                    let token: Token = response.json().unwrap();
                    Some((user_id, token))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub fn get_client_credentials(
        &self,
        creds: OAuthTestCredentials,
    ) -> ConfidentialClientApplication<ClientSecretCredential> {
        creds.client_credentials()
    }

    async fn get_access_token_async(&self, creds: OAuthTestCredentials) -> Option<(String, Token)> {
        let user_id = creds.user_id.clone()?;
        match self {
            OAuthTestClient::ClientCredentials => {
                let mut credential = creds.client_credentials();
                match credential.execute_async().await {
                    Ok(response) => {
                        let token: Token = response.json().await.unwrap();
                        Some((user_id, token))
                    }
                    Err(_) => None,
                }
            }
            OAuthTestClient::ResourceOwnerPasswordCredentials => {
                let mut credential = creds.resource_owner_password_credential();
                match credential.execute_async().await {
                    Ok(response) => {
                        let token: Token = response.json().await.unwrap();
                        Some((user_id, token))
                    }
                    Err(_) => None,
                }
            }
            _ => None,
        }
    }

    fn get_credential(
        &self,
        creds: OAuthTestCredentials,
    ) -> Option<(String, impl ClientApplication)> {
        let user_id = creds.user_id.clone()?;
        match self {
            OAuthTestClient::ClientCredentials => {
                let credential = creds.client_credentials();
                Some((user_id, credential))
            }
            _ => None,
        }
    }

    pub fn request_access_token(&self) -> Option<(String, Token)> {
        if Environment::is_local() || Environment::is_travis() {
            let map = AppRegistrationMap::from_file("./app_registrations.json").unwrap();
            let test_client_map = OAuthTestClientMap {
                clients: map.get_default_client_credentials().clients,
            };
            self.get_access_token(test_client_map.get(self).unwrap())
        } else if Environment::is_appveyor() {
            self.get_access_token(OAuthTestCredentials::new_env())
        } else if Environment::is_github() {
            let map: OAuthTestClientMap =
                serde_json::from_str(&env::var("TEST_CREDENTIALS").unwrap()).unwrap();
            self.get_access_token(map.get(self).unwrap())
        } else {
            None
        }
    }

    pub fn request_access_token_credential(&self) -> Option<(String, impl ClientApplication)> {
        if Environment::is_local() || Environment::is_travis() {
            let map = AppRegistrationMap::from_file("./app_registrations.json").unwrap();
            let test_client_map = OAuthTestClientMap {
                clients: map.get_default_client_credentials().clients,
            };
            self.get_credential(test_client_map.get(self).unwrap())
        } else if Environment::is_github() {
            let map: OAuthTestClientMap =
                serde_json::from_str(&env::var("TEST_CREDENTIALS").unwrap()).unwrap();
            self.get_credential(map.get(self).unwrap())
        } else {
            None
        }
    }

    pub async fn request_access_token_async(&self) -> Option<(String, Token)> {
        if Environment::is_local() || Environment::is_travis() {
            let map = AppRegistrationMap::from_file("./app_registrations.json").unwrap();
            let test_client_map = OAuthTestClientMap {
                clients: map.get_default_client_credentials().clients,
            };
            self.get_access_token_async(test_client_map.get(self).unwrap())
                .await
        } else if Environment::is_appveyor() {
            self.get_access_token_async(OAuthTestCredentials::new_env())
                .await
        } else if Environment::is_github() {
            let map: OAuthTestClientMap =
                serde_json::from_str(&env::var("TEST_CREDENTIALS").unwrap()).unwrap();
            self.get_access_token_async(map.get(self).unwrap()).await
        } else {
            None
        }
    }

    pub fn get_app_registration() -> Option<AppRegistrationMap> {
        if Environment::is_local() {
            AppRegistrationMap::from_file("./app_registrations.json").ok()
        } else if Environment::is_github() {
            let app_reg: AppRegistrationMap =
                serde_json::from_str(&env::var("APP_REGISTRATIONS").ok()?).ok()?;
            Some(app_reg)
        } else {
            None
        }
    }

    pub fn graph_by_rid(resource_identity: ResourceIdentity) -> Option<(String, Graph)> {
        let app_registration = OAuthTestClient::get_app_registration()?;
        let client = app_registration.get_by_resource_identity(resource_identity)?;
        let (test_client, credentials) = client.default_client()?;

        if let Some((id, token)) = test_client.get_access_token(credentials) {
            Some((id, Graph::new(token.access_token)))
        } else {
            None
        }
    }

    pub fn client_credentials_by_rid(
        resource_identity: ResourceIdentity,
    ) -> Option<ConfidentialClientApplication<ClientSecretCredential>> {
        let app_registration = OAuthTestClient::get_app_registration()?;
        let client = app_registration.get_by_resource_identity(resource_identity)?;
        let (test_client, credentials) = client.default_client()?;
        Some(test_client.get_client_credentials(credentials))
    }

    pub fn client_secret_credential_default() -> Option<ClientSecretCredential> {
        let app_registration = OAuthTestClient::get_app_registration()?;
        let app_registration_client = app_registration.get_default_client_credentials();
        let test_client = app_registration_client
            .clients
            .get(&OAuthTestClient::ClientCredentials)
            .cloned()
            .unwrap();
        let confidential_client = test_client.client_credentials();
        Some(confidential_client.into_inner())
    }

    pub async fn graph_by_rid_async(
        resource_identity: ResourceIdentity,
    ) -> Option<(String, Graph)> {
        let app_registration = OAuthTestClient::get_app_registration()?;
        let client = app_registration.get_by_resource_identity(resource_identity)?;
        let (test_client, credentials) = client.default_client()?;
        if let Some((id, client_application)) = test_client.get_credential(credentials) {
            Some((id, Graph::from_client_app(client_application)))
        } else {
            None
        }
    }

    pub fn graph(&self) -> Option<(String, Graph)> {
        if let Some((id, client_application)) = self.request_access_token_credential() {
            Some((id, Graph::from_client_app(client_application)))
        } else {
            None
        }
    }

    pub async fn graph_async(&self) -> Option<(String, Graph)> {
        if let Some((id, client_application)) = self.request_access_token_credential() {
            Some((id, Graph::from_client_app(client_application)))
        } else {
            None
        }
    }

    pub fn token(resource_identity: ResourceIdentity) -> Option<Token> {
        let app_registration = OAuthTestClient::get_app_registration()?;
        let client = app_registration.get_by_resource_identity(resource_identity)?;
        let (test_client, _credentials) = client.default_client()?;

        if let Some((_id, token)) = test_client.request_access_token() {
            Some(token)
        } else {
            None
        }
    }
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize, AsFile, FromFile)]
pub struct OAuthTestClientMap {
    clients: HashMap<OAuthTestClient, OAuthTestCredentials>,
}

impl OAuthTestClientMap {
    pub fn new() -> OAuthTestClientMap {
        OAuthTestClientMap {
            clients: HashMap::new(),
        }
    }

    pub fn insert(&mut self, client: OAuthTestClient, credentials: OAuthTestCredentials) {
        self.clients.insert(client, credentials);
    }

    pub fn get(&self, client: &OAuthTestClient) -> Option<OAuthTestCredentials> {
        self.clients.get(client).cloned()
    }

    pub fn get_any(&self) -> Option<(OAuthTestClient, OAuthTestCredentials)> {
        let client = self.get(&OAuthTestClient::ClientCredentials);
        if client.is_none() {
            self.get(&OAuthTestClient::ResourceOwnerPasswordCredentials)
                .map(|credentials| {
                    (
                        OAuthTestClient::ResourceOwnerPasswordCredentials,
                        credentials,
                    )
                })
        } else {
            client.map(|credentials| (OAuthTestClient::ClientCredentials, credentials))
        }
    }
}

impl AsRef<HashMap<OAuthTestClient, OAuthTestCredentials>> for OAuthTestClientMap {
    fn as_ref(&self) -> &HashMap<OAuthTestClient, OAuthTestCredentials> {
        &self.clients
    }
}

impl AsMut<HashMap<OAuthTestClient, OAuthTestCredentials>> for OAuthTestClientMap {
    fn as_mut(&mut self) -> &mut HashMap<OAuthTestClient, OAuthTestCredentials> {
        &mut self.clients
    }
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize, AsFile, FromFile)]
pub struct AppRegistrationClient {
    display_name: String,
    permissions: Vec<String>,
    test_envs: Vec<TestEnv>,
    test_resources: Vec<ResourceIdentity>,
    clients: HashMap<OAuthTestClient, OAuthTestCredentials>,
}

impl AppRegistrationClient {
    pub fn new(
        display_name: &str,
        permissions: Vec<String>,
        test_resources: Vec<ResourceIdentity>,
        test_envs: Vec<TestEnv>,
    ) -> AppRegistrationClient {
        AppRegistrationClient {
            display_name: display_name.into(),
            permissions,
            test_envs,
            test_resources,
            clients: HashMap::new(),
        }
    }

    pub fn insert(&mut self, client: OAuthTestClient, credentials: OAuthTestCredentials) {
        self.clients.insert(client, credentials);
    }

    pub fn get(&self, client: &OAuthTestClient) -> Option<OAuthTestCredentials> {
        self.clients.get(client).cloned()
    }

    pub fn default_client(&self) -> Option<(OAuthTestClient, OAuthTestCredentials)> {
        let client = self.get(&OAuthTestClient::ClientCredentials);
        if client.is_none() {
            self.get(&OAuthTestClient::ResourceOwnerPasswordCredentials)
                .map(|credentials| {
                    (
                        OAuthTestClient::ResourceOwnerPasswordCredentials,
                        credentials,
                    )
                })
        } else {
            client.map(|credentials| (OAuthTestClient::ClientCredentials, credentials))
        }
    }
}

pub trait GetAppRegistration {
    fn get_by_resource_identity(&self, value: ResourceIdentity) -> Option<AppRegistrationClient>;
    fn get_by_str(&self, value: &str) -> Option<AppRegistrationClient>;

    fn get_default_client_credentials(&self) -> AppRegistrationClient;
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize, AsFile, FromFile)]
pub struct AppRegistrationMap {
    apps: BTreeMap<String, AppRegistrationClient>,
}

impl AppRegistrationMap {
    pub fn insert(&mut self, app_registration: AppRegistrationClient) {
        self.apps
            .insert(app_registration.display_name.to_string(), app_registration);
    }
}

impl GetAppRegistration for AppRegistrationMap {
    fn get_by_resource_identity(&self, value: ResourceIdentity) -> Option<AppRegistrationClient> {
        self.apps
            .iter()
            .find(|(_, reg)| reg.test_resources.contains(&value))
            .map(|(_, reg)| reg.clone())
    }

    fn get_by_str(&self, value: &str) -> Option<AppRegistrationClient> {
        self.apps.get(value).cloned()
    }

    fn get_default_client_credentials(&self) -> AppRegistrationClient {
        let app_registration = self
            .apps
            .get("graph-rs-default-client-credentials")
            .cloned()
            .unwrap();
        app_registration
    }
}
