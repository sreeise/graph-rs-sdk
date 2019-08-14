use crate::drive::drive_item::application::Application;
use crate::drive::drive_item::user::User;
use std::io::Write;

#[derive(Default, Debug, Clone, Eq, PartialEq, Serialize, FromToFile, Deserialize, Setters)]
#[set = "pub set"]
pub struct CreatedBy {
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    application: Option<Application>,
}

impl CreatedBy {
    pub fn new(user: Option<User>, application: Option<Application>) -> CreatedBy {
        CreatedBy { user, application }
    }

    pub fn user(&self) -> Option<User> {
        self.user.clone()
    }

    pub fn application(&self) -> Option<Application> {
        self.application.clone()
    }
}
