use std::io::Write;

// https://docs.microsoft.com/en-us/onedrive/developer/rest-api/resources/versionaction?view=odsp-graph-online
#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize, FromToFile, Setters, Getters)]
#[set = "pub set"]
#[get = "pub"]
pub struct VersionAction {
    #[serde(rename = "newVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    new_version: Option<String>,
}
