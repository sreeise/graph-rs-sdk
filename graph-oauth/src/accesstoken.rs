use serde_json;

use crate::oautherror::OAuthResult;
use crate::stdop::StdOp;
use reqwest::Response;

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
/// let access_token = AccessToken::from(response); // -> AccessToken from response or AccessToken::default()
/// // or
/// use crate::oautherror::OAuthResult;
/// let access_token = AccessToken::from_response(response); // -> Result<AccessToken, OAuthError>
/// ```
///
/// Callers who wish to have more flexibility then provided here should use
/// AccessTokenBuilder.
#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize, Hash)]
pub struct AccessToken {
    access_token: String,
    token_type: String,
    expires_in: i64,
    scope: String,
    refresh_token: Option<String>,
    user_id: Option<String>,
    id_token: Option<String>,
    state: Option<String>,
}

impl AccessToken {
    #[allow(clippy::too_many_arguments)]
    pub fn new(token_type: &str, expires_in: i64, scope: &str, access_token: &str) -> AccessToken {
        AccessToken {
            token_type: token_type.into(),
            expires_in,
            scope: scope.into(),
            access_token: access_token.into(),
            refresh_token: None,
            user_id: None,
            id_token: None,
            state: None,
        }
    }

    pub fn token_type(&mut self, s: &str) -> &mut AccessToken {
        self.token_type = s.into();
        self
    }

    pub fn expires_in(&mut self, expires_in: i64) -> &mut AccessToken {
        self.expires_in = expires_in;
        self
    }

    pub fn scope(&mut self, s: &str) -> &mut AccessToken {
        self.scope = s.into();
        self
    }

    pub fn access_token(&mut self, s: &str) -> &mut AccessToken {
        self.access_token = s.into();
        self
    }

    pub fn refresh_token(&mut self, s: Option<&str>) -> &mut AccessToken {
        self.refresh_token = StdOp::from(s);
        self
    }

    pub fn user_id(&mut self, s: Option<&str>) -> &mut AccessToken {
        self.user_id = StdOp::from(s);
        self
    }

    pub fn id_token(&mut self, s: Option<&str>) -> &mut AccessToken {
        self.id_token = StdOp::from(s);
        self
    }

    pub fn state(&mut self, s: Option<&str>) -> &mut AccessToken {
        self.state = StdOp::from(s);
        self
    }

    pub fn get_token_type(&self) -> &str {
        self.token_type.as_str()
    }

    pub fn get_expires_in(&self) -> i64 {
        self.expires_in
    }

    pub fn get_scopes(&self) -> &str {
        self.scope.as_str()
    }

    pub fn get_access_token(&self) -> &str {
        self.access_token.as_str()
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

    pub fn get_state(&self) -> Option<String> {
        self.state.clone()
    }
}

impl OAuthResult<AccessToken> for AccessToken {}

/// Converts a reqwest crate &mut reqwest::Response to an AccessToken.
/// The Result<Response> should come from an API call for an access token.
///
/// If an error occurs, the default AccessToken is returned.
/// Use is_default to check if the returned AccessToken
/// is from Default::default.
impl From<&mut reqwest::Response> for AccessToken {
    fn from(response: &mut reqwest::Response) -> Self {
        let t: AccessToken = match response.json() {
            Ok(t) => t,
            Err(_) => return AccessToken::default(),
        };
        t
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
        let mut r: Response = match result {
            Ok(t) => t,
            Err(_) => return AccessToken::default(),
        };
        AccessToken::from(&mut r)
    }
}

/// Converts a JSON str to an AccessToken using the serde_json crate.
///
/// If an error occurs, the default AccessToken is returned.
impl From<&str> for AccessToken {
    fn from(s: &str) -> Self {
        match serde_json::from_str(s) {
            Ok(t) => t,
            Err(_) => AccessToken::default(),
        }
    }
}
