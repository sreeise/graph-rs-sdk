#[macro_use]
extern crate lazy_static;

use graph_rs_sdk::prelude::*;
use test_tools::assert_url_eq;
use test_tools::common::TestTools;

lazy_static! {
    static ref ID_VEC: Vec<String> = TestTools::random_strings(2, 20);
}

pub fn graph() -> Graph {
    Graph::new("")
}

#[test]
fn authentication_methods_policy() {
    let client = graph();

    client
        .v1()
        .authentication_methods_policy()
        .get_authentication_methods_policy();

    assert_url_eq(&client, "/authenticationMethodsPolicy");
}

#[test]
fn authentication_method_configurations() {
    let client = graph();

    client
        .v1()
        .authentication_methods_policy()
        .authentication_method_configurations()
        .list_authentication_method_configuration();

    assert_url_eq(
        &client,
        "/authenticationMethodsPolicy/authenticationMethodConfigurations",
    );

    client
        .v1()
        .authentication_methods_policy()
        .authentication_method_configuration(ID_VEC[0].as_str())
        .get_authentication_method_configuration();

    assert_url_eq(
        &client,
        format!(
            "/authenticationMethodsPolicy/authenticationMethodConfigurations/{}",
            ID_VEC[0]
        ),
    );
}
