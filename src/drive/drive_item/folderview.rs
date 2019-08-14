use std::io::Write;

// https://docs.microsoft.com/en-us/onedrive/developer/rest-api/resources/folderview?view=odsp-graph-online
#[derive(
    Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize, FromToFile, Setters, Getters,
)]
#[set = "pub set"]
#[get = "pub"]
pub struct FolderView {
    #[serde(rename = "sortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    sort_by: Option<String>,
    #[serde(rename = "sortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    sort_order: Option<String>,
    #[serde(rename = "viewType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    view_type: Option<String>,
}
