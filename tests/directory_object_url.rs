#[macro_use]
extern crate lazy_static;

use graph_rs_sdk::prelude::*;
use test_tools::common::TestTools;

lazy_static! {
    static ref ID_VEC: Vec<String> = TestTools::random_strings(2, 20);
}

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
