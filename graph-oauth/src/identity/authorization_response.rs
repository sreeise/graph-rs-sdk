/// Representation of the authorization response described in the [specification](https://www.rfc-editor.org/rfc/rfc6749.html#section-4.1.2)
///
///
/// The [specification](https://www.rfc-editor.org/rfc/rfc6749.html#section-4.1.2) states:
/// If the resource owner grants the access request, the authorization
/// server issues an authorization code and delivers it to the client by
/// adding the following parameters to the query component of the
/// redirection URI using the "application/x-www-form-urlencoded"
pub struct AuthorizationResponse {
    pub code: String,
    pub state: String,
}
