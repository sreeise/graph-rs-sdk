use test_tools::oauthrequest::OAuthRequest;

// Make sure secure environment variables are being decrypted correctly
// on Travis CI.
#[test]
fn travis_ci_env_variable() {
    if OAuthRequest::is_travis() {
        let (id, token) = OAuthRequest::request_access_token().unwrap();
        assert!(!token.bearer_token().is_empty());
        assert!(!id.is_empty());
    }
}

// Make sure secure environment variables are being decrypted correctly
// on Appveyor.
#[test]
fn appveyor_env_variable() {
    if OAuthRequest::is_appveyor() {
        let (id, token) = OAuthRequest::request_access_token().unwrap();
        assert!(!token.bearer_token().is_empty());
        assert!(!id.is_empty());
    }
}
