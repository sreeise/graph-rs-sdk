use std::io::Write;

// https://docs.microsoft.com/en-us/onedrive/developer/rest-api/resources/searchresult?view=odsp-graph-online
#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize, FromToFile, Setters, Getters)]
#[set = "pub set"]
#[get = "pub"]
pub struct SearchResult {
    #[serde(rename = "onClickTelemetryUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    on_click_telemetry_url: Option<String>,
}
