use crate::drive::drive_item::driveitem::DriveItem;
use crate::drive::drive_item::identityset::IdentitySet;
use crate::drive::drive_item::list::List;
use crate::drive::drive_item::listitem::ListItem;
use crate::drive::drive_item::site::Site;
use crate::drive::Root;

// https://docs.microsoft.com/en-us/onedrive/developer/rest-api/resources/shareddriveitem?view=odsp-graph-online
#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Setters, Getters)]
#[set = "pub set"]
#[get = "pub"]
pub struct SharedDriveItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    owner: Option<IdentitySet>,
    #[serde(rename = "driveItem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    drive_item: Option<DriveItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    items: Option<Vec<DriveItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    list: Option<List>,
    #[serde(rename = "listItem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    list_item: Option<ListItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    root: Option<Root>,
    #[serde(skip_serializing_if = "Option::is_none")]
    site: Option<Site>,
}
