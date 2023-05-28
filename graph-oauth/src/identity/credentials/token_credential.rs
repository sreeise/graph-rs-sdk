use crate::identity::{AuthorizationSerializer, TokenRequest};

pub trait TokenCredential: AuthorizationSerializer + TokenRequest {
    fn client_id(&self) -> &String;
}
