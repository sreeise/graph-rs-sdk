#[macro_use]
extern crate lazy_static;

use graph_rs_sdk::prelude::*;
use test_tools::{assert_url_eq, common::TestTools};

lazy_static! {
    static ref ID_VEC: Vec<String> = TestTools::random_strings(4, 20);
}

#[test]
fn list_contacts() {
    let client = Graph::new("");
    let _ = client.v1().me().contacts().list_contacts();
    assert_url_eq(&client, "/me/contacts");
}

#[test]
fn get_contacts() {
    let client = Graph::new("");
    let _ = client.v1().me().contact(ID_VEC[0].as_str()).get_contacts();
    assert_url_eq(&client, &format!("/me/contacts/{}", ID_VEC[0]));
}

#[test]
fn contacts_delta() {
    let client = Graph::new("");
    let _ = client.v1().me().contacts().delta();
    assert_url_eq(&client, "/me/contacts/delta()");
}

#[test]
fn create_contacts() {
    let client = Graph::new("");
    let _ = client.v1().me().contacts().create_contacts(&String::new());
    assert_url_eq(&client, "/me/contacts");
}

#[test]
fn update_contacts() {
    let client = Graph::new("");
    let _ = client
        .v1()
        .me()
        .contact(ID_VEC[0].as_str())
        .update_contacts(&String::new());
    assert_url_eq(&client, &format!("/me/contacts/{}", ID_VEC[0]));
}

#[test]
fn delete_contacts() {
    let client = Graph::new("");
    let _ = client
        .v1()
        .me()
        .contact(ID_VEC[0].as_str())
        .delete_contacts();
    assert_url_eq(&client, &format!("/me/contacts/{}", ID_VEC[0]));
}
