use std::env;
use test_tools::ci::CI;

#[test]
fn travis_works() {
    if CI::is_travis() {
        let app_id = env::var("TEST_APP_ID").expect("missing env APP_ID");
        let tenant = env::var("TEST_APP_TENANT").expect("missing env TENANT");
        let user_id = env::var("TEST_APP_USER_ID").expect("missing env USER_ID");
        let secret = env::var("TEST_APP_SECRET").expect("missing env SECRET");
        let user_name = env::var("TEST_APP_USER_NAME").expect("missing env USER_NAME");
        let password = env::var("TEST_APP_PASSWORD").expect("missing env PASSWORD");

        assert!(app_id.len() > 0);
        assert!(secret.len() > 0);
        assert!(tenant.len() > 0);
        assert!(user_id.len() > 0);
        assert!(user_name.len() > 0);
        assert!(password.len() > 0);
    }
}
