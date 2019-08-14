use crate::drive::drive_item::driveitem::DriveItem;
use crate::drive::drive_item::identityset::IdentitySet;
use crate::drive::drive_item::itemactivity::ItemActivity;
use crate::drive::drive_item::owner::Owner;
use crate::drive::drive_item::quota::Quota;
use crate::drive::drive_item::sharepointid::SharePointIds;
use std::collections::BTreeMap;
use std::io::Write;

// https://docs.microsoft.com/en-us/onedrive/developer/rest-api/resources/drive?view=odsp-graph-online
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, FromToFile, Setters, Getters)]
#[set = "pub set"]
#[get = "pub"]
pub struct Drive {
    #[serde(skip_serializing_if = "Option::is_none")]
    activities: Option<Vec<ItemActivity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(rename = "createdBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    created_by: Option<IdentitySet>,
    #[serde(rename = "createdDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    created_date_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(rename = "driveType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    drive_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    items: Option<Vec<DriveItem>>,
    #[serde(rename = "lastModifiedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    last_modified_by: Option<IdentitySet>,
    #[serde(rename = "lastModifiedDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    last_modified_date_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    owner: Option<Owner>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quota: Option<Quota>,
    #[serde(skip_serializing_if = "Option::is_none")]
    root: Option<DriveItem>,
    #[serde(rename = "sharepointIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    sharepoint_ids: Option<SharePointIds>,
    #[serde(skip_serializing_if = "Option::is_none")]
    special: Option<Vec<DriveItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<BTreeMap<String, serde_json::Value>>,
    #[serde(rename = "webUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    web_url: Option<String>,
}

impl Eq for Drive {}
