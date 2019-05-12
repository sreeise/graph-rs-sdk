use std::collections::HashMap;

use crate::stdop::StdOp;
use transform_request::RequestError;

#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Keys {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kty: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "use")]
    pub _use: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    x5t: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e: Option<String>,
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
    #[allow(dead_code)]
    pub fn discovery() -> Result<JWTKeys, RequestError> {
        let client = reqwest::Client::builder().build()?;
        let url = String::from("https://login.microsoftonline.com/common/discovery/keys");
        let response = client.get(&url).send();

        match response {
            Ok(mut t) => {
                let keys: JWTKeys = t.json()?;
                Ok(keys)
            },
            Err(e) => Err(RequestError::from(e)),
        }
    }

    #[allow(dead_code)]
    pub fn keys(&self) -> Vec<Keys> {
        self.keys.to_vec()
    }

    #[allow(dead_code)]
    pub fn key_map(&mut self) -> Vec<HashMap<String, String>> {
        let mut vec: Vec<HashMap<String, String>> = Vec::new();
        for key in self.keys.iter() {
            vec.push(key.to_map());
        }

        vec
    }
}
