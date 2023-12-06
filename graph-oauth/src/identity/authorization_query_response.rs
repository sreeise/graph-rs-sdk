use crate::identity::AppConfig;
use graph_error::{WebViewError, WebViewResult};
use serde::Deserializer;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use url::Url;

/// The specification defines errors here:
/// https://datatracker.ietf.org/doc/html/draft-ietf-oauth-v2-31#section-4.2.2.1
///
/// Microsoft has additional errors listed here:
/// https://learn.microsoft.com/en-us/entra/identity-platform/v2-oauth2-auth-code-flow#error-codes-for-authorization-endpoint-errors
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum AuthorizationQueryError {
    /// The request is missing a required parameter, includes an
    /// invalid parameter value, includes a parameter more than
    /// once, or is otherwise malformed.
    #[serde(alias = "invalid_request", alias = "InvalidRequest")]
    InvalidRequest,

    /// The client is not authorized to request an access token
    /// using this method.
    #[serde(alias = "unauthorized_client", alias = "UnauthorizedClient")]
    UnauthorizedClient,

    /// The resource owner or authorization server denied the
    /// request.
    #[serde(alias = "access_denied", alias = "AccessDenied")]
    AccessDenied,

    /// The authorization server does not support obtaining an
    /// access token using this method
    #[serde(alias = "unsupported_response_type", alias = "UnsupportedResponseType")]
    UnsupportedResponseType,

    /// The requested scope is invalid, unknown, or malformed.
    #[serde(alias = "invalid_scope", alias = "InvalidScope")]
    InvalidScope,

    /// The authorization server encountered an unexpected
    /// condition that prevented it from fulfilling the request.
    /// (This error code is needed because a 500 Internal Server
    /// Error HTTP status code cannot be returned to the client
    /// via a HTTP redirect.)
    #[serde(alias = "server_error", alias = "ServerError")]
    ServerError,

    /// The authorization server is currently unable to handle
    /// the request due to a temporary overloading or maintenance
    /// of the server.  (This error code is needed because a 503
    /// Service Unavailable HTTP status code cannot be returned
    /// to the client via a HTTP redirect.)
    #[serde(alias = "temporarily_unavailable", alias = "TemporarilyUnavailable")]
    TemporarilyUnavailable,

    /// The target resource is invalid because it doesn't exist, Microsoft Entra ID can't find it,
    /// or it's not correctly configured.
    ///
    /// The client requested silent authentication (prompt=none), but a single user couldn't be
    /// found. This error may mean there are multiple users active in the session, or no users.
    /// This error takes into account the tenant chosen. For example, if there are two Microsoft
    /// Entra accounts active and one Microsoft account, and consumers is chosen, silent
    /// authentication works.
    #[serde(alias = "invalid_resource", alias = "InvalidResource")]
    InvalidResource,

    /// Too many or no users found.
    /// The client requested silent authentication (prompt=none), but a single user couldn't be
    /// found. This error may mean there are multiple users active in the session, or no users.
    /// This error takes into account the tenant chosen. For example, if there are two Microsoft
    /// Entra accounts active and one Microsoft account, and consumers is chosen, silent
    /// authentication works.
    #[serde(alias = "login_required", alias = "LoginRequired")]
    LoginRequired,

    /// The request requires user interaction.
    /// Another authentication step or consent is required. Retry the request without prompt=none.
    #[serde(alias = "interaction_required", alias = "InteractionRequired")]
    InteractionRequired,
}

impl Display for AuthorizationQueryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:#?}")
    }
}

fn deserialize_expires_in<'de, D>(expires_in: D) -> Result<Option<i64>, D::Error>
where
    D: Deserializer<'de>,
{
    let expires_in_string_result: Result<String, D::Error> =
        serde::Deserialize::deserialize(expires_in);
    if let Ok(expires_in_string) = expires_in_string_result {
        if let Ok(expires_in) = expires_in_string.parse::<i64>() {
            return Ok(Some(expires_in));
        }
    }

    Ok(None)
}

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct PhantomAuthorizationResponse {
    pub code: Option<String>,
    pub id_token: Option<String>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_expires_in")]
    pub expires_in: Option<i64>,
    pub access_token: Option<String>,
    pub state: Option<String>,
    pub session_state: Option<String>,
    pub nonce: Option<String>,
    pub error: Option<AuthorizationQueryError>,
    pub error_description: Option<String>,
    pub error_uri: Option<Url>,
    #[serde(flatten)]
    pub additional_fields: HashMap<String, Value>,
    #[serde(skip)]
    log_pii: bool,
}

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct AuthorizationError {
    pub error: Option<AuthorizationQueryError>,
    pub error_description: Option<String>,
    pub error_uri: Option<Url>,
}

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct AuthorizationResponse {
    pub code: Option<String>,
    pub id_token: Option<String>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_expires_in")]
    pub expires_in: Option<i64>,
    pub access_token: Option<String>,
    pub state: Option<String>,
    pub session_state: Option<String>,
    pub nonce: Option<String>,
    pub error: Option<AuthorizationQueryError>,
    pub error_description: Option<String>,
    pub error_uri: Option<Url>,
    #[serde(flatten)]
    pub additional_fields: HashMap<String, Value>,
    #[serde(skip)]
    log_pii: bool,
}

impl AuthorizationResponse {
    /// Enable or disable logging of personally identifiable information such
    /// as logging the id_token. This is disabled by default. When log_pii is enabled
    /// passing [AuthorizationResponse] to logging or print functions will log both the bearer
    /// access token value of amy and the id token value.
    /// By default these do not get logged.
    pub fn enable_pii_logging(&mut self, log_pii: bool) {
        self.log_pii = log_pii;
    }

    pub fn is_err(&self) -> bool {
        self.error.is_some()
    }
}

impl Debug for AuthorizationResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.log_pii {
            f.debug_struct("AuthQueryResponse")
                .field("code", &self.code)
                .field("id_token", &self.id_token)
                .field("access_token", &self.access_token)
                .field("state", &self.state)
                .field("nonce", &self.nonce)
                .field("error", &self.error)
                .field("error_description", &self.error_description)
                .field("error_uri", &self.error_uri)
                .field("additional_fields", &self.additional_fields)
                .finish()
        } else {
            f.debug_struct("AuthQueryResponse")
                .field("code", &self.code)
                .field("id_token", &"[REDACTED]")
                .field("access_token", &"[REDACTED]")
                .field("state", &self.state)
                .field("nonce", &self.nonce)
                .field("error", &self.error)
                .field("error_description", &self.error_description)
                .field("error_uri", &self.error_uri)
                .field("additional_fields", &self.additional_fields)
                .finish()
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum AuthorizationImpeded {
    WindowClosed(String),
    InvalidUri(String),
}

#[derive(Clone, Debug)]
pub enum AuthorizationEvent<CredentialBuilder: Clone + Debug> {
    Authorized {
        authorization_response: AuthorizationResponse,
        credential_builder: CredentialBuilder,
    },
    Unauthorized(AuthorizationResponse),
    WindowClosed(String),
}

impl<CredentialBuilder: Clone + Debug> AuthorizationEvent<CredentialBuilder> {
    pub fn into_result(self) -> WebViewResult<(AuthorizationResponse, CredentialBuilder)> {
        match self {
            AuthorizationEvent::Authorized {
                authorization_response,
                credential_builder,
            } => Ok((authorization_response, credential_builder)),
            AuthorizationEvent::Unauthorized(authorization_response) => {
                Err(WebViewError::Authorization {
                    error: authorization_response
                        .error
                        .map(|query_error| query_error.to_string())
                        .unwrap_or_default(),
                    error_description: authorization_response.error_description.unwrap_or_default(),
                    error_uri: authorization_response.error_uri.map(|uri| uri.to_string()),
                })
            }
            AuthorizationEvent::WindowClosed(reason) => Err(WebViewError::WindowClosed(reason)),
        }
    }
}

pub trait IntoCredentialBuilder<CredentialBuilder: Clone + Debug> {
    fn into_credential_builder(self) -> WebViewResult<(AuthorizationResponse, CredentialBuilder)>;
}

impl<CredentialBuilder: Clone + Debug> IntoCredentialBuilder<CredentialBuilder>
    for WebViewResult<AuthorizationEvent<CredentialBuilder>>
{
    fn into_credential_builder(self) -> WebViewResult<(AuthorizationResponse, CredentialBuilder)> {
        match self {
            Ok(auth_event) => match auth_event {
                AuthorizationEvent::Authorized {
                    authorization_response,
                    credential_builder,
                } => Ok((authorization_response, credential_builder)),
                AuthorizationEvent::Unauthorized(authorization_response) => {
                    Err(WebViewError::Authorization {
                        error: authorization_response
                            .error
                            .map(|query_error| query_error.to_string())
                            .unwrap_or_default(),
                        error_description: authorization_response
                            .error_description
                            .unwrap_or_default(),
                        error_uri: authorization_response.error_uri.map(|uri| uri.to_string()),
                    })
                }
                AuthorizationEvent::WindowClosed(reason) => Err(WebViewError::WindowClosed(reason)),
            },
            Err(err) => Err(err),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    pub const AUTHORIZATION_RESPONSE: &str = r#"{
        "access_token": "token",
        "expires_in": "3600"
    }"#;

    pub const AUTHORIZATION_RESPONSE2: &str = r#"{
        "access_token": "token"
    }"#;

    #[test]
    pub fn deserialize_authorization_response_from_json() {
        let response: AuthorizationResponse = serde_json::from_str(AUTHORIZATION_RESPONSE).unwrap();
        assert_eq!(Some(String::from("token")), response.access_token);
        assert_eq!(Some(3600), response.expires_in);
    }

    #[test]
    pub fn deserialize_authorization_response_from_json2() {
        let response: AuthorizationResponse =
            serde_json::from_str(AUTHORIZATION_RESPONSE2).unwrap();
        assert_eq!(Some(String::from("token")), response.access_token);
    }

    #[test]
    pub fn deserialize_authorization_response_from_query() {
        let query = "access_token=token&expires_in=3600";
        let response: AuthorizationResponse = serde_urlencoded::from_str(query).unwrap();
        assert_eq!(Some(String::from("token")), response.access_token);
        assert_eq!(Some(3600), response.expires_in);
    }

    #[test]
    pub fn deserialize_authorization_response_from_query_without_expires_in() {
        let query = "access_token=token";
        let response: AuthorizationResponse = serde_urlencoded::from_str(query).unwrap();
        assert_eq!(Some(String::from("token")), response.access_token);
    }
}
