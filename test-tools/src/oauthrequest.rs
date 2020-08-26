use from_as::*;
use graph_rs::client::Graph;
use graph_rs::http::{AsyncHttpClient, BlockingHttpClient};
use graph_rs::oauth::{AccessToken, OAuth};
use std::collections::HashMap;
use std::env;
use std::io::Read;
use std::sync::Mutex;

// static mutex's that are used for preventing test failures
// due to too many concurrent requests for Microsoft Graph.
lazy_static! {
    pub static ref THROTTLE_MUTEX: Mutex<()> = Mutex::new(());
    pub static ref DRIVE_THROTTLE_MUTEX: Mutex<()> = Mutex::new(());
    pub static ref ASYNC_THROTTLE_MUTEX: tokio::sync::Mutex<()> = tokio::sync::Mutex::new(());
}

/*
For local testing create a env.toml file in the root of the
repository and set the oauth credentials with the same names as the
OAuth2 struct. The file will be read into OAuth2 so the names must
be the same.
*/

pub struct Environment;

impl Environment {
    pub fn is_local() -> bool {
        env::var("GRAPH_TEST_ENV") == Ok("true".to_string())
    }

    pub fn is_travis() -> bool {
        Environment::is_test_env_set() &&
            env::var("TRAVIS") == Ok("true".to_string()) &&
            env::var("TRAVIS_SECURE_ENV_VARS").is_ok()
    }

    pub fn is_appveyor() -> bool {
        Environment::is_test_env_set() && env::var("APPVEYOR") == Ok("True".to_string())
    }

    pub fn is_test_env_set() -> bool {
        env::var("TEST_APP_TENANT").is_ok() &&
            env::var("TEST_APP_ID").is_ok() &&
            env::var("TEST_APP_SECRET").is_ok() &&
            env::var("TEST_APP_USER_NAME").is_ok() &&
            env::var("TEST_APP_PASSWORD").is_ok() &&
            env::var("TEST_APP_USER_ID").is_ok()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Hash, AsFile, FromFile)]
pub struct OAuth2 {
    pub client_id: String,
    pub client_secret: String,
    pub username: String,
    pub password: String,
    pub tenant: String,
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Hash, AsFile, FromFile)]
pub struct OAuthTestCredentials {
    client_id: String,
    client_secret: String,
    username: String,
    password: String,
    tenant: String,
    scope: Vec<String>,
    user_id: Option<String>,
}

impl OAuthTestCredentials {
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
        OAuthTestCredentials::from(OAuth2::from_file(path).unwrap())
    }

    fn into_oauth(self) -> OAuth {
        let mut oauth = OAuth::new();
        oauth
            .client_id(self.client_id.as_str())
            .client_secret(self.client_secret.as_str())
            .username(self.username.as_str())
            .password(self.password.as_str())
            .extend_scopes(self.scope)
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

impl From<OAuth2> for OAuthTestCredentials {
    fn from(oauth2: OAuth2) -> Self {
        OAuthTestCredentials {
            client_id: oauth2.client_id.to_string(),
            client_secret: oauth2.client_secret.to_string(),
            username: oauth2.username.to_string(),
            password: oauth2.password.to_string(),
            tenant: oauth2.tenant.to_string(),
            scope: vec!["https://graph.microsoft.com/.default".into()],
            user_id: oauth2.user_id,
        }
    }
}

struct ClientCredentials {
    client_id: String,
    secret: String,
    username: String,
    password: String,
    tenant: String,
    scope: Vec<String>,
    user_id: Option<String>,
}

impl ClientCredentials {
    pub fn new_env() -> ClientCredentials {
        ClientCredentials {
            client_id: env::var("TEST_APP_ID").expect("Missing env TEST_APP_ID"),
            secret: env::var("TEST_APP_SECRET").expect("Missing env TEST_APP_SECRET"),
            username: env::var("TEST_APP_USER_NAME").expect("Missing env TEST_APP_USER_NAME"),
            password: env::var("TEST_APP_PASSWORD").expect("Missing env TEST_APP_PASSWORD"),
            tenant: env::var("TEST_APP_TENANT").expect("Missing env TEST_APP_TENANT"),
            scope: vec!["https://graph.microsoft.com/.default".into()],
            user_id: Some(env::var("TEST_APP_USER_ID").expect("Missing env TEST_APP_USER_ID")),
        }
    }

    pub fn new_local() -> Option<ClientCredentials> {
        if let Ok(oauth2) = OAuth2::from_file("./env.toml") {
            return Some(ClientCredentials {
                client_id: oauth2.client_id.to_string(),
                secret: oauth2.client_secret.to_string(),
                username: oauth2.username.to_string(),
                password: oauth2.password.to_string(),
                tenant: oauth2.tenant.to_string(),
                scope: vec!["https://graph.microsoft.com/.default".into()],
                user_id: oauth2.user_id,
            });
        }

        None
    }

    fn into_oauth(self) -> OAuth {
        let mut oauth = OAuth::new();
        oauth
            .client_id(self.client_id.as_str())
            .client_secret(self.secret.as_str())
            .username(self.username.as_str())
            .password(self.password.as_str())
            .extend_scopes(self.scope.clone())
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

        if let Ok(token) = req.access_token().send().await {
            Some((user_id, token))
        } else {
            None
        }
    }

    pub fn request_access_token(&self) -> Option<(String, AccessToken)> {
        if Environment::is_local() {
            self.get_access_token(OAuthTestCredentials::new_local())
        } else if Environment::is_travis() || Environment::is_appveyor() {
            self.get_access_token(OAuthTestCredentials::new_env())
        } else {
            None
        }
    }

    pub async fn request_access_token_async(&self) -> Option<(String, AccessToken)> {
        if Environment::is_local() {
            self.get_access_token_async(OAuthTestCredentials::new_local())
                .await
        } else if Environment::is_travis() || Environment::is_appveyor() {
            self.get_access_token_async(OAuthTestCredentials::new_env())
                .await
        } else {
            None
        }
    }

    pub fn graph(&self) -> Option<(String, Graph<BlockingHttpClient>)> {
        if let Some((id, token)) = self.request_access_token() {
            Some((id, Graph::new(token.bearer_token())))
        } else {
            None
        }
    }

    pub fn graph_async(&self) -> Option<(String, Graph<AsyncHttpClient>)> {
        if let Some((id, token)) = self.request_access_token() {
            Some((id, Graph::new_async(token.bearer_token())))
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

pub struct OAuthRequest;

impl OAuthRequest {
    pub fn is_local() -> bool {
        env::var("GRAPH_TEST_ENV") == Ok("true".to_string())
    }

    pub fn is_travis() -> bool {
        env::var("TRAVIS") == Ok("true".to_string())
    }

    pub fn is_appveyor() -> bool {
        env::var("APPVEYOR") == Ok("True".to_string())
    }

    pub fn is_test_env_set() -> bool {
        env::var("TEST_APP_TENANT").is_ok() &&
            env::var("TEST_APP_ID").is_ok() &&
            env::var("TEST_APP_SECRET").is_ok() &&
            env::var("TEST_APP_USER_NAME").is_ok() &&
            env::var("TEST_APP_PASSWORD").is_ok() &&
            env::var("TEST_APP_USER_ID").is_ok()
    }

    pub fn request_token_from_toml() -> Option<(String, AccessToken)> {
        let client_creds = ClientCredentials::new_local()?;
        let user_id = client_creds.user_id.clone();
        if let Some(token) = OAuthRequest::access_token(client_creds) {
            if let Some(user_id) = user_id {
                return Some((user_id, token));
            } else {
                return Some((String::new(), token));
            }
        }

        None
    }

    pub async fn request_token_from_toml_async() -> Option<(String, AccessToken)> {
        let client_creds = ClientCredentials::new_local()?;
        let user_id = client_creds.user_id.clone();
        if let Some(token) = OAuthRequest::access_token_async(client_creds).await {
            if let Some(user_id) = user_id {
                return Some((user_id, token));
            } else {
                return Some((String::new(), token));
            }
        }
        None
    }

    fn access_token(client_creds: ClientCredentials) -> Option<AccessToken> {
        let mut oauth = client_creds.into_oauth();
        let mut req = oauth.build().client_credentials();
        if let Ok(token) = req.access_token().send() {
            Some(token)
        } else {
            None
        }
    }

    async fn access_token_async(client_creds: ClientCredentials) -> Option<AccessToken> {
        let mut oauth = client_creds.into_oauth();
        let mut req = oauth.build_async().client_credentials();
        if let Ok(token) = req.access_token().send().await {
            Some(token)
        } else {
            None
        }
    }

    fn request_token_from_env() -> Option<(String, AccessToken)> {
        let client_creds = ClientCredentials::new_env();
        let user_id = client_creds.user_id.clone().unwrap();
        if let Some(token) = OAuthRequest::access_token(client_creds) {
            Some((user_id, token))
        } else {
            None
        }
    }

    async fn request_token_from_env_async() -> Option<(String, AccessToken)> {
        let client_creds = ClientCredentials::new_env();
        let user_id = client_creds.user_id.clone().unwrap();
        if let Some(token) = OAuthRequest::access_token_async(client_creds).await {
            Some((user_id, token))
        } else {
            None
        }
    }

    pub fn request_access_token() -> Option<(String, AccessToken)> {
        if OAuthRequest::is_local() {
            return OAuthRequest::request_token_from_toml();
        } else if OAuthRequest::is_test_env_set() {
            if OAuthRequest::is_travis() {
                let _ =
                    env::var("TRAVIS_SECURE_ENV_VARS").expect("Env TRAVIS_SECURE_ENV_VARS not set");
                return OAuthRequest::request_token_from_env();
            } else if OAuthRequest::is_appveyor() {
                return OAuthRequest::request_token_from_env();
            }
        }
        None
    }

    pub async fn request_access_token_async() -> Option<(String, AccessToken)> {
        if OAuthRequest::is_local() {
            return OAuthRequest::request_token_from_toml_async().await;
        } else if OAuthRequest::is_test_env_set() {
            if OAuthRequest::is_travis() {
                let _ =
                    env::var("TRAVIS_SECURE_ENV_VARS").expect("Env TRAVIS_SECURE_ENV_VARS not set");
                return OAuthRequest::request_token_from_env_async().await;
            } else if OAuthRequest::is_appveyor() {
                return OAuthRequest::request_token_from_env_async().await;
            }
        }
        None
    }

    pub fn access_token_fn<F>(f: F)
    where
        F: Fn(Option<(String, String)>),
    {
        if let Some((user_id, token)) = OAuthRequest::request_access_token() {
            let t = token.bearer_token();
            f(Some((user_id, t.to_string())));
        } else {
            f(None);
        }
    }
}
