use graph_rs::prelude::*;
use test_tools::assert_url_eq;

#[test]
pub fn list_messages() {
    let client = Graph::new("");
    let _ = client.v1().me().mail().messages().list();
    assert_url_eq(&client, "/me/messages");

    let _ = client.v1().drives("32p99453").mail().messages().list();
    assert_url_eq(&client, "/drives/32p99453/messages");

    let _ = client.v1().sites("32p99453").mail().messages().list();
    assert_url_eq(&client, "/sites/32p99453/messages");
}

#[test]
pub fn list_mail_folder_messages() {
    let client = Graph::new("");
    let _ = client
        .v1()
        .me()
        .mail()
        .mail_folder()
        .messages()
        .list("32p99453");
    assert_url_eq(&client, "/me/mailFolders/32p99453/messages");

    let _ = client
        .v1()
        .drives("32p99453")
        .mail()
        .mail_folder()
        .messages()
        .list("1234");
    assert_url_eq(&client, "/drives/32p99453/mailFolders/1234/messages");

    let _ = client
        .v1()
        .sites("32p99453")
        .mail()
        .mail_folder()
        .messages()
        .list("1234");
    assert_url_eq(&client, "/sites/32p99453/mailFolders/1234/messages");
}

#[test]
pub fn get_messages() {
    let client = Graph::new("");
    let _ = client.v1().me().mail().messages().get("1234");
    assert_url_eq(&client, "/me/messages/1234");

    let _ = client.v1().sites("32p99453").mail().messages().get("1234");
    assert_url_eq(&client, "/sites/32p99453/messages/1234");
}

#[test]
pub fn get_mail_folder_messages() {
    let client = Graph::new("");
    let _ = client
        .v1()
        .me()
        .mail()
        .mail_folder()
        .messages()
        .get("99453", "1234");
    assert_url_eq(&client, "/me/mailFolders/99453/messages/1234");

    let _ = client
        .v1()
        .drives("32p99453")
        .mail()
        .mail_folder()
        .messages()
        .get("99453", "1234");
    assert_url_eq(&client, "/drives/32p99453/mailFolders/99453/messages/1234");

    let _ = client
        .v1()
        .sites("32p99453")
        .mail()
        .mail_folder()
        .messages()
        .get("99453", "1234");
    assert_url_eq(&client, "/sites/32p99453/mailFolders/99453/messages/1234");
}

#[test]
pub fn update_messages() {
    let client = Graph::new("");
    let _ = client
        .v1()
        .me()
        .mail()
        .messages()
        .update("1234", &String::new());
    assert_url_eq(&client, "/me/messages/1234");

    let _ = client
        .v1()
        .sites("32p99453")
        .mail()
        .messages()
        .update("1234", &String::new());
    assert_url_eq(&client, "/sites/32p99453/messages/1234");
}

#[test]
pub fn update_mail_folder_messages() {
    let client = Graph::new("");
    let _ =
        client
            .v1()
            .me()
            .mail()
            .mail_folder()
            .messages()
            .update("99453", "1234", &String::new());
    assert_url_eq(&client, "/me/mailFolders/99453/messages/1234");

    let _ = client
        .v1()
        .sites("32p99453")
        .mail()
        .mail_folder()
        .messages()
        .update("99453", "1234", &String::new());
    assert_url_eq(&client, "/sites/32p99453/mailFolders/99453/messages/1234");
}

#[test]
pub fn create_messages() {
    let client = Graph::new("");
    let _ = client.v1().me().mail().messages().create(&String::new());
    assert_url_eq(&client, "/me/messages");

    let _ = client
        .v1()
        .sites("32p99453")
        .mail()
        .messages()
        .create(&String::new());
    assert_url_eq(&client, "/sites/32p99453/messages");
}

#[test]
pub fn create_mail_folder() {
    let client = Graph::new("");
    let _ = client.v1().me().mail().mail_folder().create(&String::new());
    assert_url_eq(&client, "/me/mailFolders");

    let _ = client
        .v1()
        .sites("32p99453")
        .mail()
        .mail_folder()
        .create(&String::new());
    assert_url_eq(&client, "/sites/32p99453/mailFolders");
}

#[test]
pub fn delete_messages() {
    let client = Graph::new("");
    let _ = client.v1().me().mail().messages().delete("1234");
    assert_url_eq(&client, "/me/messages/1234");

    let _ = client
        .v1()
        .sites("32p99453")
        .mail()
        .messages()
        .delete("1234");
    assert_url_eq(&client, "/sites/32p99453/messages/1234");
}

#[test]
pub fn delete_mail_folder_messages() {
    let client = Graph::new("");
    let _ = client
        .v1()
        .me()
        .mail()
        .mail_folder()
        .messages()
        .delete("99453", "1234");
    assert_url_eq(&client, "/me/mailFolders/99453/messages/1234");

    let _ = client
        .v1()
        .sites("32p99453")
        .mail()
        .mail_folder()
        .messages()
        .delete("99453", "1234");
    assert_url_eq(&client, "/sites/32p99453/mailFolders/99453/messages/1234");
}

#[test]
pub fn copy_messages() {
    let client = Graph::new("");
    let _ = client
        .v1()
        .me()
        .mail()
        .messages()
        .copy("1234", &String::new());
    assert_url_eq(&client, "/me/messages/1234/copy");

    let _ = client
        .v1()
        .sites("32p99453")
        .mail()
        .messages()
        .copy("1234", &String::new());
    assert_url_eq(&client, "/sites/32p99453/messages/1234/copy");
}

#[test]
pub fn copy_mail_folder_messages() {
    let client = Graph::new("");
    let _ = client
        .v1()
        .me()
        .mail()
        .mail_folder()
        .messages()
        .copy("99453", "1234", &String::new());
    assert_url_eq(&client, "/me/mailFolders/99453/messages/1234/copy");

    let _ = client
        .v1()
        .sites("32p99453")
        .mail()
        .mail_folder()
        .messages()
        .copy("99453", "1234", &String::new());
    assert_url_eq(
        &client,
        "/sites/32p99453/mailFolders/99453/messages/1234/copy",
    );
}

#[test]
pub fn move_messages() {
    let client = Graph::new("");
    let _ = client
        .v1()
        .me()
        .mail()
        .messages()
        .move_message("1234", &String::new());
    assert_url_eq(&client, "/me/messages/1234/move");

    let _ = client
        .v1()
        .sites("32p99453")
        .mail()
        .messages()
        .move_message("1234", &String::new());
    assert_url_eq(&client, "/sites/32p99453/messages/1234/move");
}

#[test]
pub fn move_mail_folder_messages() {
    let client = Graph::new("");
    let _ = client
        .v1()
        .me()
        .mail()
        .mail_folder()
        .messages()
        .move_message("99453", "1234", &String::new());
    assert_url_eq(&client, "/me/mailFolders/99453/messages/1234/move");

    let _ = client
        .v1()
        .sites("32p99453")
        .mail()
        .mail_folder()
        .messages()
        .move_message("99453", "1234", &String::new());
    assert_url_eq(
        &client,
        "/sites/32p99453/mailFolders/99453/messages/1234/move",
    );
}

#[test]
pub fn create_reply_messages() {
    let client = Graph::new("");
    let _ = client.v1().me().mail().messages().create_reply("1234");
    assert_url_eq(&client, "/me/messages/1234/createReply");

    let _ = client
        .v1()
        .sites("32p99453")
        .mail()
        .messages()
        .create_reply("1234");
    assert_url_eq(&client, "/sites/32p99453/messages/1234/createReply");
}

#[test]
pub fn create_reply_mail_folder_messages() {
    let client = Graph::new("");
    let _ = client
        .v1()
        .me()
        .mail()
        .mail_folder()
        .messages()
        .create_reply("99453", "1234");
    assert_url_eq(&client, "/me/mailFolders/99453/messages/1234/createReply");

    let _ = client
        .v1()
        .sites("32p99453")
        .mail()
        .mail_folder()
        .messages()
        .create_reply("99453", "1234");
    assert_url_eq(
        &client,
        "/sites/32p99453/mailFolders/99453/messages/1234/createReply",
    );
}

#[test]
pub fn create_reply_all_messages() {
    let client = Graph::new("");
    let _ = client.v1().me().mail().messages().create_reply_all("1234");
    assert_url_eq(&client, "/me/messages/1234/createReplyAll");

    let _ = client
        .v1()
        .sites("32p99453")
        .mail()
        .messages()
        .create_reply_all("1234");
    assert_url_eq(&client, "/sites/32p99453/messages/1234/createReplyAll");
}

#[test]
pub fn create_reply_all_mail_folder_messages() {
    let client = Graph::new("");
    let _ = client
        .v1()
        .me()
        .mail()
        .mail_folder()
        .messages()
        .create_reply_all("99453", "1234");
    assert_url_eq(
        &client,
        "/me/mailFolders/99453/messages/1234/createReplyAll",
    );

    let _ = client
        .v1()
        .sites("32p99453")
        .mail()
        .mail_folder()
        .messages()
        .create_reply_all("99453", "1234");
    assert_url_eq(
        &client,
        "/sites/32p99453/mailFolders/99453/messages/1234/createReplyAll",
    );
}

#[test]
pub fn reply_messages() {
    let client = Graph::new("");
    let _ = client
        .v1()
        .me()
        .mail()
        .messages()
        .reply("1234", &String::new());
    assert_url_eq(&client, "/me/messages/1234/reply");

    let _ = client
        .v1()
        .sites("32p99453")
        .mail()
        .messages()
        .reply("1234", &String::new());
    assert_url_eq(&client, "/sites/32p99453/messages/1234/reply");
}

#[test]
pub fn reply_mail_folder_messages() {
    let client = Graph::new("");
    let _ = client
        .v1()
        .me()
        .mail()
        .mail_folder()
        .messages()
        .reply("99453", "1234", &String::new());
    assert_url_eq(&client, "/me/mailFolders/99453/messages/1234/reply");

    let _ = client
        .v1()
        .sites("32p99453")
        .mail()
        .mail_folder()
        .messages()
        .reply("99453", "1234", &String::new());
    assert_url_eq(
        &client,
        "/sites/32p99453/mailFolders/99453/messages/1234/reply",
    );
}

#[test]
pub fn reply_all_messages() {
    let client = Graph::new("");
    let _ = client
        .v1()
        .me()
        .mail()
        .messages()
        .reply_all("1234", &String::new());
    assert_url_eq(&client, "/me/messages/1234/replyAll");

    let _ = client
        .v1()
        .sites("32p99453")
        .mail()
        .messages()
        .reply_all("1234", &String::new());
    assert_url_eq(&client, "/sites/32p99453/messages/1234/replyAll");
}

#[test]
pub fn reply_all_mail_folder_messages() {
    let client = Graph::new("");
    let _ =
        client
            .v1()
            .me()
            .mail()
            .mail_folder()
            .messages()
            .reply_all("99453", "1234", &String::new());
    assert_url_eq(&client, "/me/mailFolders/99453/messages/1234/replyAll");

    let _ = client
        .v1()
        .sites("32p99453")
        .mail()
        .mail_folder()
        .messages()
        .reply_all("99453", "1234", &String::new());
    assert_url_eq(
        &client,
        "/sites/32p99453/mailFolders/99453/messages/1234/replyAll",
    );
}

#[test]
pub fn create_forward_messages() {
    let client = Graph::new("");
    let _ = client.v1().me().mail().messages().create_forward("1234");
    assert_url_eq(&client, "/me/messages/1234/createForward");

    let _ = client
        .v1()
        .sites("32p99453")
        .mail()
        .messages()
        .create_forward("1234");
    assert_url_eq(&client, "/sites/32p99453/messages/1234/createForward");
}

#[test]
pub fn create_forward_mail_folder_messages() {
    let client = Graph::new("");
    let _ = client
        .v1()
        .me()
        .mail()
        .mail_folder()
        .messages()
        .create_forward("99453", "1234");
    assert_url_eq(&client, "/me/mailFolders/99453/messages/1234/createForward");

    let _ = client
        .v1()
        .sites("32p99453")
        .mail()
        .mail_folder()
        .messages()
        .create_forward("99453", "1234");
    assert_url_eq(
        &client,
        "/sites/32p99453/mailFolders/99453/messages/1234/createForward",
    );
}

#[test]
pub fn forward_messages() {
    let client = Graph::new("");
    let _ = client
        .v1()
        .me()
        .mail()
        .messages()
        .forward("1234", &String::new());
    assert_url_eq(&client, "/me/messages/1234/forward");

    let _ = client
        .v1()
        .sites("32p99453")
        .mail()
        .messages()
        .forward("1234", &String::new());
    assert_url_eq(&client, "/sites/32p99453/messages/1234/forward");
}

#[test]
pub fn forward_mail_folder_messages() {
    let client = Graph::new("");
    let _ =
        client
            .v1()
            .me()
            .mail()
            .mail_folder()
            .messages()
            .forward("99453", "1234", &String::new());
    assert_url_eq(&client, "/me/mailFolders/99453/messages/1234/forward");

    let _ = client
        .v1()
        .sites("32p99453")
        .mail()
        .mail_folder()
        .messages()
        .forward("99453", "1234", &String::new());
    assert_url_eq(
        &client,
        "/sites/32p99453/mailFolders/99453/messages/1234/forward",
    );
}

#[test]
pub fn send_mail_messages() {
    let client = Graph::new("");
    let _ = client.v1().me().mail().messages().send_mail(&String::new());
    assert_url_eq(&client, "/me/sendMail");

    let _ = client
        .v1()
        .sites("32p99453")
        .mail()
        .messages()
        .send_mail(&String::new());
    assert_url_eq(&client, "/sites/32p99453/sendMail");
}

#[test]
pub fn send_mail_mail_folder_messages() {
    let client = Graph::new("");
    let _ = client
        .v1()
        .me()
        .mail()
        .mail_folder()
        .messages()
        .send_mail(&String::new());
    assert_url_eq(&client, "/me/sendMail");

    let _ = client
        .v1()
        .sites("32p99453")
        .mail()
        .mail_folder()
        .messages()
        .send_mail(&String::new());
    assert_url_eq(&client, "/sites/32p99453/sendMail");
}

#[test]
pub fn send_messages() {
    let client = Graph::new("");
    let _ = client.v1().me().mail().messages().send_message("1234");
    assert_url_eq(&client, "/me/messages/1234/send");

    let _ = client
        .v1()
        .sites("32p99453")
        .mail()
        .messages()
        .send_message("1234");
    assert_url_eq(&client, "/sites/32p99453/messages/1234/send");
}

#[test]
fn mail_outlook_category() {
    let client = Graph::new("");

    let _ = client.v1().me().mail().outlook_category().list();
    assert_url_eq(&client, "/me/outlook/masterCategories");

    let _ = client
        .v1()
        .sites("32p99453")
        .mail()
        .outlook_category()
        .list();
    assert_url_eq(&client, "/sites/32p99453/outlook/masterCategories");

    let _ = client
        .v1()
        .sites("32p99453")
        .mail()
        .outlook_category()
        .get("1234");
    assert_url_eq(&client, "/sites/32p99453/outlook/masterCategories/1234");

    let _ = client
        .v1()
        .sites("32p99453")
        .mail()
        .outlook_category()
        .create(&serde_json::json!({}));
    assert_url_eq(&client, "/sites/32p99453/outlook/masterCategories");

    let _ = client
        .v1()
        .sites("32p99453")
        .mail()
        .outlook_category()
        .update("1234", &serde_json::json!({}));
    assert_url_eq(&client, "/sites/32p99453/outlook/masterCategories/1234");

    let _ = client
        .v1()
        .sites("32p99453")
        .mail()
        .outlook_category()
        .delete("1234");
    assert_url_eq(&client, "/sites/32p99453/outlook/masterCategories/1234");
}

#[test]
fn mail_folder_rules() {
    let client = Graph::new("");

    let _ = client.v1().me().mail().mail_folder().rules().list();
    assert_url_eq(&client, "/me/mailFolders/inbox/messageRules");

    let _ = client
        .v1()
        .sites("32p99453")
        .mail()
        .mail_folder()
        .rules()
        .list();
    assert_url_eq(&client, "/sites/32p99453/mailFolders/inbox/messageRules");

    let _ = client
        .v1()
        .sites("32p99453")
        .mail()
        .mail_folder()
        .rules()
        .get("1234");
    assert_url_eq(
        &client,
        "/sites/32p99453/mailFolders/inbox/messageRules/1234",
    );

    let _ = client
        .v1()
        .sites("32p99453")
        .mail()
        .mail_folder()
        .rules()
        .create(&serde_json::json!({}));
    assert_url_eq(&client, "/sites/32p99453/mailFolders/inbox/messageRules");

    let _ = client
        .v1()
        .sites("32p99453")
        .mail()
        .mail_folder()
        .rules()
        .update("1234", &serde_json::json!({}));
    assert_url_eq(
        &client,
        "/sites/32p99453/mailFolders/inbox/messageRules/1234",
    );

    let _ = client
        .v1()
        .sites("32p99453")
        .mail()
        .mail_folder()
        .rules()
        .delete("1234");
    assert_url_eq(
        &client,
        "/sites/32p99453/mailFolders/inbox/messageRules/1234",
    );
}

#[test]
fn mail_focused_inbox() {
    let client = Graph::new("");

    let _ = client.v1().me().mail().focused_inbox().list_overrides();
    assert_url_eq(&client, "/me/inferenceClassification/overrides");

    let _ = client
        .v1()
        .sites("32p99453")
        .mail()
        .focused_inbox()
        .list_overrides();
    assert_url_eq(&client, "/sites/32p99453/inferenceClassification/overrides");

    let _ = client
        .v1()
        .sites("32p99453")
        .mail()
        .focused_inbox()
        .create_override(&serde_json::json!({}));
    assert_url_eq(&client, "/sites/32p99453/inferenceClassification/overrides");

    let _ = client
        .v1()
        .sites("32p99453")
        .mail()
        .focused_inbox()
        .update_override("1234", &serde_json::json!({}));
    assert_url_eq(
        &client,
        "/sites/32p99453/inferenceClassification/overrides/1234",
    );

    let _ = client
        .v1()
        .sites("32p99453")
        .mail()
        .focused_inbox()
        .delete_override("1234");
    assert_url_eq(
        &client,
        "/sites/32p99453/inferenceClassification/overrides/1234",
    );
}
