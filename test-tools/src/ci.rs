use graph_rs::oauth::{AccessToken, OAuth};
use std::env;

pub struct CI;

impl CI {
    pub fn is_travis() -> bool {
        env::var("TRAVIS") == Ok("true".to_string())
    }

    pub fn assert_not_travis() {
        if let Ok(e) = env::var("TRAVIS") {
            assert_eq!(e, "false");
        }
    }

    pub fn request_access_token() -> Option<AccessToken> {
        if let Ok(value) = env::var("TRAVIS") {
            let _ = env::var("TRAVIS_SECURE_ENV_VARS").expect("Env TRAVIS_SECURE_ENV_VARS not set");
            if value.eq("true") {
                let mut oauth = OAuth::new();
                let tenant = env::var("TEST_APP_TENANT").expect("Missing env TEST_APP_TENANT");
                let id = env::var("TEST_APP_ID").expect("Missing env TEST_APP_ID");
                let secret = env::var("TEST_APP_SECRET").expect("Missing env TEST_APP_SECRET");
                let username =
                    env::var("TEST_APP_USER_NAME").expect("Missing env TEST_APP_USER_NAME");
                let password =
                    env::var("TEST_APP_PASSWORD").expect("Missing env TEST_APP_PASSWORD");

                oauth
                    .client_secret(secret.as_str())
                    .client_id(id.as_str())
                    .username(username.as_str())
                    .password(password.as_str())
                    .add_scope("https://graph.microsoft.com/.default")
                    .access_token_url(
                        format!(
                            "https://login.microsoftonline.com/{}/oauth2/v2.0/token",
                            tenant
                        )
                        .as_str(),
                    );
                let mut req = oauth.build().client_credentials();
                if let Ok(token) = req.access_token().send() {
                    return Some(token);
                }
            }
        }
        None
    }

    pub fn test_credentials<F>(f: F)
    where
        F: Fn(Option<(String, String)>),
    {
        if let Some(token) = CI::request_access_token() {
            let t = token.bearer_token();
            let id = env::var("TEST_APP_USER_ID").expect("Missing env TEST_APP_USER_ID");
            f(Some((t.to_string(), id)));
        }
    }
}
