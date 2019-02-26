use crate::auth::OAuthCredential;

// https://tools.ietf.org/html/rfc6749#section-4.1
#[allow(dead_code)]
pub struct AuthorizationCodeGrant {
    response_type: OAuthCredential,
    client_id: OAuthCredential,
    redirect_uri: OAuthCredential,
    scope: String,
    state: Option<OAuthCredential>,
}

#[allow(dead_code)]
pub struct AccessCodeRequest {
    grant_type: String,
    code: String,
    redirect_uri: String,
    client_id: String,
}
