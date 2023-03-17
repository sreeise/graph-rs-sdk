#[macro_use]
extern crate lazy_static;

use graph_rs_sdk::prelude::*;
use test_tools::common::TestTools;

lazy_static! {
    static ref ID_VEC: Vec<String> = TestTools::random_strings(4, 20);
}

#[test]
pub fn list_mail_folder_messages() {
    let client = Graph::new("");
    assert_eq!(
        format!("/v1.0/me/mailFolders/{}/messages", ID_VEC[0]),
        client
            .me()
            .mail_folder(ID_VEC[0].as_str())
            .messages()
            .list_messages()
            .url()
            .path()
    );

    assert_eq!(
        format!(
            "/v1.0/users/{}/mailFolders/{}/messages",
            ID_VEC[0], ID_VEC[1]
        ),
        client
            .user(ID_VEC[0].as_str())
            .mail_folder(ID_VEC[1].as_str())
            .messages()
            .list_messages()
            .url()
            .path()
    );
}

#[test]
pub fn get_mail_folder_messages() {
    let client = Graph::new("");
    assert_eq!(
        format!(
            "/v1.0/users/{}/mailFolders/{}/messages/{}",
            ID_VEC[0], ID_VEC[1], ID_VEC[2]
        ),
        client
            .user(ID_VEC[0].as_str())
            .mail_folder(ID_VEC[1].as_str())
            .messages_id(ID_VEC[2].as_str())
            .get_messages()
            .url()
            .path()
    );

    assert_eq!(
        format!("/v1.0/me/mailFolders/{}/messages/{}", ID_VEC[0], ID_VEC[1]),
        client
            .me()
            .mail_folder(ID_VEC[0].as_str())
            .messages_id(ID_VEC[1].as_str())
            .get_messages()
            .url()
            .path()
    );
}

#[test]
pub fn update_mail_folder_messages() {
    let client = Graph::new("");
    assert_eq!(
        format!("/v1.0/me/mailFolders/{}/messages/{}", ID_VEC[0], ID_VEC[1]),
        client
            .me()
            .mail_folder(ID_VEC[0].as_str())
            .messages_id(ID_VEC[1].as_str())
            .update_messages(&String::new())
            .url()
            .path()
    );
}

#[test]
pub fn delete_mail_folder_messages() {
    let client = Graph::new("");
    assert_eq!(
        format!(
            "/v1.0/me/mailFolders/{}/messages/{}",
            ID_VEC[0].as_str(),
            ID_VEC[1].as_str()
        ),
        client
            .me()
            .mail_folder(ID_VEC[0].as_str())
            .messages_id(ID_VEC[1].as_str())
            .update_messages(&String::new())
            .url()
            .path()
    );

    assert_eq!(
        format!(
            "/v1.0/users/{}/mailFolders/{}/messages/{}",
            ID_VEC[0].as_str(),
            ID_VEC[1].as_str(),
            ID_VEC[2].as_str()
        ),
        client
            .user(ID_VEC[0].as_str())
            .mail_folder(ID_VEC[1].as_str())
            .messages_id(ID_VEC[2].as_str())
            .update_messages(&String::new())
            .url()
            .path()
    );
}

#[test]
pub fn child_folders() {
    let client = Graph::new("");
    assert_eq!(
        format!(
            "/v1.0/me/mailFolders/{}/childFolders/$count",
            ID_VEC[0].as_str()
        ),
        client
            .me()
            .mail_folder(ID_VEC[0].as_str())
            .child_folders()
            .get_child_folders_count()
            .url()
            .path()
    );

    assert_eq!(
        format!(
            "/v1.0/users/{}/mailFolders/{}/childFolders/{}/messages/{}",
            ID_VEC[0].as_str(),
            ID_VEC[1].as_str(),
            ID_VEC[2].as_str(),
            ID_VEC[3].as_str()
        ),
        client
            .user(ID_VEC[0].as_str())
            .mail_folder(ID_VEC[1].as_str())
            .child_folder(ID_VEC[2].as_str())
            .messages_id(ID_VEC[3].as_str())
            .get_messages()
            .url()
            .path()
    );

    assert_eq!(
        format!(
            "/v1.0/users/{}/mailFolders/{}/childFolders/{}/messageRules/{}",
            ID_VEC[0].as_str(),
            ID_VEC[1].as_str(),
            ID_VEC[2].as_str(),
            ID_VEC[3].as_str()
        ),
        client
            .user(ID_VEC[0].as_str())
            .mail_folder(ID_VEC[1].as_str())
            .child_folder(ID_VEC[2].as_str())
            .delete_message_rules(ID_VEC[3].as_str())
            .url()
            .path()
    );

    assert_eq!(
        format!(
            "/v1.0/users/{}/mailFolders/{}/childFolders/{}/messages/{}/extensions/{}",
            ID_VEC[0].as_str(),
            ID_VEC[1].as_str(),
            ID_VEC[2].as_str(),
            ID_VEC[3].as_str(),
            ID_VEC[0].as_str()
        ),
        client
            .user(ID_VEC[0].as_str())
            .mail_folder(ID_VEC[1].as_str())
            .child_folder(ID_VEC[2].as_str())
            .messages_id(ID_VEC[3].as_str())
            .delete_extensions(ID_VEC[0].as_str())
            .url()
            .path()
    );
}
