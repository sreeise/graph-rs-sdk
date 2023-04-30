use serde_json::Value;
use std::collections::HashMap;
use std::time::Duration;

#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct DeviceCode {
    pub device_code: String,
    pub expires_in: u64,
    pub interval: Duration,
    pub message: String,
    pub user_code: String,
    pub verification_uri: String,
    #[serde(flatten)]
    pub additional_fields: HashMap<String, Value>,
}
