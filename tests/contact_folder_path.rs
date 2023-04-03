#[macro_use]
extern crate lazy_static;

use graph_rs_sdk::*;
use test_tools::common::TestTools;

lazy_static! {
    static ref ID_VEC: Vec<String> = TestTools::random_strings(4, 20);
}

#[test]
fn get_contact_folder() {
    let client = Graph::new("");

    assert_eq!(
        format!("/v1.0/me/contactFolders/{}", ID_VEC[0].as_str()),
        client
            .me()
            .contact_folder(ID_VEC[0].as_str())
            .get_contact_folders()
            .url()
            .path()
    );
}

#[test]
fn contact_folder_delta() {
    let client = Graph::new("");

    assert_eq!(
        "/v1.0/me/contactFolders/delta()".to_string(),
        client.me().contact_folders().delta().url().path()
    );
}

#[test]
fn update_contact_folder() {
    let client = Graph::new("");

    assert_eq!(
        format!("/v1.0/me/contactFolders/{}", ID_VEC[0].as_str()),
        client
            .me()
            .contact_folder(ID_VEC[0].as_str())
            .update_contact_folders(&String::new())
            .url()
            .path()
    );
}

#[test]
fn list_child_folders() {
    let client = Graph::new("");

    assert_eq!(
        format!(
            "/v1.0/me/contactFolders/{}/childFolders",
            ID_VEC[0].as_str()
        ),
        client
            .me()
            .contact_folder(ID_VEC[0].as_str())
            .child_folders()
            .list_child_folders()
            .url()
            .path()
    );
}

#[test]
fn list_contacts() {
    let client = Graph::new("");

    assert_eq!(
        format!("/v1.0/me/contactFolders/{}/contacts", ID_VEC[0].as_str()),
        client
            .me()
            .contact_folder(ID_VEC[0].as_str())
            .contacts()
            .list_contacts()
            .url()
            .path()
    );
}

#[test]
fn create_contacts() {
    let client = Graph::new("");

    assert_eq!(
        format!("/v1.0/me/contactFolders/{}/contacts", ID_VEC[0].as_str()),
        client
            .me()
            .contact_folder(ID_VEC[0].as_str())
            .contacts()
            .create_contacts(&String::new())
            .url()
            .path()
    );
}
