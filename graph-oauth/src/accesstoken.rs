use crate::idtoken::IdToken;
use crate::jwt::{Claim, JsonWebToken, JwtParser};
use chrono::{DateTime, Duration, TimeZone, Utc};
use chrono_humanize::HumanTime;
use from_as::*;
use graph_error::{GraphError, GraphFailure, GraphHeaders};
use std::convert::TryFrom;
use std::fmt;

/// OAuth 2.0 Access Token
///
/// Create a new AccessToken.
/// # Example
/// ```
/// # use graph_oauth::oauth::AccessToken;
/// let access_token = AccessToken::new("Bearer", 3600, "Read Read.Write", "ASODFIUJ34KJ;LADSK");
/// ```
///
/// You can also get the claims using the claims() method as well as
/// the remaining duration that the access token is valid using the elapsed()
/// method.
///
/// Tokens returned for personal microsoft accounts that use legacy MSA
/// are encrypted and cannot be parsed. This bearer token may still be
/// valid but the jwt() method will return None.
/// For more info see:
/// [Microsoft identity platform acccess tokens](https://docs.microsoft.com/en-us/azure/active-directory/develop/access-tokens)
///
///
/// For tokens where the JWT can be parsed the elapsed() method uses
/// the `exp` field in the JWT's claims. If the claims do not contain an
/// `exp` field or the token could not be parsed the elapsed() method
/// uses the expires_in field returned in the response body to caculate
/// the remaining time. These fields are only used once during
/// initialization to set a timestamp for future expiration of the access
/// token.
///
/// # Example
/// ```
/// # use graph_oauth::oauth::AccessToken;
/// # let access_token = AccessToken::new("Bearer", 3600, "Read Read.Write", "ASODFIUJ34KJ;LADSK");
///
/// // Claims
/// println!("{:#?}", access_token.claims());
///
/// // Duration left until expired.
/// println!("{:#?}", access_token.elapsed());
/// ```
#[derive(Clone, Eq, PartialEq, Serialize, Deserialize, AsFile, FromFile)]
pub struct AccessToken {
    access_token: String,
    token_type: String,
    #[serde(deserialize_with = "from_str")]
    expires_in: i64,
    scope: Option<String>,
    refresh_token: Option<String>,
    user_id: Option<String>,
    id_token: Option<String>,
    state: Option<String>,
    timestamp: Option<DateTime<Utc>>,
    #[serde(skip)]
    jwt: Option<JsonWebToken>,
}

use std::str::FromStr;
use serde::de::Deserialize;
fn from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where T: FromStr,
          T::Err: std::fmt::Display,
          D: serde::de::Deserializer<'de>
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(serde::de::Error::custom)
}

impl AccessToken {
    pub fn new(token_type: &str, expires_in: i64, scope: &str, access_token: &str) -> AccessToken {
        let mut token = AccessToken {
            token_type: token_type.into(),
            expires_in,
            scope: Some(scope.into()),
            access_token: access_token.into(),
            refresh_token: None,
            user_id: None,
            id_token: None,
            state: None,
            timestamp: Some(Utc::now() + Duration::seconds(expires_in)),
            jwt: None,
        };
        token.parse_jwt();
        token
    }

    /// Set the token type.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::AccessToken;
    ///
    /// let mut access_token = AccessToken::default();
    /// access_token.set_token_type("Bearer");
    /// ```
    pub fn set_token_type(&mut self, s: &str) -> &mut AccessToken {
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
    /// access_token.set_expires_in(3600);
    /// ```
    pub fn set_expires_in(&mut self, expires_in: i64) -> &mut AccessToken {
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
    /// access_token.set_scope("Read Read.Write");
    /// ```
    pub fn set_scope(&mut self, s: &str) -> &mut AccessToken {
        self.scope = Some(s.to_string());
        self
    }

    /// Set the access token.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::AccessToken;
    ///
    /// let mut access_token = AccessToken::default();
    /// access_token.set_bearer_token("ASODFIUJ34KJ;LADSK");
    /// ```
    pub fn set_bearer_token(&mut self, s: &str) -> &mut AccessToken {
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
    /// access_token.set_refresh_token("#ASOD323U5342");
    /// ```
    pub fn set_refresh_token(&mut self, s: &str) -> &mut AccessToken {
        self.refresh_token = Some(s.to_string());
        self
    }

    /// Set the user id.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::AccessToken;
    ///
    /// let mut access_token = AccessToken::default();
    /// access_token.set_user_id("user_id");
    /// ```
    pub fn set_user_id(&mut self, s: &str) -> &mut AccessToken {
        self.user_id = Some(s.to_string());
        self
    }

    /// Set the id token.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::{AccessToken, IdToken};
    ///
    /// let mut access_token = AccessToken::default();
    /// access_token.set_id_token("id_token");
    /// ```
    pub fn set_id_token(&mut self, s: &str) -> &mut AccessToken {
        self.id_token = Some(s.to_string());
        self
    }

    /// Set the id token.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::{AccessToken, IdToken};
    ///
    /// let mut access_token = AccessToken::default();
    /// access_token.with_id_token(IdToken::new("id_token", "code", "state", "session_state"));
    /// ```
    pub fn with_id_token(&mut self, id_token: IdToken) {
        self.id_token = Some(id_token.get_id_token());
    }

    /// Set the state.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::AccessToken;
    /// # use graph_oauth::oauth::IdToken;
    ///
    /// let mut access_token = AccessToken::default();
    /// access_token.set_state("state");
    /// ```
    pub fn set_state(&mut self, s: &str) -> &mut AccessToken {
        self.state = Some(s.to_string());
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
    pub fn gen_timestamp(&mut self) {
        self.timestamp = Some(Utc::now() + Duration::seconds(self.expires_in));
    }

    /// Get the token type.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::AccessToken;
    ///
    /// let mut access_token = AccessToken::default();
    /// println!("{:#?}", access_token.token_type());
    /// ```
    pub fn token_type(&self) -> &str {
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
    /// println!("{:#?}", access_token.expires_in());
    /// ```
    pub fn expires_in(&self) -> i64 {
        self.expires_in
    }

    /// Get the scopes.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::AccessToken;
    ///
    /// let mut access_token = AccessToken::default();
    /// println!("{:#?}", access_token.scopes());
    /// ```
    pub fn scopes(&self) -> Option<&String> {
        self.scope.as_ref()
    }

    /// Get the access token.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::AccessToken;
    ///
    /// let mut access_token = AccessToken::default();
    /// println!("{:#?}", access_token.bearer_token());
    /// ```
    pub fn bearer_token(&self) -> &str {
        self.access_token.as_str()
    }

    /// Get the user id.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::AccessToken;
    ///
    /// let mut access_token = AccessToken::default();
    /// println!("{:#?}", access_token.user_id());
    /// ```
    pub fn user_id(&self) -> Option<String> {
        self.user_id.clone()
    }

    /// Get the refresh token.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::AccessToken;
    ///
    /// let mut access_token = AccessToken::default();
    /// println!("{:#?}", access_token.refresh_token());
    /// ```
    pub fn refresh_token(self) -> Option<String> {
        match self.refresh_token {
            Some(t) => Some(t),
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
    /// println!("{:#?}", access_token.id_token());
    /// ```
    pub fn id_token(&self) -> Option<String> {
        self.id_token.clone()
    }

    /// Get the state.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::AccessToken;
    ///
    /// let mut access_token = AccessToken::default();
    /// println!("{:#?}", access_token.state());
    /// ```
    pub fn state(&self) -> Option<String> {
        self.state.clone()
    }

    /// Get the timestamp.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::AccessToken;
    ///
    /// let mut access_token = AccessToken::default();
    /// println!("{:#?}", access_token.timestamp());
    /// ```
    pub fn timestamp(&self) -> Option<DateTime<Utc>> {
        self.timestamp
    }

    // TODO: This should checked using the bearer token.
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

    // TODO: This should checked using the bearer token.
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
            let ht = HumanTime::from(timestamp);
            return Some(ht);
        }
        None
    }

    fn parse_jwt(&mut self) {
        let mut set_timestamp = false;
        if let Ok(jwt) = JwtParser::parse(self.bearer_token()) {
            if let Some(claims) = jwt.claims() {
                if let Some(claim) = claims
                    .iter()
                    .find(|item| item.key().eq(&String::from("exp")))
                {
                    let value = claim.value();
                    let number = value.as_i64().unwrap();
                    self.timestamp = Some(Utc.timestamp(number, 0));
                    set_timestamp = true;
                }
            }
            self.jwt = Some(jwt);
        }

        if !set_timestamp {
            self.gen_timestamp();
        }
    }

    pub fn claims(&self) -> Option<Vec<Claim>> {
        self.jwt.as_ref()?.claims()
    }

    pub fn jwt(&self) -> Option<&JsonWebToken> {
        self.jwt.as_ref()
    }

    pub(crate) async fn try_from_async(
        builder: reqwest::RequestBuilder,
    ) -> Result<AccessToken, GraphFailure> {
        let value = builder.send().await?;
        let status = value.status().as_u16();
        if GraphError::is_error(status) {
            let mut graph_error = GraphError::try_from(status)?;
            let graph_headers = GraphHeaders::from(&value);
            graph_error.set_headers(graph_headers);
            return Err(GraphFailure::from(graph_error));
        }
        let mut access_token: AccessToken = value.json().await?;
        access_token.parse_jwt();
        Ok(access_token)
    }
}

impl Default for AccessToken {
    fn default() -> Self {
        AccessToken {
            token_type: String::new(),
            expires_in: 0,
            scope: None,
            access_token: String::new(),
            refresh_token: None,
            user_id: None,
            id_token: None,
            state: None,
            timestamp: Some(Utc::now() + Duration::seconds(0)),
            jwt: None,
        }
    }
}

impl TryFrom<&str> for AccessToken {
    type Error = GraphFailure;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut access_token: AccessToken = serde_json::from_str(value)?;
        access_token.parse_jwt();
        Ok(access_token)
    }
}

impl TryFrom<reqwest::blocking::RequestBuilder> for AccessToken {
    type Error = GraphFailure;

    fn try_from(value: reqwest::blocking::RequestBuilder) -> Result<Self, Self::Error> {
        let response = value.send()?;
        let access_token: AccessToken = AccessToken::try_from(response)?;
        Ok(access_token)
    }
}

impl TryFrom<Result<reqwest::blocking::Response, reqwest::Error>> for AccessToken {
    type Error = GraphFailure;

    fn try_from(
        value: Result<reqwest::blocking::Response, reqwest::Error>,
    ) -> Result<Self, Self::Error> {
        let response = value?;
        AccessToken::try_from(response)
    }
}

impl TryFrom<reqwest::blocking::Response> for AccessToken {
    type Error = GraphFailure;

    fn try_from(value: reqwest::blocking::Response) -> Result<Self, Self::Error>
    where
        Self: serde::Serialize + for<'de> serde::Deserialize<'de>,
    {
        let status = value.status().as_u16();
        if GraphError::is_error(status) {
            let mut graph_error = GraphError::try_from(status)?;
            let graph_headers = GraphHeaders::from(&value);
            graph_error.set_headers(graph_headers);
            return Err(GraphFailure::from(graph_error));
        }
        let mut access_token: AccessToken = value.json()?;
        access_token.parse_jwt();
        Ok(access_token)
    }
}

impl fmt::Debug for AccessToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[bearer token not shown]\ntoken_type: {:#?}\nexpires_in: \
             {:#?}\nscope: {:#?}\nuser_id: {:#?}\nstate: {:#?}\ntimestamp: {:#?}",
            self.token_type, self.expires_in, self.scope, self.user_id, self.state, self.timestamp,
        )
    }
}
