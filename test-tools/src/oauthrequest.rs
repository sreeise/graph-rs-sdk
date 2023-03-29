use from_as::*;
use graph_rs_sdk::client::Graph;
use graph_rs_sdk::core::ResourceIdentity;
use graph_rs_sdk::oauth::{AccessToken, OAuth};
use std::collections::{BTreeMap, HashMap};
use std::convert::TryFrom;
use std::env;
use std::io::{Read, Write};
use std::sync::Mutex;

// static mutex's that are used for preventing test failures
// due to too many concurrent requests for Microsoft Graph.
lazy_static! {
    pub static ref THROTTLE_MUTEX: Mutex<()> = Mutex::new(());
    pub static ref DRIVE_THROTTLE_MUTEX: Mutex<()> = Mutex::new(());
    pub static ref ASYNC_THROTTLE_MUTEX: tokio::sync::Mutex<()> = tokio::sync::Mutex::new(());
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, AsFile, FromFile)]
pub enum TestEnv {
    AppVeyor,
    GitHub,
    TravisCI,
    Local,
}

impl TestEnv {
    pub fn is_env_set(&self) -> bool {
        match self {
            TestEnv::AppVeyor => Environment::is_appveyor(),
            TestEnv::GitHub => Environment::is_github(),
            TestEnv::TravisCI => Environment::is_travis(),
            TestEnv::Local => Environment::is_local(),
        }
    }
}

impl Default for TestEnv {
    fn default() -> Self {
        TestEnv::Local
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

    pub fn new_local() -> OAuthTestCredentials {
        OAuthTestCredentials::new_local_from_path("./env.toml")
    }

    pub fn new_local_from_path(path: &str) -> OAuthTestCredentials {
        let mut creds: OAuthTestCredentials = OAuthTestCredentials::from_file(path).unwrap();
        creds
            .scope
            .push("https://graph.microsoft.com/.default".into());
        creds
    }

    fn into_oauth(self) -> OAuth {
        let mut oauth = OAuth::new();
        oauth
            .client_id(self.client_id.as_str())
            .client_secret(self.client_secret.as_str())
            .username(self.username.as_str())
            .password(self.password.as_str())
            .add_scope("https://graph.microsoft.com/.default")
            .access_token_url(
                format!(
                    "https://login.microsoftonline.com/{}/oauth2/v2.0/token",
                    self.tenant.as_str()
                )
                .as_str(),
            );
        oauth
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Hash, AsFile, FromFile)]
pub enum OAuthTestClient {
    ClientCredentials,
    ROPC,
}

impl OAuthTestClient {
    fn get_access_token(&self, creds: OAuthTestCredentials) -> Option<(String, AccessToken)> {
        let user_id = creds.user_id.clone()?;
        let mut oauth: OAuth = creds.into_oauth();
        let mut req = {
            match self {
                OAuthTestClient::ClientCredentials => oauth.build().client_credentials(),
                OAuthTestClient::ROPC => oauth.build().resource_owner_password_credentials(),
            }
        };

        if let Ok(token) = req.access_token().send() {
            Some((user_id, token))
        } else {
            None
        }
    }

    async fn get_access_token_async(
        &self,
        creds: OAuthTestCredentials,
    ) -> Option<(String, AccessToken)> {
        let user_id = creds.user_id.clone()?;
        let mut oauth: OAuth = creds.into_oauth();
        let mut req = {
            match self {
                OAuthTestClient::ClientCredentials => oauth.build_async().client_credentials(),
                OAuthTestClient::ROPC => oauth.build_async().resource_owner_password_credentials(),
            }
        };

        return match req.access_token().send().await {
            Ok(token) => Some((user_id, token)),
            Err(err) => {
                dbg!(&err);
                None
            }
        };
    }

    pub fn request_access_token(&self) -> Option<(String, AccessToken)> {
        if Environment::is_local() || Environment::is_travis() {
            let map: OAuthTestClientMap = OAuthTestClientMap::from_file("./env.json").unwrap();
            self.get_access_token(map.get(self).unwrap())
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

    pub async fn request_access_token_async(&self) -> Option<(String, AccessToken)> {
        if Environment::is_local() || Environment::is_travis() {
            let map: OAuthTestClientMap = OAuthTestClientMap::from_file("./env.json").unwrap();
            self.get_access_token_async(map.get(self).unwrap()).await
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
        let mut app_registration = OAuthTestClient::get_app_registration()?;
        let client = app_registration.get_by(resource_identity)?;
        let (test_client, credentials) = client.default_client()?;

        if let Some((id, token)) = test_client.get_access_token(credentials) {
            Some((id, Graph::new(token.bearer_token())))
        } else {
            None
        }
    }

    pub async fn graph_by_rid_async(
        resource_identity: ResourceIdentity,
    ) -> Option<(String, Graph)> {
        let mut app_registration = OAuthTestClient::get_app_registration()?;
        dbg!(&app_registration);
        let client = app_registration.get_by(resource_identity)?;
        dbg!(&client);
        let (test_client, credentials) = client.default_client()?;
        dbg!(&credentials);
        if let Some((id, token)) = test_client.get_access_token_async(credentials).await {
            dbg!(&token);
            Some((id, Graph::new(token.bearer_token())))
        } else {
            None
        }
    }

    pub fn graph(&self) -> Option<(String, Graph)> {
        if let Some((id, token)) = self.request_access_token() {
            Some((id, Graph::new(token.bearer_token())))
        } else {
            None
        }
    }

    pub async fn graph_v2(&self) -> Option<(String, Graph)> {
        if let Some((id, token)) = self.request_access_token_async().await {
            Some((id, Graph::new(token.bearer_token())))
        } else {
            None
        }
    }

    pub async fn graph_async(&self) -> Option<(String, Graph)> {
        if let Some((id, token)) = self.request_access_token_async().await {
            Some((id, Graph::new(token.bearer_token())))
        } else {
            None
        }
    }

    pub fn token(resource_identity: ResourceIdentity) -> Option<AccessToken> {
        let mut app_registration = OAuthTestClient::get_app_registration()?;
        let client = app_registration.get_by(resource_identity)?;
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
            self.get(&OAuthTestClient::ROPC)
                .map(|credentials| (OAuthTestClient::ROPC, credentials))
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
    clients: OAuthTestClientMap,
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
            clients: OAuthTestClientMap::new(),
        }
    }

    pub fn insert(&mut self, client: OAuthTestClient, credentials: OAuthTestCredentials) {
        self.clients.insert(client, credentials);
    }

    pub fn get(&self, client: &OAuthTestClient) -> Option<OAuthTestCredentials> {
        self.clients.get(client)
    }

    pub fn default_client(&self) -> Option<(OAuthTestClient, OAuthTestCredentials)> {
        self.clients.get_any()
    }
}

pub trait GetBy<T, U> {
    fn get_by(&mut self, value: T) -> U;
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

impl GetBy<&str, Option<AppRegistrationClient>> for AppRegistrationMap {
    fn get_by(&mut self, value: &str) -> Option<AppRegistrationClient> {
        self.apps.get(value).cloned()
    }
}

impl GetBy<ResourceIdentity, Option<AppRegistrationClient>> for AppRegistrationMap {
    fn get_by(&mut self, value: ResourceIdentity) -> Option<AppRegistrationClient> {
        self.apps
            .iter()
            .find(|(_, reg)| reg.test_resources.contains(&value))
            .map(|(_, reg)| reg.clone())
    }
}
