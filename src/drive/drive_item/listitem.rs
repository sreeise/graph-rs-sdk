use crate::drive::drive_item::contenttypeinfo::ContentTypeInfo;
use crate::drive::drive_item::driveitem::DriveItem;
use crate::drive::drive_item::fieldvalueset::FieldValueSet;
use crate::drive::drive_item::identityset::IdentitySet;
use crate::drive::drive_item::itemactivity::ItemActivity;
use crate::drive::drive_item::itemreference::ItemReference;
use crate::drive::drive_item::listitemversion::ListItemVersion;
use crate::drive::drive_item::sharepointid::SharePointIds;
use std::io::Write;

// https://docs.microsoft.com/en-us/onedrive/developer/rest-api/resources/listitem?view=odsp-graph-online
#[derive(
    Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize, FromToFile, Setters, Getters,
)]
#[set = "pub set"]
#[get = "pub"]
pub struct ListItem {
    #[serde(rename = "contentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    content_type: Option<ContentTypeInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fields: Option<FieldValueSet>,
    #[serde(rename = "sharepointIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    sharepoint_ids: Option<SharePointIds>,
    #[serde(skip_serializing_if = "Option::is_none")]
    activities: Option<Vec<ItemActivity>>,
    #[serde(rename = "driveItem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    drive_item: Option<DriveItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    versions: Option<Vec<ListItemVersion>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(rename = "createdBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    created_by: Option<IdentitySet>,
    #[serde(rename = "createdDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    created_date_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(rename = "eTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    e_tag: Option<String>,
    #[serde(rename = "lastModifiedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    last_modified_by: Option<IdentitySet>,
    #[serde(rename = "lastModifiedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    last_modified_date_time: Option<String>,
    #[serde(rename = "parentReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_reference: Option<ItemReference>,
    #[serde(rename = "webUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    web_url: Option<String>,
}
