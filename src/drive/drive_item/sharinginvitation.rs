use crate::drive::drive_item::identityset::IdentitySet;
use std::io::Write;

// https://docs.microsoft.com/en-us/onedrive/developer/rest-api/resources/sharinginvitation?view=odsp-graph-online
#[derive(
    Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize, FromToFile, Setters, Getters,
)]
#[set = "pub set"]
#[get = "pub"]
pub struct SharingInvitation {
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,
    #[serde(rename = "invitedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    invited_by: Option<IdentitySet>,
    #[serde(rename = "signInRequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    sign_in_required: Option<bool>,
}
