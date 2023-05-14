use serde_json::Value;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

#[derive(Clone, Serialize, Deserialize)]
pub struct AuthQueryResponse {
    pub code: Option<String>,
    pub id_token: Option<String>,
    pub access_token: Option<String>,
    pub state: Option<String>,
    pub nonce: Option<String>,
    #[serde(flatten)]
    additional_fields: HashMap<String, Value>,
    #[serde(skip)]
    log_pii: bool,
}

impl Debug for AuthQueryResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.log_pii {
            f.debug_struct("AuthQueryResponse")
                .field("code", &self.code)
                .field("id_token", &self.id_token)
                .field("access_token", &self.access_token)
                .field("state", &self.state)
                .field("nonce", &self.nonce)
                .field("additional_fields(serde flatten)", &self.additional_fields)
                .finish()
        } else {
            f.debug_struct("AuthQueryResponse")
                .field("code", &self.code)
                .field("id_token", &"[REDACTED]")
                .field("access_token", &"[REDACTED]")
                .field("state", &self.state)
                .field("nonce", &self.nonce)
                .field("additional_fields(serde flatten)", &self.additional_fields)
                .finish()
        }
    }
}
