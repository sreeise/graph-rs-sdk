use crate::drive::base::user::User;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SharedBy {
    user: Option<User>,
}

impl SharedBy {
    pub fn new(user: Option<User>) -> Self {
        SharedBy { user }
    }

    pub fn user(&self) -> Option<User> {
        self.user.clone()
    }
}
