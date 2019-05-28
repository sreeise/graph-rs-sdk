use crate::drive::drive_item::group::Group;
use crate::drive::drive_item::user::User;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Setters)]
#[set = "pub set"]
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

    pub fn user(&self) -> Option<User> {
        self.user.clone()
    }

    pub fn group(&self) -> Option<Group> {
        self.group.clone()
    }
}
