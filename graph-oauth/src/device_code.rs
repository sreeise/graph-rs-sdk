use serde_json::Value;
use std::collections::HashMap;
use std::time::Duration;

/// https://datatracker.ietf.org/doc/html/rfc8628#section-3.2
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct DeviceCode {
    pub device_code: String,
    pub expires_in: u64,
    /// OPTIONAL
    /// The minimum amount of time in seconds that the client
    /// SHOULD wait between polling requests to the token endpoint.  If no
    /// value is provided, clients MUST use 5 as the default.
    #[serde(default = "default_interval")]
    pub interval: Option<Duration>,
    pub message: String,
    pub user_code: String,
    pub verification_uri: String,
    pub verification_uri_complete: Option<String>,
    #[serde(flatten)]
    pub additional_fields: HashMap<String, Value>,
}

fn default_interval() -> Option<Duration> {
    Some(Duration::from_secs(5))
}
