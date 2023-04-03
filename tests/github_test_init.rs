use graph_rs_sdk::core::ResourceIdentity;
use test_tools::oauth_request::{OAuthTestClient, TestEnv};

#[test]
fn runs_on_correct_envs() {
    if TestEnv::Local.is_env_set() || TestEnv::GitHub.is_env_set() {
        assert!(OAuthTestClient::graph_by_rid(ResourceIdentity::Applications).is_some());
    }
}
