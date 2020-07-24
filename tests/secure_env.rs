use test_tools::oauthrequest::OAuthRequest;

// Make sure secure environment variables are being decrypted correctly
// on Travis CI.
#[test]
fn test_decrypted_env() {
    if OAuthRequest::is_travis() {
        let (id, token) = OAuthRequest::request_access_token().unwrap();
        assert!(!token.bearer_token().is_empty());
        assert!(!id.is_empty());
    }
}