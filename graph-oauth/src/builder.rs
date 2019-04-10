use crate::auth::OAuthReq;
use reqwest::Client;
use std::collections::btree_map::BTreeMap;

pub struct Builder {
    builder: BTreeMap<String, String>,
}

impl Builder {
    pub fn new() -> Builder {
        Builder {
            builder: BTreeMap::new(),
        }
    }

    pub fn insert<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: ToString,
        V: ToString,
    {
        self.builder.insert(key.to_string(), value.to_string());
        self
    }

    pub fn build(&mut self, url: &str) -> OAuthReq<Client> {
        let client = reqwest::Client::builder().build()?;
        client.get(url).json(&self.builder);
        Ok(client)
    }
}
