// https://docs.microsoft.com/en-us/onedrive/developer/rest-api/resources/datetimecolumn?view=odsp-graph-online
#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Setters, Getters)]
#[set = "pub set"]
#[get = "pub"]
pub struct DateTimeColumn {
    #[serde(rename = "displayAs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    display_as: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    format: Option<String>,
}
