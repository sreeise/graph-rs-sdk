use std::sync::LazyLock;

use graph_rs_sdk::*;
use test_tools::common::TestTools;

static ID_VEC: LazyLock<Vec<String>> = LazyLock::new(|| TestTools::random_strings(2, 20));

#[test]
fn directory_objects() {
    let client = Graph::new("");

    assert_eq!(
        "/v1.0/directoryObjects".to_string(),
        client
            .directory_objects()
            .create_directory_object(&String::new())
            .url()
            .path()
    );
}

#[test]
fn directory_object_id() {
    let client = Graph::new("");

    assert_eq!(
        format!("/v1.0/directoryObjects/{}/checkMemberGroups", ID_VEC[0]),
        client
            .directory_object(ID_VEC[0].as_str())
            .check_member_groups(&String::new())
            .url()
            .path()
    );
}
