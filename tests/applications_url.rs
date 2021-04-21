#[macro_use]
extern crate lazy_static;

use graph_http::BlockingHttpClient;
use graph_rs_sdk::prelude::*;
use test_tools::assert_url_eq;
use test_tools::common::TestTools;

lazy_static! {
    static ref ID_VEC: Vec<String> = TestTools::random_strings(2, 20);
}

pub fn graph() -> Graph<BlockingHttpClient> {
    Graph::new("")
}

#[test]
fn applications_url() {
    let client = graph();

    client.v1().applications().list_application();
    assert_url_eq(&client, "/applications");

    client.v1().applications().get_by_ids(&String::new());
    assert_url_eq(&client, "/applications/getByIds");
}

#[test]
fn applications_rid_url() {
    let client = graph();

    client
        .v1()
        .application(ID_VEC[0].as_str())
        .get_application();

    assert_url_eq(&client, &format!("/applications/{}", ID_VEC[0]));

    client
        .v1()
        .application(ID_VEC[0].as_str())
        .get_extension_properties(ID_VEC[1].as_str());
    assert_url_eq(
        &client,
        &format!(
            "/applications/{}/extensionProperties/{}",
            ID_VEC[0], ID_VEC[1]
        ),
    );
}
