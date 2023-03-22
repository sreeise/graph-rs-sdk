#[macro_use]
extern crate lazy_static;

use graph_rs_sdk::prelude::*;
use test_tools::common::TestTools;

lazy_static! {
    static ref ID_VEC: Vec<String> = TestTools::random_strings(2, 20);
}

pub fn graph() -> Graph {
    Graph::new("")
}

#[test]
fn authentication_methods_policy() {
    let client = Graph::new("");

    assert_eq!(
        "/v1.0/authenticationMethodsPolicy".to_string(),
        client
            .authentication_methods_policy()
            .get_authentication_methods_policy()
            .url()
            .path()
    );
}

#[test]
fn authentication_method_configurations() {
    let client = Graph::new("");

    assert_eq!(
        "/v1.0/authenticationMethodsPolicy/authenticationMethodConfigurations".to_string(),
        client
            .authentication_methods_policy()
            .authentication_method_configurations()
            .list_authentication_method_configuration()
            .url()
            .path()
    );

    assert_eq!(
        format!(
            "/v1.0/authenticationMethodsPolicy/authenticationMethodConfigurations/{}",
            ID_VEC[0]
        ),
        client
            .authentication_methods_policy()
            .authentication_method_configuration(ID_VEC[0].as_str())
            .get_authentication_method_configuration()
            .url()
            .path()
    );
}
