use from_as::*;

// https://docs.microsoft.com/en-us/onedrive/developer/rest-api/resources/asyncjobstatus?view=odsp-graph-online
#[derive(Default, Debug, Clone, Serialize, Deserialize, FromFile, AsFile, Setters, Getters)]
#[set = "pub set"]
#[get = "pub"]
pub struct AsyncJobStatus {
    #[serde(skip_serializing_if = "Option::is_none")]
    operation: Option<String>,
    #[serde(rename = "percentageComplete")]
    #[serde(skip_serializing_if = "Option::is_none")]
    percentage_complete: Option<f64>,
    #[serde(rename = "resourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    resource_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<String>,
    #[serde(rename = "statusDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    status_description: Option<String>,
}
