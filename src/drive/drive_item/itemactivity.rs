use crate::drive::drive_item::driveitem::DriveItem;
use crate::drive::drive_item::identityset::IdentitySet;
use crate::drive::drive_item::itemactionset::ItemActionSet;
use crate::drive::drive_item::itemactivitytimeset::ItemActivityTimeSet;
use crate::drive::drive_item::listitem::ListItem;
use std::io::Write;

// https://docs.microsoft.com/en-us/onedrive/developer/rest-api/resources/itemactivity?view=odsp-graph-online
#[derive(
    Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize, FromToFile, Setters, Getters,
)]
#[set = "pub set"]
#[get = "pub"]
pub struct ItemActivity {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<ItemActionSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    actor: Option<IdentitySet>,
    #[serde(rename = "driveItem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    drive_item: Option<DriveItem>,
    #[serde(rename = "listItem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    list_item: Option<ListItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    times: Option<ItemActivityTimeSet>,
}
