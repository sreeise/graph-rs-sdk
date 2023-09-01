use crate::id_token::IdToken;
use crate::jwt::{JsonWebToken, JwtParser};
use chrono::{DateTime, Duration, Utc};
use chrono_humanize::HumanTime;
use graph_error::GraphFailure;
use serde::{Deserialize, Deserializer};
use serde_aux::prelude::*;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt;

use std::str::FromStr;

// Used to set timestamp based on expires in
// which can only be done after deserialization.
#[derive(Clone, Serialize, Deserialize)]
struct PhantomAccessToken {
    access_token: String,
    token_type: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    expires_in: i64,
    /// Legacy version of expires_in
    ext_expires_in: Option<i64>,
    scope: Option<String>,
    refresh_token: Option<String>,
    user_id: Option<String>,
    id_token: Option<String>,
    state: Option<String>,
    correlation_id: Option<String>,
    client_info: Option<String>,
    #[serde(flatten)]
    additional_fields: HashMap<String, Value>,
}

/// OAuth 2.0 Access Token
///
/// Create a new AccessToken.
/// # Example
/// ```
/// # use graph_oauth::oauth::MsalTokenResponse;
/// let token_response = MsalTokenResponse::new("Bearer", 3600, "Read Read.Write", "ASODFIUJ34KJ;LADSK");
/// ```
/// The [MsalTokenResponse::jwt] method attempts to parse the access token as a JWT.
/// Tokens returned for personal microsoft accounts that use legacy MSA
/// are encrypted and cannot be parsed. This bearer token may still be
/// valid but the jwt() method will return None.
/// For more info see:
/// [Microsoft identity platform access tokens](https://docs.microsoft.com/en-us/azure/active-directory/develop/access-tokens)
///
/// * Access Tokens: https://datatracker.ietf.org/doc/html/rfc6749#section-1.4
/// * Refresh Tokens: https://datatracker.ietf.org/doc/html/rfc6749#section-1.5
///
/// # Example
/// ```
/// # use graph_oauth::oauth::MsalTokenResponse;
/// # let mut token = MsalTokenResponse::new("Bearer", 3600, "Read Read.Write", "ASODFIUJ34KJ;LADSK");
///
/// // Duration left until expired.
/// println!("{:#?}", token.elapsed());
/// ```
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct MsalTokenResponse {
    pub access_token: String,
    pub token_type: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub expires_in: i64,
    /// Legacy version of expires_in
    pub ext_expires_in: Option<i64>,
    pub scope: Option<String>,
    pub refresh_token: Option<String>,
    pub user_id: Option<String>,
    pub id_token: Option<String>,
    pub state: Option<String>,
    pub correlation_id: Option<String>,
    pub client_info: Option<String>,
    pub timestamp: Option<DateTime<Utc>>,
    /// Any extra returned fields for AccessToken.
    #[serde(flatten)]
    pub additional_fields: HashMap<String, Value>,
    #[serde(skip)]
    log_pii: bool,
}

impl MsalTokenResponse {
    pub fn new(
        token_type: &str,
        expires_in: i64,
        scope: &str,
        access_token: &str,
    ) -> MsalTokenResponse {
        MsalTokenResponse {
            token_type: token_type.into(),
            ext_expires_in: Some(expires_in),
            expires_in,
            scope: Some(scope.into()),
            access_token: access_token.into(),
            refresh_token: None,
            user_id: None,
            id_token: None,
            state: None,
            correlation_id: None,
            client_info: None,
            timestamp: Some(Utc::now() + Duration::seconds(expires_in)),
            additional_fields: Default::default(),
            log_pii: false,
        }
    }

    /// Set the token type.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::MsalTokenResponse;
    ///
    /// let mut access_token = MsalTokenResponse::default();
    /// access_token.set_token_type("Bearer");
    /// ```
    pub fn set_token_type(&mut self, s: &str) -> &mut MsalTokenResponse {
        self.token_type = s.into();
        self
    }

    /// Set the expies in time. This should usually be done in seconds.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::MsalTokenResponse;
    ///
    /// let mut access_token = MsalTokenResponse::default();
    /// access_token.set_expires_in(3600);
    /// ```
    pub fn set_expires_in(&mut self, expires_in: i64) -> &mut MsalTokenResponse {
        self.expires_in = expires_in;
        self.timestamp = Some(Utc::now() + Duration::seconds(expires_in));
        self
    }

    /// Set the scope.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::MsalTokenResponse;
    ///
    /// let mut access_token = MsalTokenResponse::default();
    /// access_token.set_scope("Read Read.Write");
    /// ```
    pub fn set_scope(&mut self, s: &str) -> &mut MsalTokenResponse {
        self.scope = Some(s.to_string());
        self
    }

    /// Set the access token.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::MsalTokenResponse;
    ///
    /// let mut access_token = MsalTokenResponse::default();
    /// access_token.set_bearer_token("ASODFIUJ34KJ;LADSK");
    /// ```
    pub fn set_bearer_token(&mut self, s: &str) -> &mut MsalTokenResponse {
        self.access_token = s.into();
        self
    }

    /// Set the refresh token.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::MsalTokenResponse;
    ///
    /// let mut access_token = MsalTokenResponse::default();
    /// access_token.set_refresh_token("#ASOD323U5342");
    /// ```
    pub fn set_refresh_token(&mut self, s: &str) -> &mut MsalTokenResponse {
        self.refresh_token = Some(s.to_string());
        self
    }

    /// Set the user id.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::MsalTokenResponse;
    ///
    /// let mut access_token = MsalTokenResponse::default();
    /// access_token.set_user_id("user_id");
    /// ```
    pub fn set_user_id(&mut self, s: &str) -> &mut MsalTokenResponse {
        self.user_id = Some(s.to_string());
        self
    }

    /// Set the id token.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::{MsalTokenResponse, IdToken};
    ///
    /// let mut access_token = MsalTokenResponse::default();
    /// access_token.set_id_token("id_token");
    /// ```
    pub fn set_id_token(&mut self, s: &str) -> &mut MsalTokenResponse {
        self.id_token = Some(s.to_string());
        self
    }

    /// Set the id token.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::{MsalTokenResponse, IdToken};
    ///
    /// let mut access_token = MsalTokenResponse::default();
    /// access_token.with_id_token(IdToken::new("id_token", "code", "state", "session_state"));
    /// ```
    pub fn with_id_token(&mut self, id_token: IdToken) {
        self.id_token = Some(id_token.get_id_token());
    }

    pub fn parse_id_token(&mut self) -> Option<Result<IdToken, serde::de::value::Error>> {
        self.id_token.clone().map(|s| IdToken::from_str(s.as_str()))
    }

    /// Set the state.
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::MsalTokenResponse;
    /// # use graph_oauth::oauth::IdToken;
    ///
    /// let mut access_token = MsalTokenResponse::default();
    /// access_token.set_state("state");
    /// ```
    pub fn set_state(&mut self, s: &str) -> &mut MsalTokenResponse {
        self.state = Some(s.to_string());
        self
    }

    /// Enable or disable logging of personally identifiable information such
    /// as logging the id_token. This is disabled by default. When log_pii is enabled
    /// passing [MsalTokenResponse] to logging or print functions will log both the bearer
    /// access token value, the refresh token value if any, and the id token value.
    /// By default these do not get logged.
    pub fn enable_pii_logging(&mut self, log_pii: bool) {
        self.log_pii = log_pii;
    }

    /// Timestamp field is used to tell whether the access token is expired.
    /// This method is mainly used internally as soon as the access token
    /// is deserialized from the api response for an accurate reading
    /// on when the access token expires.
    ///
    /// You most likely do not want to use this method unless you are deserializing
    /// the access token using custom deserialization or creating your own access tokens
    /// manually.
    ///
    /// This method resets the access token timestamp based on the expires_in field
    /// which is the total seconds that the access token is valid for starting
    /// from when the token was first retrieved.
    ///
    /// This will reset the the timestamp from Utc Now + expires_in. This means
    /// that if calling [MsalTokenResponse::gen_timestamp] will only be reliable if done
    /// when the access token is first retrieved.
    ///
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::MsalTokenResponse;
    ///
    /// let mut access_token = MsalTokenResponse::default();
    /// access_token.expires_in = 86999;
    /// access_token.gen_timestamp();
    /// println!("{:#?}", access_token.timestamp);
    /// // The timestamp is in UTC.
    /// ```
    pub fn gen_timestamp(&mut self) {
        self.timestamp = Some(Utc::now() + Duration::seconds(self.expires_in));
    }

    /// Check whether the access token is expired. Uses the expires_in
    /// field to check time elapsed since token was first deserialized.
    /// This is done using a Utc timestamp set when the [MsalTokenResponse] is
    /// deserialized from the api response
    ///
    ///
    /// # Example
    /// ```
    /// # use graph_oauth::oauth::MsalTokenResponse;
    ///
    /// let mut access_token = MsalTokenResponse::default();
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
    /// # use graph_oauth::oauth::MsalTokenResponse;
    ///
    /// let mut access_token = MsalTokenResponse::default();
    /// println!("{:#?}", access_token.elapsed());
    /// ```
    pub fn elapsed(&self) -> Option<HumanTime> {
        if let Some(timestamp) = self.timestamp {
            let ht = HumanTime::from(timestamp);
            return Some(ht);
        }
        None
    }

    pub fn jwt(&mut self) -> Option<JsonWebToken> {
        JwtParser::parse(self.access_token.as_str()).ok()
    }
}

impl Default for MsalTokenResponse {
    fn default() -> Self {
        MsalTokenResponse {
            token_type: String::new(),
            expires_in: 0,
            ext_expires_in: Some(0),
            scope: None,
            access_token: String::new(),
            refresh_token: None,
            user_id: None,
            id_token: None,
            state: None,
            correlation_id: None,
            client_info: None,
            timestamp: Some(Utc::now() + Duration::seconds(0)),
            additional_fields: Default::default(),
            log_pii: false,
        }
    }
}

impl TryFrom<&str> for MsalTokenResponse {
    type Error = GraphFailure;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(serde_json::from_str(value)?)
    }
}

impl TryFrom<reqwest::blocking::RequestBuilder> for MsalTokenResponse {
    type Error = GraphFailure;

    fn try_from(value: reqwest::blocking::RequestBuilder) -> Result<Self, Self::Error> {
        let response = value.send()?;
        MsalTokenResponse::try_from(response)
    }
}

impl TryFrom<Result<reqwest::blocking::Response, reqwest::Error>> for MsalTokenResponse {
    type Error = GraphFailure;

    fn try_from(
        value: Result<reqwest::blocking::Response, reqwest::Error>,
    ) -> Result<Self, Self::Error> {
        let response = value?;
        MsalTokenResponse::try_from(response)
    }
}

impl TryFrom<reqwest::blocking::Response> for MsalTokenResponse {
    type Error = GraphFailure;

    fn try_from(value: reqwest::blocking::Response) -> Result<Self, Self::Error> {
        Ok(value.json::<MsalTokenResponse>()?)
    }
}

impl fmt::Debug for MsalTokenResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.log_pii {
            f.debug_struct("AccessToken")
                .field("bearer_token", &self.access_token)
                .field("refresh_token", &self.refresh_token)
                .field("token_type", &self.token_type)
                .field("expires_in", &self.expires_in)
                .field("scope", &self.scope)
                .field("user_id", &self.user_id)
                .field("id_token", &self.id_token)
                .field("state", &self.state)
                .field("timestamp", &self.timestamp)
                .field("additional_fields", &self.additional_fields)
                .finish()
        } else {
            f.debug_struct("AccessToken")
                .field("bearer_token", &"[REDACTED]")
                .field("token_type", &self.token_type)
                .field("expires_in", &self.expires_in)
                .field("scope", &self.scope)
                .field("user_id", &self.user_id)
                .field("id_token", &"[REDACTED]")
                .field("state", &self.state)
                .field("timestamp", &self.timestamp)
                .field("additional_fields", &self.additional_fields)
                .finish()
        }
    }
}

impl AsRef<str> for MsalTokenResponse {
    fn as_ref(&self) -> &str {
        self.access_token.as_str()
    }
}

impl<'de> Deserialize<'de> for MsalTokenResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let phantom_access_token: PhantomAccessToken = Deserialize::deserialize(deserializer)?;
        Ok(MsalTokenResponse {
            access_token: phantom_access_token.access_token,
            token_type: phantom_access_token.token_type,
            expires_in: phantom_access_token.expires_in,
            ext_expires_in: phantom_access_token.ext_expires_in,
            scope: phantom_access_token.scope,
            refresh_token: phantom_access_token.refresh_token,
            user_id: phantom_access_token.user_id,
            id_token: phantom_access_token.id_token,
            state: phantom_access_token.state,
            correlation_id: phantom_access_token.correlation_id,
            client_info: phantom_access_token.client_info,
            timestamp: Some(Utc::now() + Duration::seconds(phantom_access_token.expires_in)),
            additional_fields: phantom_access_token.additional_fields,
            log_pii: false,
        })
    }
}
