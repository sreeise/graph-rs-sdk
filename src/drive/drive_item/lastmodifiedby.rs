use crate::drive::drive_item::application::Application;
use crate::drive::drive_item::user::User;
use std::io::Write;

#[derive(
    Default, Debug, Clone, Eq, PartialEq, Serialize, Deserialize, FromToFile, Setters, Getters,
)]
#[set = "pub set"]
#[get = "pub"]
pub struct LastModifiedBy {
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    application: Option<Application>,
}

impl LastModifiedBy {
    pub fn new(user: Option<User>, application: Option<Application>) -> Self {
        LastModifiedBy { user, application }
    }
}
