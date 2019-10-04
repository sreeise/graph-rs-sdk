use test_tools::oauthrequest::OAuthRequest;

// Most OAuth flows for Microsoft Graph require a browser
// so only those that don't are tested here.

#[test]
fn client_credentials_test() {
    if let Some(token) = OAuthRequest::request_access_token() {
        assert!(token.1.bearer_token().len() > 0);
    }
}
