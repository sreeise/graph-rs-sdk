// https://docs.microsoft.com/en-us/onedrive/developer/rest-api/resources/renameaction?view=odsp-graph-online
#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Setters, Getters)]
#[set = "pub set"]
#[get = "pub"]
pub struct RenameAction {
    #[serde(rename = "oldName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    old_name: Option<String>,
    #[serde(rename = "newName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    new_name: Option<String>,
}
