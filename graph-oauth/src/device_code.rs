use serde_json::Value;
use std::collections::{BTreeSet, HashMap};
use std::time::Duration;

/// https://datatracker.ietf.org/doc/html/rfc8628#section-3.2
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
    pub interval: Option<Duration>,
    ///  User friendly text response that can be used for display purpose.
    pub message: String,
    pub user_code: String,
    /// Verification URL where the user must navigate to authenticate using the device code
    /// and credentials.
    pub verification_uri: String,
    pub verification_uri_complete: Option<String>,
    /// List of the scopes that would be held by token.
    pub scopes: Option<BTreeSet<String>>,
    #[serde(flatten)]
    pub additional_fields: HashMap<String, Value>,
}

fn default_interval() -> Option<Duration> {
    Some(Duration::from_secs(5))
}
