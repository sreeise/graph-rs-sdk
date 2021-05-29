#[macro_use]
extern crate lazy_static;

use graph_rs_sdk::prelude::*;
use test_tools::{assert_url_eq, common::TestTools};

lazy_static! {
    static ref ID_VEC: Vec<String> = TestTools::random_strings(4, 20);
}

#[test]
pub fn list_mail_folder_messages() {
    let client = Graph::new("");
    let _ = client
        .v1()
        .me()
        .mail_folder(ID_VEC[0].as_str())
        .messages()
        .list_messages();
    assert_url_eq(&client, &format!("/me/mailFolders/{}/messages", ID_VEC[0]));

    let _ = client
        .v1()
        .user(ID_VEC[0].as_str())
        .mail_folder(ID_VEC[1].as_str())
        .messages()
        .list_messages();
    assert_url_eq(
        &client,
        &format!("/users/{}/mailFolders/{}/messages", ID_VEC[0], ID_VEC[1]),
    );
}

#[test]
pub fn get_mail_folder_messages() {
    let client = Graph::new("");
    let _ = client
        .v1()
        .me()
        .mail_folder(ID_VEC[0].as_str())
        .message(ID_VEC[1].as_str())
        .get_messages();
    assert_url_eq(
        &client,
        &format!("/me/mailFolders/{}/messages/{}", ID_VEC[0], ID_VEC[1]),
    );
}

#[test]
pub fn update_mail_folder_messages() {
    let client = Graph::new("");
    let _ = client
        .v1()
        .me()
        .mail_folder(ID_VEC[0].as_str())
        .message(ID_VEC[1].as_str())
        .update_messages(&String::new());
    assert_url_eq(
        &client,
        &format!("/me/mailFolders/{}/messages/{}", ID_VEC[0], ID_VEC[1]),
    );
}

#[test]
pub fn create_mail_folder() {
    let client = Graph::new("");
    let _ = client
        .v1()
        .user(ID_VEC[0].as_str())
        .mail_folders()
        .create_mail_folders(&String::new());
    assert_url_eq(&client, &format!("/users/{}/mailFolders", ID_VEC[0]));
}

#[test]
pub fn delete_mail_folder_messages() {
    let client = Graph::new("");
    let _ = client
        .v1()
        .me()
        .mail_folder(ID_VEC[0].as_str())
        .message(ID_VEC[1].as_str())
        .delete_messages();
    assert_url_eq(
        &client,
        &format!(
            "/me/mailFolders/{}/messages/{}",
            ID_VEC[0].as_str(),
            ID_VEC[1].as_str()
        ),
    );

    let _ = client
        .v1()
        .user(ID_VEC[0].as_str())
        .mail_folder(ID_VEC[1].as_str())
        .message(ID_VEC[2].as_str())
        .delete_messages();
    assert_url_eq(
        &client,
        &format!(
            "/users/{}/mailFolders/{}/messages/{}",
            ID_VEC[0].as_str(),
            ID_VEC[1].as_str(),
            ID_VEC[2].as_str(),
        ),
    );
}

#[test]
pub fn create_reply_mail_folder_messages() {
    let client = Graph::new("");
    let _ = client
        .v1()
        .me()
        .mail_folder(ID_VEC[0].as_str())
        .message(ID_VEC[1].as_str())
        .create_reply(&String::new());
    assert_url_eq(
        &client,
        &format!(
            "/me/mailFolders/{}/messages/{}/createReply",
            ID_VEC[0], ID_VEC[1]
        ),
    );
}
