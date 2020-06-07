use graph_error::GraphFailure;
use std::collections::HashMap;

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
        hashmap.insert("kty".into(), self.kty.clone().unwrap_or_default());
        hashmap.insert("use".into(), self._use.clone().unwrap_or_default());
        hashmap.insert("kid".into(), self.kid.clone().unwrap_or_default());
        hashmap.insert("x5t".into(), self.x5t.clone().unwrap_or_default());
        hashmap.insert("n".into(), self.n.clone().unwrap_or_default());
        hashmap.insert("e".into(), self.e.clone().unwrap_or_default());
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
    pub fn discovery() -> Result<JWTKeys, GraphFailure> {
        let client = reqwest::blocking::Client::builder().build()?;
        let url = String::from("https://login.microsoftonline.com/common/discovery/keys");
        let response = client.get(&url).send();

        match response {
            Ok(t) => {
                let keys: JWTKeys = t.json()?;
                Ok(keys)
            },
            Err(e) => Err(GraphFailure::from(e)),
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
