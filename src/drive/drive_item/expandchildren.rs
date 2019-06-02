use crate::drive::drive_item::Root;
use crate::drive::filesysteminfo::FileSystemInfo;
use crate::drive::folder::Folder;
use crate::drive::parentreference::ParentReference;
use crate::drive::value::Value;

#[derive(Debug, Serialize, Deserialize, Getters, Setters)]
#[set = "pub set"]
pub struct ExpandChildren {
    #[serde(rename = "@odata.context")]
    odata_context: String,
    #[serde(rename = "createdDateTime")]
    created_date_time: String,
    id: String,
    #[serde(rename = "lastModifiedDateTime")]
    last_modified_date_time: String,
    name: String,
    #[serde(rename = "webUrl")]
    web_url: String,
    size: i64,
    #[serde(rename = "parentReference")]
    parent_reference: ParentReference,
    #[serde(rename = "fileSystemInfo")]
    file_system_info: FileSystemInfo,
    folder: Folder,
    root: Root,
    #[serde(rename = "children@odata.context")]
    children_odata_context: String,
    #[serde(rename = "children")]
    children: Vec<Value>,
}
