use test_tools::oauth_request::OAuthTestClient;

// Most OAuth flows for Microsoft Graph require a browser
// so only those that don't are tested here.

#[test]
fn client_credentials_test() {
    if let Some(token) = OAuthTestClient::ClientCredentials.request_access_token() {
        assert!(!token.1.access_token.is_empty());
    }
}
