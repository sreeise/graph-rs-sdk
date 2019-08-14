use crate::drive::drive_item::columnlink::ColumnLink;
use crate::drive::drive_item::contenttypeorder::ContentTypeOrder;
use crate::drive::drive_item::itemreference::ItemReference;
use std::io::Write;

// https://docs.microsoft.com/en-us/onedrive/developer/rest-api/resources/contenttype?view=odsp-graph-online
#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize, FromToFile, Setters, Getters)]
#[set = "pub set"]
#[get = "pub"]
pub struct ContentType {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hidden: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(rename = "inheritedFrom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    inherited_from: Option<ItemReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    order: Option<ContentTypeOrder>,
    #[serde(rename = "parentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_id: Option<String>,
    #[serde(rename = "readOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    read_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sealed: Option<bool>,
    #[serde(rename = "columnLinks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    column_links: Option<Vec<ColumnLink>>,
}
