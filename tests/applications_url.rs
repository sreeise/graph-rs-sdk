#[macro_use]
extern crate lazy_static;

use graph_rs_sdk::prelude::*;
use test_tools::common::TestTools;

lazy_static! {
    static ref ID_VEC: Vec<String> = TestTools::random_strings(2, 20);
}

#[test]
fn applications_url() {
    let client = Graph::new("");

    assert_eq!(
        "/v1.0/applications".to_string(),
        client.applications().list_application().url().path()
    );
    assert_eq!(
        "/v1.0/applications/getByIds".to_string(),
        client
            .applications()
            .get_by_ids(&String::new())
            .url()
            .path()
    );
}

#[test]
fn applications_rid_url() {
    let client = Graph::new("");

    assert_eq!(
        format!("/v1.0/applications/{}", ID_VEC[0]),
        client
            .application(ID_VEC[0].as_str())
            .get_application()
            .url()
            .path()
    );

    assert_eq!(
        format!(
            "/v1.0/applications/{}/extensionProperties/{}",
            ID_VEC[0], ID_VEC[1]
        ),
        client
            .application(ID_VEC[0].as_str())
            .get_extension_properties(ID_VEC[1].as_str())
            .url()
            .path()
    );
}
