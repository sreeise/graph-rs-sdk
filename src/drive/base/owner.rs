use crate::drive::base::group::Group;
use crate::drive::base::user::User;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Owner {
    user: Option<User>,
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
