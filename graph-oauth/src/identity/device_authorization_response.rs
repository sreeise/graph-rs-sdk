use std::collections::{BTreeSet, HashMap};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use serde_json::Value;

#[cfg(feature = "interactive-auth")]
use graph_core::http::JsonHttpResponse;

#[cfg(feature = "interactive-auth")]
use crate::web::WindowCloseReason;

#[cfg(feature = "interactive-auth")]
use crate::identity::{DeviceCodeCredential, PublicClientApplication};

/// The Device Authorization Response: the authorization server generates a unique device
/// verification code and an end-user code that are valid for a limited time and includes
/// them in the HTTP response body using the "application/json" format [RFC8259] with a
/// 200 (OK) status code
///
/// The actual [device code response](https://learn.microsoft.com/en-us/entra/identity-platform/v2-oauth2-device-code#device-authorization-response)
/// that is received from Microsoft Graph does not include the verification_uri_complete field
/// even though it's in the [specification](https://datatracker.ietf.org/doc/html/rfc8628#section-3.2).
/// The device code response from Microsoft Graph looks like similar to the following:
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
pub struct DeviceAuthorizationResponse {
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
    pub interval: u64,
    ///  User friendly text response that can be used for display purpose.
    pub message: String,
    /// A short string shown to the user that's used to identify the session on a secondary device.
    pub user_code: String,
    /// Verification URL where the user must navigate to authenticate using the device code
    /// and credentials.
    pub verification_uri: String,
    /// The verification_uri_complete response field is not included or supported
    /// by Microsoft at this time. It is included here because it is part of the
    /// [standard](https://datatracker.ietf.org/doc/html/rfc8628) and in the case
    /// that Microsoft decides to include it.
    pub verification_uri_complete: Option<String>,
    /// List of the scopes that would be held by token.
    pub scopes: Option<BTreeSet<String>>,
    #[serde(flatten)]
    pub additional_fields: HashMap<String, Value>,
}

fn default_interval() -> u64 {
    5
}

impl Display for DeviceAuthorizationResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}, {}, {}, {}, {}, {}, {:#?}, {:#?}",
            self.device_code,
            self.expires_in,
            self.interval,
            self.message,
            self.user_code,
            self.verification_uri,
            self.verification_uri_complete,
            self.scopes
        )
    }
}

/// Response types used when polling for a device code
/// https://datatracker.ietf.org/doc/html/rfc8628#section-3.5
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum PollDeviceCodeEvent {
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
}

impl PollDeviceCodeEvent {
    pub fn as_str(&self) -> &'static str {
        match self {
            PollDeviceCodeEvent::AuthorizationPending => "authorization_pending",
            PollDeviceCodeEvent::AuthorizationDeclined => "authorization_declined",
            PollDeviceCodeEvent::BadVerificationCode => "bad_verification_code",
            PollDeviceCodeEvent::ExpiredToken => "expired_token",
            PollDeviceCodeEvent::AccessDenied => "access_denied",
            PollDeviceCodeEvent::SlowDown => "slow_down",
        }
    }
}

impl FromStr for PollDeviceCodeEvent {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "authorization_pending" => Ok(PollDeviceCodeEvent::AuthorizationPending),
            "authorization_declined" => Ok(PollDeviceCodeEvent::AuthorizationDeclined),
            "bad_verification_code" => Ok(PollDeviceCodeEvent::BadVerificationCode),
            "expired_token" => Ok(PollDeviceCodeEvent::ExpiredToken),
            "access_denied" => Ok(PollDeviceCodeEvent::AccessDenied),
            "slow_down" => Ok(PollDeviceCodeEvent::SlowDown),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for PollDeviceCodeEvent {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Display for PollDeviceCodeEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(feature = "interactive-auth")]
#[derive(Debug)]
pub enum InteractiveDeviceCodeEvent {
    DeviceAuthorizationResponse {
        response: JsonHttpResponse,
        device_authorization_response: Option<DeviceAuthorizationResponse>,
    },
    PollDeviceCode {
        poll_device_code_event: PollDeviceCodeEvent,
        response: JsonHttpResponse,
    },
    WindowClosed(WindowCloseReason),
    SuccessfulAuthEvent {
        response: JsonHttpResponse,
        public_application: PublicClientApplication<DeviceCodeCredential>,
    },
}
