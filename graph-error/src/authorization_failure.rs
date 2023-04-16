#[derive(Debug, thiserror::Error)]
pub enum AuthorizationFailure {
    #[error("Required value missing:\n{0:#?}", name)]
    RequiredValue {
        name: String,
        message: Option<String>,
    },
}

impl AuthorizationFailure {
    pub fn required_value<T>(name: &str, message: Option<&str>) -> Result<T, AuthorizationFailure> {
        Err(AuthorizationFailure::RequiredValue {
            name: name.to_owned(),
            message: message.map(|s| s.to_owned()),
        })
    }
}
