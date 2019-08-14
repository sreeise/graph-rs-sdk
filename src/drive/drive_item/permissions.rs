use crate::drive::drive_item::identityset::IdentitySet;
use crate::drive::drive_item::itemreference::ItemReference;
use crate::drive::drive_item::sharinginvitation::SharingInvitation;
use crate::drive::drive_item::sharinglink::SharingLink;
use std::io::Write;

// https://docs.microsoft.com/en-us/onedrive/developer/rest-api/resources/permission?view=odsp-graph-online
#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize, FromToFile, Setters, Getters)]
#[set = "pub set"]
#[get = "pub"]
pub struct Permissions {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    #[serde(rename = "grantedTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    granted_to: Option<IdentitySet>,
    #[serde(rename = "grantedToIdentities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    granted_to_identities: Option<Vec<IdentitySet>>,
    #[serde(rename = "inheritedFrom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    inherited_from: Option<ItemReference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invitation: Option<SharingInvitation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    link: Option<SharingLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    roles: Option<Vec<String>>,
    #[serde(rename = "shareId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    share_id: Option<String>,
}
