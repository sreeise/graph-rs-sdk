use std::io::Write;

// https://docs.microsoft.com/en-us/onedrive/developer/rest-api/resources/listinfo?view=odsp-graph-online
#[derive(
    Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize, FromToFile, Setters, Getters,
)]
#[set = "pub set"]
#[get = "pub"]
pub struct ListInfo {
    #[serde(rename = "@odata.type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    odata_type: Option<String>,
    #[serde(rename = "contentTypesEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    content_types_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hidden: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    template: Option<String>,
}
