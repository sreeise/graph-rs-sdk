#[macro_use]
extern crate lazy_static;

use graph_rs_sdk::*;
use test_tools::common::TestTools;

lazy_static! {
    static ref ID_VEC: Vec<String> = TestTools::random_strings(2, 20);
}

#[test]
fn directory_role_templates() {
    let client = Graph::new("");

    assert_eq!(
        "/v1.0/directoryRoleTemplates".to_string(),
        client
            .directory_role_templates()
            .list_directory_role_template()
            .url()
            .path()
    );
}

#[test]
fn directory_role_template() {
    let client = Graph::new("");

    assert_eq!(
        format!("/v1.0/directoryRoleTemplates/{}", ID_VEC[0]),
        client
            .directory_role_template(ID_VEC[0].as_str())
            .get_directory_role_template()
            .url()
            .path()
    );
}
