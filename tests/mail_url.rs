use graph_rs::prelude::*;
use test_tools::drive::assert_url_eq;

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
    let _ = client.v1().me().mail().mail_folder().list("32p99453");
    assert_url_eq(&client, "/me/mailFolders/32p99453/messages");

    let _ = client
        .v1()
        .drives("32p99453")
        .mail()
        .mail_folder()
        .list("1234");
    client.format_ord();
    assert_url_eq(&client, "/drives/32p99453/mailFolders/1234/messages");

    let _ = client
        .v1()
        .sites("32p99453")
        .mail()
        .mail_folder()
        .list("1234");
    client.format_ord();
    assert_url_eq(&client, "/sites/32p99453/mailFolders/1234/messages");
}

#[test]
pub fn get_messages() {
    let client = Graph::new("");
    let _ = client.v1().me().mail().messages().get().by_id("1234");
    assert_url_eq(&client, "/me/messages/1234");

    let _ = client
        .v1()
        .drives("32p99453")
        .mail()
        .messages()
        .get()
        .by_id("1234");
    assert_url_eq(&client, "/drives/32p99453/messages/1234");

    let _ = client
        .v1()
        .sites("32p99453")
        .mail()
        .messages()
        .get()
        .by_id("1234");
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
        .get("32p99453")
        .by_id("1234");
    assert_url_eq(&client, "/me/mailFolders/32p99453/messages/1234");

    let _ = client
        .v1()
        .drives("32p99453")
        .mail()
        .mail_folder()
        .get("99453")
        .by_id("1234");
    assert_url_eq(&client, "/drives/32p99453/mailFolders/99453/messages/1234");

    let _ = client
        .v1()
        .sites("32p99453")
        .mail()
        .mail_folder()
        .get("99453")
        .by_id("1234");
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
        .update(&String::new())
        .by_id("1234");
    assert_url_eq(&client, "/me/messages/1234");

    let _ = client
        .v1()
        .sites("32p99453")
        .mail()
        .messages()
        .update(&String::new())
        .by_id("1234");
    assert_url_eq(&client, "/sites/32p99453/messages/1234");
}

#[test]
pub fn update_mail_folder_messages() {
    let client = Graph::new("");
    let _ = client
        .v1()
        .me()
        .mail()
        .mail_folder()
        .update("99453", &String::new())
        .by_id("1234");
    assert_url_eq(&client, "/me/mailFolders/99453/messages/1234");

    let _ = client
        .v1()
        .sites("32p99453")
        .mail()
        .mail_folder()
        .update("99453", &String::new())
        .by_id("1234");
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
pub fn create_mail_folder_messages() {
    let client = Graph::new("");
    let _ = client
        .v1()
        .me()
        .mail()
        .mail_folder()
        .create("99453", &String::new());
    assert_url_eq(&client, "/me/mailFolders/99453/messages");

    let _ = client
        .v1()
        .sites("32p99453")
        .mail()
        .mail_folder()
        .create("99453", &String::new())
        .by_id("1234");
    assert_url_eq(&client, "/sites/32p99453/mailFolders/99453/messages/1234");
}

#[test]
pub fn delete_messages() {
    let client = Graph::new("");
    let _ = client.v1().me().mail().messages().delete().by_id("1234");
    assert_url_eq(&client, "/me/messages/1234");

    let _ = client
        .v1()
        .sites("32p99453")
        .mail()
        .messages()
        .delete()
        .by_id("1234");
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
        .delete("32p99453")
        .by_id("1234");
    assert_url_eq(&client, "/me/mailFolders/32p99453/messages/1234");

    let _ = client
        .v1()
        .sites("32p99453")
        .mail()
        .mail_folder()
        .delete("99453")
        .by_id("1234");
    assert_url_eq(&client, "/sites/32p99453/mailFolders/99453/messages/1234");
}

#[test]
pub fn copy_messages() {
    let client = Graph::new("");
    let _ = client.v1().me().mail().messages().copy("").by_id("1234");
    assert_url_eq(&client, "/me/messages/1234/copy");

    let _ = client
        .v1()
        .sites("32p99453")
        .mail()
        .messages()
        .copy("")
        .by_id("1234");
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
        .copy("99453", "")
        .by_id("1234");
    assert_url_eq(&client, "/me/mailFolders/99453/messages/1234/copy");

    let _ = client
        .v1()
        .sites("32p99453")
        .mail()
        .mail_folder()
        .copy("99453", "")
        .by_id("1234");
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
        .move_message("")
        .by_id("1234");
    assert_url_eq(&client, "/me/messages/1234/move");

    let _ = client
        .v1()
        .sites("32p99453")
        .mail()
        .messages()
        .move_message("")
        .by_id("1234");
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
        .move_message("99453", "")
        .by_id("1234");
    assert_url_eq(&client, "/me/mailFolders/99453/messages/1234/move");

    let _ = client
        .v1()
        .sites("32p99453")
        .mail()
        .mail_folder()
        .move_message("99453", "")
        .by_id("1234");
    assert_url_eq(
        &client,
        "/sites/32p99453/mailFolders/99453/messages/1234/move",
    );
}

#[test]
pub fn create_reply_messages() {
    let client = Graph::new("");
    let _ = client
        .v1()
        .me()
        .mail()
        .messages()
        .create_reply()
        .by_id("1234");
    assert_url_eq(&client, "/me/messages/1234/createReply");

    let _ = client
        .v1()
        .sites("32p99453")
        .mail()
        .messages()
        .create_reply()
        .by_id("1234");
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
        .create_reply("99453")
        .by_id("1234");
    assert_url_eq(&client, "/me/mailFolders/99453/messages/1234/createReply");

    let _ = client
        .v1()
        .sites("32p99453")
        .mail()
        .mail_folder()
        .create_reply("99453")
        .by_id("1234");
    assert_url_eq(
        &client,
        "/sites/32p99453/mailFolders/99453/messages/1234/createReply",
    );
}

#[test]
pub fn create_reply_all_messages() {
    let client = Graph::new("");
    let _ = client
        .v1()
        .me()
        .mail()
        .messages()
        .create_reply_all()
        .by_id("1234");
    assert_url_eq(&client, "/me/messages/1234/createReplyAll");

    let _ = client
        .v1()
        .sites("32p99453")
        .mail()
        .messages()
        .create_reply_all()
        .by_id("1234");
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
        .create_reply_all("99453")
        .by_id("1234");
    assert_url_eq(
        &client,
        "/me/mailFolders/99453/messages/1234/createReplyAll",
    );

    let _ = client
        .v1()
        .sites("32p99453")
        .mail()
        .mail_folder()
        .create_reply_all("99453")
        .by_id("1234");
    assert_url_eq(
        &client,
        "/sites/32p99453/mailFolders/99453/messages/1234/createReplyAll",
    );
}

#[test]
pub fn reply_messages() {
    let client = Graph::new("");
    let _ = client.v1().me().mail().messages().reply("").by_id("1234");
    assert_url_eq(&client, "/me/messages/1234/reply");

    let _ = client
        .v1()
        .sites("32p99453")
        .mail()
        .messages()
        .reply("")
        .by_id("1234");
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
        .reply("99453", "")
        .by_id("1234");
    assert_url_eq(&client, "/me/mailFolders/99453/messages/1234/reply");

    let _ = client
        .v1()
        .sites("32p99453")
        .mail()
        .mail_folder()
        .reply("99453", "")
        .by_id("1234");
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
        .reply_all("")
        .by_id("1234");
    assert_url_eq(&client, "/me/messages/1234/replyAll");

    let _ = client
        .v1()
        .sites("32p99453")
        .mail()
        .messages()
        .reply_all("")
        .by_id("1234");
    assert_url_eq(&client, "/sites/32p99453/messages/1234/replyAll");
}

#[test]
pub fn reply_all_mail_folder_messages() {
    let client = Graph::new("");
    let _ = client
        .v1()
        .me()
        .mail()
        .mail_folder()
        .reply_all("99453", "")
        .by_id("1234");
    assert_url_eq(&client, "/me/mailFolders/99453/messages/1234/replyAll");

    let _ = client
        .v1()
        .sites("32p99453")
        .mail()
        .mail_folder()
        .reply_all("99453", "")
        .by_id("1234");
    assert_url_eq(
        &client,
        "/sites/32p99453/mailFolders/99453/messages/1234/replyAll",
    );
}

#[test]
pub fn create_forward_messages() {
    let client = Graph::new("");
    let _ = client
        .v1()
        .me()
        .mail()
        .messages()
        .create_forward()
        .by_id("1234");
    assert_url_eq(&client, "/me/messages/1234/createForward");

    let _ = client
        .v1()
        .sites("32p99453")
        .mail()
        .messages()
        .create_forward()
        .by_id("1234");
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
        .create_forward("99453")
        .by_id("1234");
    assert_url_eq(&client, "/me/mailFolders/99453/messages/1234/createForward");

    let _ = client
        .v1()
        .sites("32p99453")
        .mail()
        .mail_folder()
        .create_forward("99453")
        .by_id("1234");
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
        .forward::<()>("", &vec![])
        .by_id("1234");
    assert_url_eq(&client, "/me/messages/1234/forward");

    let _ = client
        .v1()
        .sites("32p99453")
        .mail()
        .messages()
        .forward::<()>("", &vec![])
        .by_id("1234");
    assert_url_eq(&client, "/sites/32p99453/messages/1234/forward");
}

#[test]
pub fn forward_mail_folder_messages() {
    let client = Graph::new("");
    let _ = client
        .v1()
        .me()
        .mail()
        .mail_folder()
        .forward::<()>("99453", "", &vec![])
        .by_id("1234");
    assert_url_eq(&client, "/me/mailFolders/99453/messages/1234/forward");

    let _ = client
        .v1()
        .sites("32p99453")
        .mail()
        .mail_folder()
        .forward::<()>("99453", "", &vec![])
        .by_id("1234");
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
        .send_mail("99453", &String::new());
    assert_url_eq(&client, "/me/mailFolders/99453/sendMail");

    let _ = client
        .v1()
        .sites("32p99453")
        .mail()
        .mail_folder()
        .send_mail("99453", &String::new());
    assert_url_eq(&client, "/sites/32p99453/mailFolders/99453/sendMail");
}

#[test]
pub fn send_messages() {
    let client = Graph::new("");
    let _ = client
        .v1()
        .me()
        .mail()
        .messages()
        .send_message()
        .by_id("1234");
    assert_url_eq(&client, "/me/messages/1234/send");

    let _ = client
        .v1()
        .sites("32p99453")
        .mail()
        .messages()
        .send_message()
        .by_id("1234");
    assert_url_eq(&client, "/sites/32p99453/messages/1234/send");
}

#[test]
pub fn send_mail_folder_messages() {
    let client = Graph::new("");
    let _ = client
        .v1()
        .me()
        .mail()
        .mail_folder()
        .send_message("99453")
        .by_id("1234");
    assert_url_eq(&client, "/me/mailFolders/99453/messages/1234/send");

    let _ = client
        .v1()
        .sites("32p99453")
        .mail()
        .mail_folder()
        .send_message("99453")
        .by_id("1234");
    assert_url_eq(
        &client,
        "/sites/32p99453/mailFolders/99453/messages/1234/send",
    );
}
