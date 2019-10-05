#[derive(Clone, Debug, Default)]
pub struct BatchResponse {
    pub responses: Vec<serde_json::Value>,
}
