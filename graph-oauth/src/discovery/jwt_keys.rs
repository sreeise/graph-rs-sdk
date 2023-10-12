use std::collections::HashMap;

use graph_error::GraphResult;

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
    pub x5t: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x5c: Option<Vec<String>>,
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
    pub fn discovery() -> GraphResult<JWTKeys> {
        let client = reqwest::blocking::Client::new();
        let response = client
            .get("https://login.microsoftonline.com/common/discovery/keys")
            .send()?;
        let keys: JWTKeys = response.json()?;
        Ok(keys)
    }

    pub async fn async_discovery() -> GraphResult<JWTKeys> {
        let client = reqwest::Client::new();
        let response = client
            .get("https://login.microsoftonline.com/common/discovery/keys")
            .send()
            .await?;
        let keys: JWTKeys = response.json().await?;
        Ok(keys)
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

impl IntoIterator for JWTKeys {
    type Item = Keys;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.keys.into_iter()
    }
}
