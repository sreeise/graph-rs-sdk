use graph_rs::oauth::{AccessToken, OAuth};
use std::env;

pub struct CI;

impl CI {
    pub fn is_travis() -> bool {
        env::var("TRAVIS") == Ok("true".to_string())
    }

    pub fn will_succeed() -> bool {
        env::var("TEST_APP_TENANT").is_ok() &&
            env::var("TEST_APP_ID").is_ok() &&
            env::var("TEST_APP_SECRET").is_ok() &&
            env::var("TEST_APP_USER_NAME").is_ok() &&
            env::var("TEST_APP_PASSWORD").is_ok() &&
            env::var("TEST_APP_USER_ID").is_ok()
    }

    pub fn assert_not_travis() {
        if let Ok(e) = env::var("TRAVIS") {
            assert_eq!(e, "false");
        }
    }

    pub fn request_access_token() -> Option<(String, AccessToken)> {
        if !CI::will_succeed() {
            return None;
        }

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
                let user_id = env::var("TEST_APP_USER_ID").expect("Missing env TEST_APP_USER_ID");

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
                    return Some((user_id, token));
                }
            }
        }
        None
    }

    pub fn test_credentials<F>(f: F)
    where
        F: Fn(Option<(String, String)>),
    {
        if !CI::will_succeed() {
            f(None);
        } else if let Some((user_id, token)) = CI::request_access_token() {
            let t = token.bearer_token();
            f(Some((user_id, t.to_string())));
        } else {
            f(None);
        }
    }
}
