use graph_rs_sdk::core::ResourceIdentity;
use test_tools::common::TestTools;
use test_tools::oauthrequest::{OAuthTestClient, TestEnv, AppRegistrationMap};

#[test]
fn runs_on_correct_envs() {
    if TestEnv::GitHub.is_env_set() {
        let creds_secret = std::env::var("APP_REGISTRATIONS");
        if let Ok(creds) = creds_secret {
            assert!(!creds.is_empty());
            let _: AppRegistrationMap =
                serde_json::from_str(&creds).unwrap();
        } else if let Err(err) = creds_secret {
            panic!("Error with APP_REGISTRATIONS test secret:\n{:#?}", err);
        }
    }

    if TestEnv::Local.is_env_set() || TestEnv::GitHub.is_env_set() {
        assert!(OAuthTestClient::graph_by_rid(ResourceIdentity::Applications).is_some());
    }
}

#[test]
fn list_applications() {
    if let Some((_id, client)) = OAuthTestClient::graph_by_rid(ResourceIdentity::Applications) {
        let response = client.v1().applications().list_application().send();
        TestTools::assert_success(
            &response,
            "List applications Resource Identity: Applications",
        );
    }
}
