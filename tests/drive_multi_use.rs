#[macro_use]
extern crate lazy_static;

use graph_rs_sdk::prelude::*;
use test_tools::common::TestTools;

lazy_static! {
    static ref ID_VEC: Vec<String> = TestTools::random_strings(4, 20);
}

#[test]
fn sites_to_drive() {
    let client = Graph::new("");

    assert_eq!(
        format!("/v1.0/sites/{}/drives", ID_VEC[0]),
        client.site(ID_VEC[0].as_str()).list_drives().url().path()
    );

    /*
    assert_eq!(
        format!("/v1.0/sites/{}/drives", ID_VEC[0]),
        client
            .site(ID_VEC[0].as_str())
            .drive()
            .create_drive()
            .url()
            .path()
    );
     */

    assert_eq!(
        format!("/v1.0/sites/{}/drive", ID_VEC[0]),
        client.site(ID_VEC[0].as_str()).get_drive().url().path()
    );

    assert_eq!(
        format!("/v1.0/sites/{}/drive", ID_VEC[0]),
        client
            .site(ID_VEC[0].as_str())
            .drive()
            .update_drive(&String::new())
            .url()
            .path()
    );
}
