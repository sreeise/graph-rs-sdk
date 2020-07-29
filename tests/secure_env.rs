use test_tools::oauthrequest::OAuthRequest;

// Make sure secure environment variables are being decrypted correctly
// on Travis CI.
#[test]
fn travis_ci_env_variable() {
    if OAuthRequest::is_travis() {
        assert!(OAuthRequest::is_test_env_set());
    }
}

// Make sure secure environment variables are being decrypted correctly
// on Appveyor.
#[test]
fn appveyor_env_variable() {
    if OAuthRequest::is_appveyor() {
        assert!(OAuthRequest::is_test_env_set());
    }
}
