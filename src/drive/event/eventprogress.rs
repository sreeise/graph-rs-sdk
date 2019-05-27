/// The progress status of long running events.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventProgress {
    pub operation: Option<String>,
    #[serde(rename = "percentageComplete")]
    pub percentage_complete: f64,
    #[serde(rename = "resourceId")]
    resource_id: Option<String>,
    pub status: String,
}

impl EventProgress {
    #[allow(dead_code)]
    pub fn resource_id(&self) -> Option<String> {
        self.resource_id.clone()
    }

    pub fn is_completed(&self) -> bool {
        self.status.as_str() == "completed"
    }
}
