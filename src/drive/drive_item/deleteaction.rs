use std::io::Write;

// https://docs.microsoft.com/en-us/onedrive/developer/rest-api/resources/deleteaction?view=odsp-graph-online
#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize, FromToFile, Setters, Getters)]
#[set = "pub set"]
#[get = "pub"]
pub struct DeleteAction {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(rename = "objectType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    object_type: Option<String>,
}
