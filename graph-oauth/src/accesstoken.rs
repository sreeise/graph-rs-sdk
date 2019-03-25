use crate::stdop::StdOp;
use chrono::{DateTime, Duration, Utc};
use chrono_humanize::HumanTime;
use reqwest::Response;
use serde_json;
use transform_request::prelude::*;

/// AccessToken that is used for api calls to OneDrive and Graph.
///
/// The implementation of AccessToken is very generic. Callers should
/// utilize the builder: oauth::AccessTokenBuilder or use OAuth which
/// sets AccessTokens automatically. Those who wish to make their own
/// requests can use the provided Transform trait implementations for
/// converting from a reqwest::Response to an AccessToken:
///
///
/// An access token represents the metadata for a OAuth 2.0 access token.
/// The token_type, expires_in, scope, and access_token fields are required.
/// The access_token field is the field normally used to make authenticated
/// queries.
///
/// # Example
/// ```rust,ignore
/// let access_token = AccessToken::from(response); // -> AccessToken from response or AccessToken::default()
/// // or
/// use rust_onedrive::oauth::AccessToken;
/// let access_token = AccessToken::transform(response); // -> Result<AccessToken, OAuthError>
/// ```
///
/// Callers who wish to have more flexibility then provided here should use
/// AccessTokenBuilder.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Hash, FromFile, ToFile)]
pub struct AccessToken {
    access_token: String,
    token_type: String,
    expires_in: i64,
    scope: String,
    refresh_token: Option<String>,
    user_id: Option<String>,
    id_token: Option<String>,
    state: Option<String>,
    timestamp: DateTime<Utc>,
}

impl AccessToken {
    /// Create a new AccessToken.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::AccessToken;
    /// let access_token = AccessToken::new("Bearer", 3600, "Read Read.Write", "ASODFIUJ34KJ;LADSK");
    /// ```
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
            timestamp: Some(Utc::now() + Duration::seconds(expires_in)),
        }
    }

    /// Set the token type.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::AccessToken;
    ///
    /// let mut access_token = AccessToken::default();
    /// access_token.token_type("Bearer");
    /// ```
    pub fn token_type(&mut self, s: &str) -> &mut AccessToken {
        self.token_type = s.into();
        self
    }

    /// Set the expies in time. This should usually be done in seconds.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::AccessToken;
    ///
    /// let mut access_token = AccessToken::default();
    /// access_token.expires_in(3600);
    /// ```
    pub fn expires_in(&mut self, expires_in: i64) -> &mut AccessToken {
        self.expires_in = expires_in;
        self.timestamp = Some(Utc::now() + Duration::seconds(expires_in));
        self
    }

    /// Set the scope.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::AccessToken;
    ///
    /// let mut access_token = AccessToken::default();
    /// access_token.scope("Read Read.Write");
    /// ```
    pub fn scope(&mut self, s: &str) -> &mut AccessToken {
        self.scope = s.into();
        self
    }

    /// Set the access token.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::AccessToken;
    ///
    /// let mut access_token = AccessToken::default();
    /// access_token.access_token("ASODFIUJ34KJ;LADSK");
    /// ```
    pub fn access_token(&mut self, s: &str) -> &mut AccessToken {
        self.access_token = s.into();
        self
    }

    /// Set the refresh token.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::AccessToken;
    ///
    /// let mut access_token = AccessToken::default();
    /// access_token.refresh_token(Some("#ASOD323U5342"));
    /// ```
    pub fn refresh_token(&mut self, s: Option<&str>) -> &mut AccessToken {
        self.refresh_token = StdOp::from(s);
        self
    }

    /// Set the user id.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::AccessToken;
    ///
    /// let mut access_token = AccessToken::default();
    /// access_token.user_id(Some("user_id"));
    /// ```
    pub fn user_id(&mut self, s: Option<&str>) -> &mut AccessToken {
        self.user_id = StdOp::from(s);
        self
    }

    /// Set the id token.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::AccessToken;
    ///
    /// let mut access_token = AccessToken::default();
    /// access_token.id_token(Some("id_token"));
    /// ```
    pub fn id_token(&mut self, s: Option<&str>) -> &mut AccessToken {
        self.id_token = StdOp::from(s);
        self
    }

    /// Set the state.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::AccessToken;
    ///
    /// let mut access_token = AccessToken::default();
    /// access_token.state(Some("state"));
    /// ```
    pub fn state(&mut self, s: Option<&str>) -> &mut AccessToken {
        self.state = StdOp::from(s);
        self
    }

    /// Reset the access token timestmap.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::AccessToken;
    ///
    /// let mut access_token = AccessToken::default();
    /// access_token.timestamp();
    /// // The timestamp is in UTC.
    /// ```
    pub fn timestamp(&mut self) {
        self.timestamp = Some(Utc::now() + Duration::seconds(self.expires_in));
    }

    /// Get the token type.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::AccessToken;
    ///
    /// let mut access_token = AccessToken::default();
    /// println!("{:#?}", access_token.get_token_type());
    /// ```
    pub fn get_token_type(&self) -> &str {
        self.token_type.as_str()
    }

    /// Set the user id.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::AccessToken;
    ///
    /// let mut access_token = AccessToken::default();
    /// // This is the original amount that was set not the difference.
    /// // To get the difference you can use access_token.elapsed().
    /// println!("{:#?}", access_token.get_expires_in());
    /// ```
    pub fn get_expires_in(&self) -> i64 {
        self.expires_in
    }

    /// Get the scopes.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::AccessToken;
    ///
    /// let mut access_token = AccessToken::default();
    /// println!("{:#?}", access_token.get_scopes());
    /// ```
    pub fn get_scopes(&self) -> &str {
        self.scope.as_str()
    }

    /// Get the access token.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::AccessToken;
    ///
    /// let mut access_token = AccessToken::default();
    /// println!("{:#?}", access_token.get_access_token());
    /// ```
    pub fn get_access_token(&self) -> &str {
        self.access_token.as_str()
    }

    /// Get the user id.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::AccessToken;
    ///
    /// let mut access_token = AccessToken::default();
    /// println!("{:#?}", access_token.get_user_id());
    /// ```
    pub fn get_user_id(&self) -> Option<String> {
        self.user_id.clone()
    }

    /// Get the refresh token.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::AccessToken;
    ///
    /// let mut access_token = AccessToken::default();
    /// println!("{:#?}", access_token.get_refresh_token());
    /// ```
    pub fn get_refresh_token(self) -> Option<String> {
        match self.refresh_token {
            Some(t) => Some(t.clone()),
            None => None,
        }
    }

    /// Get the id token.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::AccessToken;
    ///
    /// let mut access_token = AccessToken::default();
    /// println!("{:#?}", access_token.get_id_token());
    /// ```
    pub fn get_id_token(&self) -> Option<String> {
        self.id_token.clone()
    }

    /// Get the state.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::AccessToken;
    ///
    /// let mut access_token = AccessToken::default();
    /// println!("{:#?}", access_token.get_state());
    /// ```
    pub fn get_state(&self) -> Option<String> {
        self.state.clone()
    }

    /// Get the timestamp.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::AccessToken;
    ///
    /// let mut access_token = AccessToken::default();
    /// println!("{:#?}", access_token.get_timestamp());
    /// ```
    pub fn get_timestamp(&self) -> Option<DateTime<Utc>> {
        self.timestamp
    }

    /// Check whether the access token is expired. An access token is considerd
    /// expired when there is a negative difference between the timestamp set
    /// for the access token and the expires_in field.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::AccessToken;
    ///
    /// let mut access_token = AccessToken::default();
    /// println!("{:#?}", access_token.is_expired());
    /// ```
    pub fn is_expired(&self) -> bool {
        if let Some(human_time) = self.elapsed() {
            return human_time.le(&HumanTime::from(Duration::seconds(0)));
        }
        true
    }

    /// Get the time left in seconds until the access token expires.
    /// See the HumanTime crate. If you just need to know if the access token
    /// is expired then use the is_expired() message which returns a boolean
    /// true for the token has expired and false otherwise.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::AccessToken;
    ///
    /// let mut access_token = AccessToken::default();
    /// println!("{:#?}", access_token.elapsed());
    /// ```
    pub fn elapsed(&self) -> Option<HumanTime> {
        if let Some(timestamp) = self.timestamp {
            let ht = HumanTime::from(timestamp + Duration::seconds(self.expires_in));
            return Some(ht);
        }
        None
    }
}

impl Default for AccessToken {
    fn default() -> Self {
        AccessToken {
            token_type: String::new(),
            expires_in: 0,
            scope: String::new(),
            access_token: String::new(),
            refresh_token: None,
            user_id: None,
            id_token: None,
            state: None,
            timestamp: Some(Utc::now() + Duration::seconds(0)),
        }
    }
}

/// Transforms &mut reqwest::Response to Result<AccessToken, OAuthError>
impl Transform<&mut Response> for AccessToken {
    type Err = RequestError;

    fn transform(rhs: &mut Response) -> Result<Self, Self::Err>
    where
        Self: serde::Serialize + for<'de> serde::Deserialize<'de>,
    {
        let t: AccessToken = rhs.json()?;
        Ok(t)
    }
}

/// Transforms a JSON &str to Result<AccessToken, OAuthError> using serde_json
impl Transform<&str> for AccessToken {
    type Err = RequestError;

    fn transform(rhs: &str) -> Result<Self, Self::Err>
    where
        Self: serde::Serialize + for<'de> serde::Deserialize<'de>,
    {
        let t: AccessToken = serde_json::from_str(rhs)?;
        Ok(t)
    }
}

/// Transforms a Result<Response> where response: reqwest::Response
/// to Result<AccessToken, OAuthError> using serde_json
impl Transform<Result<reqwest::Response, reqwest::Error>> for AccessToken {
    type Err = RequestError;

    fn transform(rhs: Result<reqwest::Response, reqwest::Error>) -> Result<Self, Self::Err>
    where
        Self: serde::Serialize + for<'de> serde::Deserialize<'de>,
    {
        let mut r: Response = rhs?;
        let t: AccessToken = r.json()?;
        Ok(t)
    }
}

/// Converts a reqwest crate &mut reqwest::Response to an AccessToken.
/// The Result<Response> should come from an API call for an access token.
///
/// Errors will return the default AccessToken. Callers should only
/// use this method is they are ok with receiving a default access token
/// or you are sure that the response will result in an AccessToken.
///
/// In most circumstances, callers should use the AccessToken::transform(&mut response)
/// from the OAuthResultTrait implemented by AccessToken.
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
/// Errors will return the default AccessToken. Callers should only
/// use this method is they are ok with receiving a default access token
/// or you are sure that the response will result in an AccessToken.
///
/// In most circumstances, callers should use the AccessToken::transform(&mut response)
/// from the OAuthResultTrait implemented by AccessToken.
/// This will return a Result<AccessToken, OAuthError>
impl From<Result<reqwest::Response, reqwest::Error>> for AccessToken {
    fn from(result: Result<reqwest::Response, reqwest::Error>) -> Self {
        let mut r: Response = match result {
            Ok(t) => t,
            Err(_) => return AccessToken::default(),
        };
        AccessToken::from(&mut r)
    }
}

/// Converts a JSON str to an AccessToken using the serde_json crate.
///
/// Errors will return the default AccessToken. Callers should only
/// use this method is they are ok with receiving a default access token
/// or you are sure that the response will result in an AccessToken.
///
/// In most circumstances, callers should use the AccessToken::transform(&str)
/// from the OAuthResultTrait implemented by AccessToken.
/// This will return a Result<AccessToken, OAuthError>
impl From<&str> for AccessToken {
    fn from(s: &str) -> Self {
        match serde_json::from_str(s) {
            Ok(t) => t,
            Err(_) => AccessToken::default(),
        }
    }
}
