use crate::stdop::StdOp;
use reqwest;
use serde_json;

/// AccessToken that is used for api calls to OneDrive and Graph.
///
/// The implementation of AccessToken is very generic. Callers should
/// utilize the builder: oauth::AccessTokenBuilder or use OAuth which
/// sets AccessTokens automatically. Those who wish to make their own
/// requests can use the provided From trait implementations for
/// converting from a reqwest::Response to an AccessToken:
///
/// # Example
/// ```rust,ignore
///
/// let access_token = AccessToken::from(response);
/// ```
///
/// Callers who wish to have more flexibility then provided here should use
/// AccessTokenBuilder.
#[derive(Builder, Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize, Hash)]
#[builder(setter(into))]
pub struct AccessToken {
    access_token: String,
    token_type: String,
    expires_in: i64,
    scope: String,
    refresh_token: Option<String>,
    user_id: Option<String>,
    id_token: Option<String>,
}

impl AccessToken {
    pub fn new(
        token_type: &str,
        expires_in: i64,
        scope: &str,
        access_token: &str,
        refresh_token: Option<&str>,
        user_id: Option<&str>,
        id_token: Option<&str>, // Azure AD Id Token
    ) -> AccessToken {
        AccessToken {
            token_type: token_type.into(),
            expires_in,
            scope: scope.into(),
            access_token: access_token.into(),
            refresh_token: StdOp::from(refresh_token),
            user_id: StdOp::from(user_id),
            id_token: StdOp::from(id_token),
        }
    }

    // TODO: Try to remove the getter methods here after switching to
    // the new oauth version: oauth::OAuth. Callers will use the builder
    // helper that is built at compile time for AccessToken. The original
    // OAuth implementation references these so they cannot be removed yet.
    pub fn get_token_type(&self) -> String {
        self.token_type.clone()
    }

    pub fn get_expires_in(&self) -> i64 {
        self.expires_in
    }

    pub fn get_scopes(&self) -> String {
        self.scope.clone()
    }

    pub fn get_access_token(&self) -> &String {
        &self.access_token
    }

    pub fn get_user_id(&self) -> Option<String> {
        self.user_id.clone()
    }

    pub fn get_refresh_token(self) -> Option<String> {
        match self.refresh_token {
            Some(t) => Some(t.clone()),
            None => None,
        }
    }

    pub fn get_id_token(&self) -> Option<String> {
        self.id_token.clone()
    }

    pub fn set_refresh_token(&mut self, refresh_token: &str) {
        self.refresh_token = Some(refresh_token.into());
    }
}

/// Converts a reqwest crate &mut reqwest::Response to an AccessToken.
/// The Result<Response> should come from an API call for an access token.
///
/// If an error occurs, the default AccessToken is returned.
/// Use is_default to check if the returned AccessToken
/// is from Default::default.
impl From<&mut reqwest::Response> for AccessToken {
    fn from(response: &mut reqwest::Response) -> Self {
        match response.json() {
            Ok(t) => t,
            Err(e) => panic!("Error: {:#?}", e),
        }
    }
}

/// Converts a reqwest crate Result<reqwest::Response, reqwest::Error> to an AccessToken.
/// The Result<Response> should come from an API call for an access token.
///
/// If an error occurs, the default AccessToken is returned.
/// Use is_default to check if the returned AccessToken
/// is from Default::default.
impl From<&mut std::result::Result<reqwest::Response, reqwest::Error>> for AccessToken {
    fn from(result: &mut Result<reqwest::Response, reqwest::Error>) -> Self {
        let r = match result {
            Ok(t) => t,
            Err(_) => return Default::default(),
        };

        AccessToken::from(r)
    }
}

/// Converts a reqwest crate Result<reqwest::Response, reqwest::Error> to an AccessToken.
/// The Result<Response> should come from an API call for an access token.
///
/// If an error occurs, the default AccessToken is returned.
/// Use is_default to check if the returned AccessToken
/// is from Default::default.
impl From<std::result::Result<reqwest::Response, reqwest::Error>> for AccessToken {
    fn from(result: std::result::Result<reqwest::Response, reqwest::Error>) -> Self {
        let mut r = match result {
            Ok(t) => t,
            Err(_) => return Default::default(),
        };

        AccessToken::from(&mut r)
    }
}

/// Converts a JSON str to an AccessToken using the serde_json crate.
///
/// If an error occurs, the default AccessToken is returned.
/// Use is_default to check if the returned AccessToken
/// is from Default::default.
impl From<&str> for AccessToken {
    fn from(s: &str) -> Self {
        match serde_json::from_str(s) {
            Ok(t) => t,
            Err(_) => Default::default(),
        }
    }
}
