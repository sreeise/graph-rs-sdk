// use test_tools::oauthrequest::Environment;

// Make sure secure environment variables are being decrypted correctly
// on Travis CI.
#[test]
fn travis_ci_env_variable() {
    // Uncomment if testing out new secure environment variables
    // to make sure they get set correctly.
    // if Environment::is_travis() {
    // assert!(Environment::is_test_env_set());
    // }
}

// Make sure secure environment variables are being decrypted correctly
// on Appveyor.
#[test]
fn appveyor_env_variable() {
    // Uncomment if testing out new secure environment variables
    // to make sure they get set correctly.
    // if Environment::is_appveyor() {
    // assert!(Environment::is_test_env_set());
    // }
}
