/*
           let code = query.get("code");
           let id_token = query.get("id_token");
           let access_token = query.get("access_token");
           let state = query.get("state");
           let nonce = query.get("nonce");
*/

use serde_json::Value;
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize)]
pub struct AuthResponseQuery {
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
