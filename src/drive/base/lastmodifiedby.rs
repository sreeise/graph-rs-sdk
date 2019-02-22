use crate::drive::base::application::Application;
use crate::drive::base::user::User;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LastModifiedBy {
    user: Option<User>,
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
