// TODO: #292 before tests are brought back.

#[macro_use]
extern crate lazy_static;

use graph_rs::prelude::*;
use test_tools::assert_url_eq;
use test_tools::common::TestTools;

lazy_static! {
    static ref ID_VEC: Vec<String> = TestTools::random_strings(4, 20);
}

#[test]
pub fn list_messages() {
    let client = Graph::new("");
    let _ = client.v1().me().messages().list_messages();
    assert_url_eq(&client, "/me/messages");

    let _ = client
        .v1()
        .user(ID_VEC[0].as_str())
        .messages()
        .list_messages();
    assert_url_eq(&client, &format!("/users/{}/messages", ID_VEC[0]));
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
pub fn get_messages() {
    let client = Graph::new("");
    let _ = client.v1().me().message(ID_VEC[0].as_str()).get_messages();
    assert_url_eq(&client, &format!("/me/messages/{}", ID_VEC[0]));

    let _ = client
        .v1()
        .user(ID_VEC[0].as_str())
        .message(ID_VEC[1].as_str())
        .get_messages();
    assert_url_eq(
        &client,
        &format!("/users/{}/messages/{}", ID_VEC[0], ID_VEC[1]),
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
pub fn update_messages() {
    let client = Graph::new("");
    let _ = client
        .v1()
        .me()
        .message(ID_VEC[0].as_str())
        .update_messages(&String::new());
    assert_url_eq(&client, &format!("/me/messages/{}", ID_VEC[0]));
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
pub fn delete_messages() {
    let client = Graph::new("");
    let _ = client
        .v1()
        .me()
        .message(ID_VEC[0].as_str())
        .delete_messages();
    assert_url_eq(&client, &format!("/me/messages/{}", ID_VEC[0]));

    let _ = client
        .v1()
        .user(ID_VEC[0].as_str())
        .message(ID_VEC[1].as_str())
        .delete_messages();
    assert_url_eq(
        &client,
        &format!("/users/{}/messages/{}", ID_VEC[0], ID_VEC[1]),
    );
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

#[test]
pub fn create_reply_all_messages() {
    let client = Graph::new("");
    let _ = client
        .v1()
        .me()
        .message(ID_VEC[0].as_str())
        .create_reply_all(&String::new());
    assert_url_eq(
        &client,
        &format!("/me/messages/{}/createReplyAll", ID_VEC[0]),
    );
}

#[test]
pub fn copy_messages() {
    let client = Graph::new("");
    let _ = client
        .v1()
        .me()
        .message(ID_VEC[0].as_str())
        .copy(&String::new());
    assert_url_eq(&client, &format!("/me/messages/{}/copy", ID_VEC[0]));
}

#[test]
pub fn move_messages() {
    let client = Graph::new("");
    let _ = client
        .v1()
        .me()
        .message(ID_VEC[0].as_str())
        .move_message(&String::new());
    assert_url_eq(&client, &format!("/me/messages/{}/move", ID_VEC[0]));
}

#[test]
pub fn reply_messages() {
    let client = Graph::new("");
    let _ = client
        .v1()
        .me()
        .message(ID_VEC[0].as_str())
        .reply(&String::new());
    assert_url_eq(&client, &format!("/me/messages/{}/reply", ID_VEC[0]));
}

#[test]
pub fn reply_all_messages() {
    let client = Graph::new("");
    let _ = client
        .v1()
        .me()
        .message(ID_VEC[0].as_str())
        .reply_all(&String::new());
    assert_url_eq(&client, &format!("/me/messages/{}/replyAll", ID_VEC[0]));
}

#[test]
pub fn create_forward_messages() {
    let client = Graph::new("");
    let _ = client
        .v1()
        .me()
        .message(ID_VEC[0].as_str())
        .create_forward(&String::new());
    assert_url_eq(
        &client,
        &format!("/me/messages/{}/createForward", ID_VEC[0]),
    );
}

#[test]
pub fn forward_messages() {
    let client = Graph::new("");
    let _ = client
        .v1()
        .me()
        .message(ID_VEC[0].as_str())
        .forward(&String::new());
    assert_url_eq(&client, &format!("/me/messages/{}/forward", ID_VEC[0]));
}

#[test]
pub fn send_mail_messages() {
    let client = Graph::new("");
    let _ = client.v1().me().send_mail(&String::new());
    assert_url_eq(&client, "/me/sendMail");
}

#[test]
fn mail_outlook_category() {
    let client = Graph::new("");

    let _ = client.v1().me().outlook().list_master_categories();
    assert_url_eq(&client, "/me/outlook/masterCategories");

    let _ = client
        .v1()
        .user(ID_VEC[0].as_str())
        .outlook()
        .list_master_categories();
    assert_url_eq(
        &client,
        &format!("/users/{}/outlook/masterCategories", ID_VEC[0]),
    );
}
