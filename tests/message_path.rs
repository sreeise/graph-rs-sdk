#[macro_use]
extern crate lazy_static;

use graph_rs_sdk::prelude::*;
use test_tools::common::TestTools;

lazy_static! {
    static ref ID_VEC: Vec<String> = TestTools::random_strings(2, 20);
}

#[test]
pub fn list_messages() {
    let client = Graph::new("");

    assert_eq!(
        "/v1.0/me/messages",
        client.me().messages().list_messages().url().path()
    );

    assert_eq!(
        format!("/v1.0/users/{}/messages", ID_VEC[0]),
        client
            .user(ID_VEC[0].as_str())
            .messages()
            .list_messages()
            .url()
            .path()
    );
}

#[test]
pub fn get_messages() {
    let client = Graph::new("");

    assert_eq!(
        format!("/v1.0/me/messages/{}", ID_VEC[0]),
        client
            .me()
            .message(ID_VEC[0].as_str())
            .get_messages()
            .url()
            .path()
    );

    assert_eq!(
        format!("/v1.0/users/{}/messages/{}", ID_VEC[0], ID_VEC[1]),
        client
            .user(ID_VEC[0].as_str())
            .message(ID_VEC[1].as_str())
            .get_messages()
            .url()
            .path()
    );
}

#[test]
pub fn update_messages() {
    let client = Graph::new("");

    assert_eq!(
        format!("/v1.0/me/messages/{}", ID_VEC[0]),
        client
            .me()
            .message(ID_VEC[0].as_str())
            .update_messages(&String::new())
            .url()
            .path()
    );
}

#[test]
pub fn delete_messages() {
    let client = Graph::new("");

    assert_eq!(
        format!("/v1.0/me/messages/{}", ID_VEC[0]),
        client
            .me()
            .message(ID_VEC[0].as_str())
            .delete_messages()
            .url()
            .path()
    );

    assert_eq!(
        format!("/v1.0/users/{}/messages/{}", ID_VEC[0], ID_VEC[1]),
        client
            .user(ID_VEC[0].as_str())
            .message(ID_VEC[1].as_str())
            .delete_messages()
            .url()
            .path()
    );
}

#[test]
pub fn move_messages() {
    let client = Graph::new("");

    assert_eq!(
        format!("/v1.0/me/messages/{}/move", ID_VEC[0]),
        client
            .me()
            .message(ID_VEC[0].as_str())
            .move_message(&String::new())
            .url()
            .path()
    );
}

#[test]
pub fn reply_messages() {
    let client = Graph::new("");

    assert_eq!(
        format!(
            "/v1.0/users/{}/messages/{}/reply",
            ID_VEC[0],
            ID_VEC[1].as_str()
        ),
        client
            .user(ID_VEC[0].as_str())
            .message(ID_VEC[1].as_str())
            .reply(&String::new())
            .url()
            .path()
    );
}

#[test]
pub fn forward_messages() {
    let client = Graph::new("");

    assert_eq!(
        format!("/v1.0/me/messages/{}/forward", ID_VEC[0]),
        client
            .me()
            .message(ID_VEC[0].as_str())
            .forward(&String::new())
            .url()
            .path()
    );
}

#[test]
pub fn send_mail_messages() {
    let client = Graph::new("");

    assert_eq!(
        "/v1.0/me/sendMail".to_string(),
        client.me().send_mail(&String::new()).url().path()
    );
}

#[test]
fn mail_outlook_category() {
    let client = Graph::new("");

    assert_eq!(
        "/v1.0/me/outlook/masterCategories".to_string(),
        client.me().outlook().list_master_categories().url().path()
    );
}
