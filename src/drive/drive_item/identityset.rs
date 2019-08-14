use crate::drive::drive_item::application::Application;
use crate::drive::drive_item::group::Group;
use crate::drive::drive_item::identity::Identity;
use crate::drive::drive_item::user::User;
use std::io::Write;

// https://docs.microsoft.com/en-us/onedrive/developer/rest-api/resources/identityset?view=odsp-graph-online
#[derive(
    Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize, FromToFile, Setters, Getters,
)]
#[set = "pub set"]
#[get = "pub"]
pub struct IdentitySet {
    #[serde(skip_serializing_if = "Option::is_none")]
    application: Option<Application>,
    #[serde(skip_serializing_if = "Option::is_none")]
    device: Option<Identity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group: Option<Group>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<User>,
}
