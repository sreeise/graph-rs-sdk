use crate::identity::{AuthorizationSerializer, CredentialStoreType, TokenRequest};

pub trait TokenCredential: AuthorizationSerializer + TokenRequest {
    fn client_id(&self) -> &String;
}
