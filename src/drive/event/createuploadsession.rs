use crate::drive::drive_item::filesysteminfo::FileSystemInfo;
use crate::drive::event::ConflictBehavior;

use crate::drive::ItemResult;
use graph_error::GraphFailure;

#[derive(Default, Debug, Clone, Serialize, Deserialize, Setters, Getters)]
#[set = "pub set"]
#[get = "pub"]
pub struct CreateUploadSession {
    #[serde(rename = "@microsoft.graph.conflictBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    microsoft_graph_conflict_behavior: Option<ConflictBehavior>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(rename = "fileSystemInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    file_system_info: Option<FileSystemInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, Setters, Getters)]
#[set = "pub set"]
#[get = "pub"]
pub struct UploadSessionJson {
    item: CreateUploadSession,
}

impl UploadSessionJson {
    pub fn new(item: CreateUploadSession) -> UploadSessionJson {
        UploadSessionJson { item }
    }

    pub fn as_json(&self) -> ItemResult<String> {
        serde_json::to_string(&self).map_err(GraphFailure::from)
    }
}
