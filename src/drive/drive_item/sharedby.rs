use crate::drive::drive_item::user::User;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Setters)]
#[set = "pub set"]
pub struct SharedBy {
    #[serde(skip_serializing_if = "Option::is_none")]
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
