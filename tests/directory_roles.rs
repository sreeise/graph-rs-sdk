use std::sync::LazyLock;

use graph_rs_sdk::*;
use test_tools::common::TestTools;

static ID_VEC: LazyLock<Vec<String>> = LazyLock::new(|| TestTools::random_strings(2, 20));

#[test]
fn directory_roles() {
    let client = Graph::new("");

    assert_eq!(
        "/v1.0/directoryRoles".to_string(),
        client.directory_roles().list_directory_role().url().path()
    );
}

#[test]
fn directory_role() {
    let client = Graph::new("");

    assert_eq!(
        format!("/v1.0/directoryRoles/{}", ID_VEC[0]),
        client
            .directory_role(ID_VEC[0].as_str())
            .get_directory_role()
            .url()
            .path()
    );
}
