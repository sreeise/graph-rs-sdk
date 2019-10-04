use graph_error::GraphResult;
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Batch {
    id: usize,
    method: String,
    url: String,
    body: Option<serde_json::Value>,
    headers: HashMap<String, String>,
}

impl Batch {
    pub fn new(id: usize, method: &str, url: &str) -> GraphResult<Batch> {
        Ok(Batch {
            id,
            method: method.into(),
            url: url.into(),
            body: None,
            headers: Default::default(),
        })
    }

    pub fn body(&mut self, body: serde_json::Value) {
        self.body = Some(body);
    }

    pub fn header(&mut self, key: &str, value: &str) {
        self.headers.insert(key.into(), value.into());
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct BatchRequest {
    requests: Vec<Batch>,
}

impl BatchRequest {
    pub fn new() -> BatchRequest {
        BatchRequest {
            requests: Vec::new(),
        }
    }

    pub fn add(&mut self, batch: Batch) {
        self.requests.push(batch);
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct BatchResponse {
    responses: Vec<serde_json::Value>,
}
