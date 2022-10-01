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
fn directory_role_templates() {
    let client = get_graph();
    client
        .v1()
        .directory_role_templates()
        .list_directory_role_template();

    assert_url_eq(&client, "/directoryRoleTemplates");
}

#[test]
fn directory_role_template() {
    let client = get_graph();
    client
        .v1()
        .directory_role_template(ID_VEC[0].as_str())
        .get_directory_role_template();

    assert_url_eq(&client, format!("/directoryRoleTemplates/{}", ID_VEC[0]));
}
