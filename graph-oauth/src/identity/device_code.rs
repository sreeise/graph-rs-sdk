use serde_json::Value;
use std::collections::{BTreeSet, HashMap};
use std::str::FromStr;
use std::time::Duration;

/// https://datatracker.ietf.org/doc/html/rfc8628#section-3.2
/// The actual device code response that is received from Microsoft Graph
/// looks similar to the following:
///
/// ```json
/// {
///     "device_code": String("FABABAAEAAAD--DLA3VO7QrddgJg7WevrgJ7Czy_TDsDClt2ELoEC8ePWFs"),
///     "expires_in": Number(900),
///     "interval": Number(5),
///     "message": String("To sign in, use a web browser to open the page https://microsoft.com/devicelogin and enter the code FQK5HW3UF to authenticate."),
///     "user_code": String("FQK5HW3UF"),
///     "verification_uri": String("https://microsoft.com/devicelogin"),
/// }
/// ```
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct DeviceCode {
    ///  A long string used to verify the session between the client and the authorization server.
    /// The client uses this parameter to request the access token from the authorization server.
    pub device_code: String,
    /// The number of seconds before the device_code and user_code expire.
    pub expires_in: u64,
    /// OPTIONAL
    /// The minimum amount of time in seconds that the client
    /// SHOULD wait between polling requests to the token endpoint.  If no
    /// value is provided, clients MUST use 5 as the default.
    #[serde(default = "default_interval")]
    pub interval: Duration,
    ///  User friendly text response that can be used for display purpose.
    pub message: String,
    pub user_code: String,
    /// Verification URL where the user must navigate to authenticate using the device code
    /// and credentials.
    pub verification_uri: String,
    /// The verification_uri_complete response field is not included or supported
    /// by Microsoft at this time.
    pub verification_uri_complete: Option<String>,
    /// List of the scopes that would be held by token.
    pub scopes: Option<BTreeSet<String>>,
    pub error: Option<String>,
    #[serde(flatten)]
    pub additional_fields: HashMap<String, Value>,
}

fn default_interval() -> Duration {
    Duration::from_secs(5)
}

/// Response types used when polling for a device code
/// https://datatracker.ietf.org/doc/html/rfc8628#section-3.5
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum PollDeviceCodeType {
    /// The user hasn't finished authenticating, but hasn't canceled the flow.
    /// Repeat the request after at least interval seconds.
    AuthorizationPending,
    /// The end user denied the authorization request.
    /// Stop polling and revert to an unauthenticated state.
    AuthorizationDeclined,
    /// The device_code sent to the /token endpoint wasn't recognized.
    /// Verify that the client is sending the correct device_code in the request.
    BadVerificationCode,
    /// Value of expires_in has been exceeded and authentication is no longer possible
    /// with device_code.
    /// Stop polling and revert to an unauthenticated state.
    ExpiredToken,

    /// Not yet supported by Microsoft but listed in the specification.
    ///
    /// The authorization request was denied.
    AccessDenied,

    /// Not yet supported by Microsoft but listed in the specification.
    ///
    /// A variant of "authorization_pending", the authorization request is
    /// still pending and polling should continue, but the interval MUST
    /// be increased by 5 seconds for this and all subsequent requests.
    SlowDown,

    /// Indicates the value is not an actual PollDeviceCodeType - this is an internal type not a
    /// type used in Microsoft Identity Platform or in the OAuth2/OpenId specification.
    ///
    /// This is a catch all to prevent parsing errors and break from
    /// any loop that is used to poll for the device code.
    InvalidType,
}

impl FromStr for PollDeviceCodeType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "authorization_pending" => Ok(PollDeviceCodeType::AuthorizationPending),
            "authorization_declined" => Ok(PollDeviceCodeType::AuthorizationDeclined),
            "bad_verification_code" => Ok(PollDeviceCodeType::BadVerificationCode),
            "expired_token" => Ok(PollDeviceCodeType::ExpiredToken),
            "access_denied" => Ok(PollDeviceCodeType::AccessDenied),
            "slow_down" => Ok(PollDeviceCodeType::SlowDown),
            _ => Ok(PollDeviceCodeType::InvalidType),
        }
    }
}
