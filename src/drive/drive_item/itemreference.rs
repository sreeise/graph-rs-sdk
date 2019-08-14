use crate::drive::drive_item::sharepointid::SharePointIds;
use std::io::Write;

// https://docs.microsoft.com/en-us/onedrive/developer/rest-api/resources/itemreference?view=odsp-graph-online
#[derive(
    Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize, FromToFile, Setters, Getters,
)]
#[set = "pub set"]
#[get = "pub"]
pub struct ItemReference {
    #[serde(rename = "driveId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    drive_id: Option<String>,
    #[serde(rename = "driveType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    drive_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(rename = "listId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    list_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<String>,
    #[serde(rename = "shareId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    share_id: Option<String>,
    #[serde(rename = "sharepointIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    share_point_ids: Option<SharePointIds>,
    #[serde(rename = "siteId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    site_id: Option<String>,
}
