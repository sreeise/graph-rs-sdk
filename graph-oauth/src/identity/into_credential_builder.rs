use std::fmt::Debug;

pub trait IntoCredentialBuilder<CredentialBuilder: Clone + Debug> {
    type Response;
    type Error: std::error::Error;

    fn into_credential_builder(self) -> Result<(Self::Response, CredentialBuilder), Self::Error>;
}
