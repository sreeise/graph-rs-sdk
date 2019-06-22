// https://docs.microsoft.com/en-us/onedrive/developer/rest-api/resources/itemactivitytimeset?view=odsp-graph-online
#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Setters, Getters)]
#[set = "pub set"]
#[get = "pub"]
pub struct ItemActivityTimeSet {
    #[serde(rename = "observedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    observed_date_time: Option<String>,
    #[serde(rename = "recordedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    recorded_date_time: Option<String>,
}
