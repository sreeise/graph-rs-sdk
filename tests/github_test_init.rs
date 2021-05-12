use test_tools::oauthrequest::{TestEnv, OAuthTestClient};
use graph_rs_sdk::core::ResourceIdentity;

#[test]
fn runs_on_correct_envs() {
    if TestEnv::Local.is_env_set() || TestEnv::GitHub.is_env_set() {
        assert!(OAuthTestClient::graph_by_rid(ResourceIdentity::Applications).is_some());
    }
}
