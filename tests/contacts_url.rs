#[macro_use]
extern crate lazy_static;

use graph_rs_sdk::prelude::*;
use test_tools::common::TestTools;

lazy_static! {
    static ref ID_VEC: Vec<String> = TestTools::random_strings(4, 20);
}

#[test]
fn list_contacts() {
    let client = Graph::new("");

    assert_eq!(
        "/v1.0/me/contacts".to_string(),
        client.me().contacts().list_contacts().url().path()
    );

    assert_eq!(
        format!("/v1.0/me/contacts/{}", ID_VEC[0]),
        client
            .me()
            .contact(ID_VEC[0].as_str())
            .get_contacts()
            .url()
            .path()
    );

    assert_eq!(
        "/v1.0/me/contacts/delta()".to_string(),
        client.me().contacts().delta().url().path()
    );
}

#[test]
fn create_contacts() {
    let client = Graph::new("");

    assert_eq!(
        "/v1.0/me/contacts".to_string(),
        client
            .me()
            .contacts()
            .create_contacts(&String::new())
            .url()
            .path()
    );
}

#[test]
fn update_contacts() {
    let client = Graph::new("");

    assert_eq!(
        format!("/v1.0/me/contacts/{}", ID_VEC[0]),
        client
            .me()
            .contact(ID_VEC[0].as_str())
            .update_contacts(&String::new())
            .url()
            .path()
    );
}

#[test]
fn delete_contacts() {
    let client = Graph::new("");

    assert_eq!(
        format!("/v1.0/me/contacts/{}", ID_VEC[0]),
        client
            .me()
            .contact(ID_VEC[0].as_str())
            .delete_contacts()
            .url()
            .path()
    );
}
