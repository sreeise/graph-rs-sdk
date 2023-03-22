#[macro_use]
extern crate lazy_static;

use graph_rs_sdk::prelude::*;
use test_tools::common::TestTools;

lazy_static! {
    static ref ID_VEC: Vec<String> = TestTools::random_strings(2, 20);
}

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
