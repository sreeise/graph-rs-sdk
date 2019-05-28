use crate::drive::drive_item::application::Application;
use crate::drive::drive_item::user::User;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Setters)]
#[set = "pub set"]
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

    pub fn user(&self) -> Option<User> {
        self.user.clone()
    }

    pub fn application(&self) -> Option<Application> {
        self.application.clone()
    }
}
