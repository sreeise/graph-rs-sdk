use crate::oautherror::OAuthError;
use crate::stdop::StdOp;
use std::collections::HashMap;

#[derive(Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct SigningKeys {
    authorization_endpoint: String,
    token_endpoint: String,
    token_endpoint_auth_methods_supported: Vec<String>,
    jwks_uri: String,
    response_modes_supported: Vec<String>,
    subject_types_supported: Vec<String>,
    id_token_signing_alg_values_supported: Vec<String>,
    http_logout_supported: bool,
    frontchannel_logout_supported: bool,
    end_session_endpoint: String,
    response_types_supported: Vec<String>,
    scopes_supported: Vec<String>,
    issuer: String,
    claims_supported: Vec<String>,
    microsoft_multi_refresh_token: bool,
    check_session_iframe: String,
    userinfo_endpoint: String,
    #[serde(skip)]
    tenant_region_scope: String,
    cloud_instance_name: String,
    cloud_graph_host_name: String,
    msgraph_host: String,
    rbac_url: String,
}

impl SigningKeys {
    pub fn new() -> Result<SigningKeys, OAuthError> {
        let client = reqwest::Client::builder().build().unwrap();
        let url = String::from(
            "https://login.microsoftonline.com/common/.well-known/openid-configuration",
        );
        let response = client.get(&url).send();

        match response {
            Ok(mut t) => {
                let keys: SigningKeys = t.json()?;
                Ok(keys)
            },
            Err(e) => Err(OAuthError::from(e)),
        }
    }
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Keys {
    #[serde(skip_serializing_if = "Option::is_none")]
    kty: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "use")]
    _use: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    x5t: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    n: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    e: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    x5c: Option<Vec<String>>,
}

impl Keys {
    pub fn to_map(&self) -> HashMap<String, String> {
        let mut hashmap: HashMap<String, String> = HashMap::new();
        hashmap.insert("kty".into(), StdOp::convert_to_string(self.kty.clone()));
        hashmap.insert("use".into(), StdOp::convert_to_string(self._use.clone()));
        hashmap.insert("kid".into(), StdOp::convert_to_string(self.kid.clone()));
        hashmap.insert("x5t".into(), StdOp::convert_to_string(self.x5t.clone()));
        hashmap.insert("n".into(), StdOp::convert_to_string(self.n.clone()));
        hashmap.insert("e".into(), StdOp::convert_to_string(self.e.clone()));
        if let Some(x5) = &self.x5c {
            hashmap.insert("x5c".into(), x5[0].to_string());
        }
        hashmap
    }
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct JWTKeys {
    keys: Vec<Keys>,
}

impl JWTKeys {
    pub fn discovery() -> Result<JWTKeys, OAuthError> {
        let client = reqwest::Client::builder().build().unwrap();
        let url = String::from("https://login.microsoftonline.com/common/discovery/keys");
        let response = client.get(&url).send();

        match response {
            Ok(mut t) => {
                let keys: JWTKeys = t.json()?;
                Ok(keys)
            },
            Err(e) => Err(OAuthError::from(e)),
        }
    }

    pub fn keys(&self) -> Vec<Keys> {
        self.keys.to_vec()
    }

    pub fn key_map(&mut self) -> Vec<HashMap<String, String>> {
        let mut vec: Vec<HashMap<String, String>> = Vec::new();
        for key in self.keys.iter() {
            vec.push(key.to_map());
        }

        vec
    }
}
