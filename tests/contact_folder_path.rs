#[macro_use]
extern crate lazy_static;

use graph_rs_sdk::prelude::*;
use test_tools::{assert_url_eq, common::TestTools};

lazy_static! {
    static ref ID_VEC: Vec<String> = TestTools::random_strings(4, 20);
}

#[test]
fn get_contact_folder() {
    let client = Graph::new("");
    let _ = client
        .v1()
        .me()
        .contact_folder(ID_VEC[0].as_str())
        .get_contact_folders();
    assert_url_eq(
        &client,
        &format!("/me/contactFolders/{}", ID_VEC[0].as_str()),
    );
}

#[test]
fn contact_folder_delta() {
    let client = Graph::new("");
    let _ = client.v1().me().contact_folders().delta();
    assert_url_eq(&client, "/me/contactFolders/delta()");
}

#[test]
fn update_contact_folder() {
    let client = Graph::new("");
    let _ = client
        .v1()
        .me()
        .contact_folder(ID_VEC[0].as_str())
        .update_contact_folders(&String::new());
    assert_url_eq(
        &client,
        &format!("/me/contactFolders/{}", ID_VEC[0].as_str()),
    );
}

#[test]
fn list_child_folders() {
    let client = Graph::new("");
    let _ = client
        .v1()
        .me()
        .contact_folder(ID_VEC[0].as_str())
        .child_folders()
        .list_child_folders();
    assert_url_eq(
        &client,
        &format!("/me/contactFolders/{}/childFolders", ID_VEC[0].as_str()),
    );
}

#[test]
fn list_contacts() {
    let client = Graph::new("");

    let _ = client
        .v1()
        .me()
        .contact_folder(ID_VEC[0].as_str())
        .list_contacts();
    assert_url_eq(
        &client,
        &format!("/me/contactFolders/{}/contacts", ID_VEC[0].as_str()),
    );
}

#[test]
fn create_contacts() {
    let client = Graph::new("");
    let _ = client
        .v1()
        .me()
        .contact_folder(ID_VEC[0].as_str())
        .contacts()
        .create_contacts(&String::new());
    assert_url_eq(
        &client,
        &format!("/me/contactFolders/{}/contacts", ID_VEC[0].as_str()),
    );
}
