use crate::drive::drive_item::columndefinition::ColumnDefinition;
use crate::drive::drive_item::contenttype::ContentType;
use crate::drive::drive_item::drive::Drive;
use crate::drive::drive_item::identityset::IdentitySet;
use crate::drive::drive_item::itemactivity::ItemActivity;
use crate::drive::drive_item::itemreference::ItemReference;
use crate::drive::drive_item::listinfo::ListInfo;
use crate::drive::drive_item::listitem::ListItem;
use crate::drive::drive_item::sharepointid::SharePointIds;

// https://docs.microsoft.com/en-us/onedrive/developer/rest-api/resources/list?view=odsp-graph-online
#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Setters, Getters)]
#[set = "pub set"]
#[get = "pub"]
pub struct List {
    #[serde(skip_serializing_if = "Option::is_none")]
    activities: Option<Vec<ItemActivity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    columns: Option<Vec<ColumnDefinition>>,
    #[serde(rename = "contentTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    content_types: Option<Vec<ContentType>>,
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    drive: Option<Drive>,
    #[serde(skip_serializing_if = "Option::is_none")]
    items: Option<Vec<ListItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    list: Option<ListInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<bool>,
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
    #[serde(rename = "sharepointIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    sharepoint_ids: Option<SharePointIds>,
    #[serde(rename = "webUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    web_url: Option<String>,
}
