use crate::drive::drive_item::group::Group;
use crate::drive::drive_item::user::User;
use std::io::Write;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, FromToFile, Setters, Getters)]
#[set = "pub set"]
#[get = "pub"]
pub struct Owner {
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    group: Option<Group>,
}

impl Owner {
    pub fn new(user: Option<User>, group: Option<Group>) -> Self {
        Owner { user, group }
    }
}
