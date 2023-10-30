use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

use serde_json::Value;
use url::Url;

/// https://datatracker.ietf.org/doc/html/draft-ietf-oauth-v2-31#section-4.2.2.1
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
}

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct AuthorizationQueryResponse {
    pub code: Option<String>,
    pub id_token: Option<String>,
    pub expires_in: Option<String>,
    pub access_token: Option<String>,
    pub state: Option<String>,
    pub nonce: Option<String>,
    pub error: Option<AuthorizationQueryError>,
    pub error_description: Option<String>,
    pub error_uri: Option<Url>,
    #[serde(flatten)]
    pub additional_fields: HashMap<String, Value>,
    #[serde(skip)]
    log_pii: bool,
}

impl AuthorizationQueryResponse {
    /// Enable or disable logging of personally identifiable information such
    /// as logging the id_token. This is disabled by default. When log_pii is enabled
    /// passing [AuthorizationQueryResponse] to logging or print functions will log both the bearer
    /// access token value of amy and the id token value.
    /// By default these do not get logged.
    pub fn enable_pii_logging(&mut self, log_pii: bool) {
        self.log_pii = log_pii;
    }
}

impl Debug for AuthorizationQueryResponse {
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
