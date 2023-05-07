use crate::identity::{EnvironmentCredential, PublicClientApplication};
use std::env::VarError;

pub struct PublicClientApplicationBuilder;

impl PublicClientApplicationBuilder {
    pub fn try_from_environment() -> Result<PublicClientApplication, VarError> {
        EnvironmentCredential::resource_owner_password_credential()
    }
}
