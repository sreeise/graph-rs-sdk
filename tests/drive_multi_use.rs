#[macro_use]
extern crate lazy_static;

use graph_http::BlockingHttpClient;
use graph_rs::prelude::*;
use test_tools::assert_url_eq;
use test_tools::common::TestTools;

lazy_static! {
    static ref ID_VEC: Vec<String> = TestTools::random_strings(4, 20);
}

pub fn graph() -> Graph<BlockingHttpClient> {
    Graph::new("")
}

#[test]
fn sites_to_drive() {
    let client = graph();

    client.v1().site(ID_VEC[0].as_str()).list_drives();

    assert_url_eq(&client, &format!("/sites/{}/drives", ID_VEC[0]));

    client
        .v1()
        .site(ID_VEC[0].as_str())
        .create_drives(&String::new());

    assert_url_eq(&client, &format!("/sites/{}/drives", ID_VEC[0]));

    client.v1().site(ID_VEC[0].as_str()).get_drive();

    assert_url_eq(&client, &format!("/sites/{}/drive", ID_VEC[0]));

    client
        .v1()
        .site(ID_VEC[0].as_str())
        .update_drive(&String::new());

    assert_url_eq(&client, &format!("/sites/{}/drive", ID_VEC[0]));
}
