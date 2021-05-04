use graph_error::{GraphError, GraphResult};
use graph_http::NextSession;
use graph_rs_sdk::core::ResourceIdentity;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::ffi::OsString;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;
use test_tools::common::TestTools;
use test_tools::oauth::OAuthTestTool;
use test_tools::oauthrequest::{Environment, OAuthTestClient};
use test_tools::oauthrequest::{TestEnv, DRIVE_THROTTLE_MUTEX};
use test_tools::support::cleanup::CleanUp;

#[test]
fn runs_on_correct_envs() {
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
