use from_as::*;
use graph_rs::oauth::{AccessToken, OAuth};
use std::env;
use std::io::Read;

/*
For local testing create a env.toml file in the root of the
repository and set the oauth credentials with the same names as the
OAuth2 struct. The file will be read into OAuth2 so the names must
be the same.
*/

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Hash, AsFile, FromFile)]
pub struct OAuth2 {
    pub client_id: String,
    pub client_secret: String,
    pub username: String,
    pub password: String,
    pub tenant: String,
    pub user_id: Option<String>,
}

pub struct OAuthRequest;

impl OAuthRequest {
    pub fn is_travis() -> bool {
        env::var("TRAVIS") == Ok("true".to_string())
    }

    pub fn is_travis_env_set() -> bool {
        env::var("TEST_APP_TENANT").is_ok() &&
            env::var("TEST_APP_ID").is_ok() &&
            env::var("TEST_APP_SECRET").is_ok() &&
            env::var("TEST_APP_USER_NAME").is_ok() &&
            env::var("TEST_APP_PASSWORD").is_ok() &&
            env::var("TEST_APP_USER_ID").is_ok()
    }

    pub fn is_file_env_set() -> bool {
        env::var("GRAPH_TEST_ENV") == Ok("true".to_string())
    }

    pub fn request_token_toml_env() -> Option<(String, AccessToken)> {
        if let Ok(oauth2) = OAuth2::from_file("./env.toml") {
            let access_token = OAuthRequest::access_token(
                oauth2.client_id.as_str(),
                oauth2.client_secret.as_str(),
                oauth2.username.as_str(),
                oauth2.password.as_str(),
                oauth2.tenant.as_str(),
                &["https://graph.microsoft.com/.default"],
            );
            if let Some(token) = access_token {
                if let Some(user_id) = oauth2.user_id {
                    return Some((user_id.to_string(), token));
                } else {
                    return Some((String::new(), token));
                }
            }
        }
        None
    }

    fn access_token(
        client_id: &str,
        secret: &str,
        username: &str,
        password: &str,
        tenant: &str,
        scope: &[&str],
    ) -> Option<AccessToken> {
        let mut oauth = OAuth::new();
        oauth
            .client_id(client_id)
            .client_secret(secret)
            .username(username)
            .password(password)
            .extend_scopes(scope)
            .access_token_url(
                format!(
                    "https://login.microsoftonline.com/{}/oauth2/v2.0/token",
                    tenant
                )
                .as_str(),
            );

        let mut req = oauth.build().client_credentials();
        if let Ok(token) = req.access_token().send() {
            Some(token)
        } else {
            None
        }
    }

    pub fn request_access_token() -> Option<(String, AccessToken)> {
        if let Ok(env) = env::var("GRAPH_TEST_ENV") {
            if env.eq("true") {
                return OAuthRequest::request_token_toml_env();
            }
        } else if OAuthRequest::is_travis_env_set() {
            if let Ok(value) = env::var("TRAVIS") {
                let _ =
                    env::var("TRAVIS_SECURE_ENV_VARS").expect("Env TRAVIS_SECURE_ENV_VARS not set");
                if value.eq("true") {
                    let tenant = env::var("TEST_APP_TENANT").expect("Missing env TEST_APP_TENANT");
                    let id = env::var("TEST_APP_ID").expect("Missing env TEST_APP_ID");
                    let secret = env::var("TEST_APP_SECRET").expect("Missing env TEST_APP_SECRET");
                    let username =
                        env::var("TEST_APP_USER_NAME").expect("Missing env TEST_APP_USER_NAME");
                    let password =
                        env::var("TEST_APP_PASSWORD").expect("Missing env TEST_APP_PASSWORD");
                    let user_id =
                        env::var("TEST_APP_USER_ID").expect("Missing env TEST_APP_USER_ID");

                    if let Some(token) = OAuthRequest::access_token(
                        id.as_str(),
                        secret.as_str(),
                        username.as_str(),
                        password.as_str(),
                        tenant.as_str(),
                        &["https://graph.microsoft.com/.default"],
                    ) {
                        return Some((user_id, token));
                    }
                }
            }
        }
        None
    }

    pub fn test_credentials<F>(f: F)
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
