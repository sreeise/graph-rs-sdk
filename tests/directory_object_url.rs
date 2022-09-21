#[macro_use]
extern crate lazy_static;

use graph_http::BlockingHttpClient;
use graph_rs_sdk::prelude::*;
use test_tools::assert_url_eq;
use test_tools::common::TestTools;

lazy_static! {
    static ref ID_VEC: Vec<String> = TestTools::random_strings(2, 20);
}

fn get_graph() -> Graph<BlockingHttpClient> {
    Graph::new("")
}

#[test]
fn directory_objects() {
    let client = get_graph();
    client
        .v1()
        .directory_objects()
        .create_directory_object(&String::new());

    assert_url_eq(&client, "/directoryObjects");
}

#[test]
fn directory_object_id() {
    let client = get_graph();
    client
        .v1()
        .directory_object(ID_VEC[0].as_str())
        .check_member_groups(&String::new());

    assert_url_eq(
        &client,
        format!(
            "/directoryObjects/{}/microsoft.graph.checkMemberGroups",
            ID_VEC[0]
        ),
    );
}
