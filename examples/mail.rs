use graph_rs::prelude::*;

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";

static MESSAGE_ID: &str = "MESSAGE_ID";

static MAIL_FOLDER_ID: &str = "MAIL_FOLDER_ID_OR_WELL_KNOWN_FOLDER";

static ATTACHMENT_ID: &str = "ATTACHMENT_ID";

fn main() {
    list_messages();
    create_message();
    update_message();
    delete_message();
    add_attachment();
    get_attachment();
    get_attachment_content();
    create_mail_folder_message();
    create_mail_folder_draft_message();
    delete_mail_folder_message();
    add_mail_folder_message_attachment();
    get_child_folders_attachment(vec!["1ST_CHILD_FOLDER_ID", "2ND_CHILD_FOLDER_ID"]);
    send_mail();
}

fn list_messages() {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client.v1().me().messages().list_messages().send();
    println!("{:#?}", response);
}

fn delete_message() {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client
        .v1()
        .me()
        .message(MESSAGE_ID)
        .delete_messages()
        .send();

    println!("{:#?}", response);
}

fn create_message() {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client
        .v1()
        .me()
        .messages()
        .create_messages(&serde_json::json!({
            "subject":"Did you see last night's game?",
            "importance":"Low",
            "body":{
                "contentType":"HTML",
                "content":"They were <b>awesome</b>!"
            },
            "toRecipients":[
                {
                    "emailAddress":{
                        "address":"miriamg@sreeise.onmicrosoft.com"
                    }
                }
            ]
        }))
        .send();

    println!("{:#?}", response);
}

pub fn send_mail() {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client
        .v1()
        .me()
        .send_mail(&serde_json::json!({
                "message": {
                "subject": "Meet for lunch?",
                "body": {
                    "contentType": "Text",
                    "content": "The new cafeteria is open."
                },
                "toRecipients": [
                    {
                        "emailAddress": {
                        "address": "fannyd@contoso.onmicrosoft.com"
                        }
                    }
                ],
                "ccRecipients": [
                    {
                        "emailAddress": {
                        "address": "danas@contoso.onmicrosoft.com"
                        }
                    }
                ]
            },
            "saveToSentItems": "false"
        }))
        .send();

    println!("{:#?}", response);
}

/*
fn add_attachment() {
    let client = Graph::new(ACCESS_TOKEN);

    let add_attachment_response = client
        .v1()
        .me()
        .mail()
        .messages()
        .attachments()
        .add(
            MESSAGE_ID,
            &serde_json::json!({
                "@odata.type": "#microsoft.graph.fileAttachment",
                "name": "smile",
                "contentBytes": "R0lGODdhEAYEAA7"
            }),
        )
        .send();

    println!("{:#?}", add_attachment_response);
}

fn get_attachment() {
    let client = Graph::new(ACCESS_TOKEN);

    let get_attachment_response = client
        .v1()
        .me()
        .mail()
        .messages()
        .attachments()
        .get(MESSAGE_ID, ATTACHMENT_ID)
        .send();

    println!("{:#?}", get_attachment_response);
}

fn get_attachment_content() {
    let client = Graph::new(ACCESS_TOKEN);

    let attachment_content_response = client
        .v1()
        .me()
        .mail()
        .messages()
        .attachments()
        .content(MESSAGE_ID, ATTACHMENT_ID)
        .send();

    println!("{:#?}", attachment_content_response);
}

fn update_message() {
    let client = Graph::new(ACCESS_TOKEN);

    let update_message_response = client
        .v1()
        .me()
        .mail()
        .messages()
        .update(
            MESSAGE_ID,
            &serde_json::json!({
                "subject": "subject-value",
                    "body": {
                    "contentType": "",
                    "content": "content-value"
                },
                "inferenceClassification": "other"
            }),
        )
        .send();

    println!("{:#?}", update_message_response);
}

fn create_mail_folder_message() {
    let client = Graph::new(ACCESS_TOKEN);
    let message_response = client
        .v1()
        .me()
        .mail()
        .mail_folder()
        .messages()
        .create(
            MAIL_FOLDER_ID,
            &serde_json::json!({
                "subject":"Did you see last night's game?",
                "importance":"Low",
                "body":{
                    "contentType":"HTML",
                    "content":"They were <b>awesome</b>!"
                },
                "toRecipients":[
                    {
                            "emailAddress":{
                            "address":"miriamg@sreeise.onmicrosoft.com"
                        }
                    }
                ]
            }),
        )
        .send();

    println!("{:#?}", message_response);
}

fn create_mail_folder_draft_message() {
    let client = Graph::new(ACCESS_TOKEN);
    let draft_message_response = client
        .v1()
        .me()
        .mail()
        .mail_folder()
        .messages()
        .create(
            "drafts",
            &serde_json::json!({
                "subject":"Did you see last night's game?",
                "importance":"Low",
                "body":{
                    "contentType":"HTML",
                    "content":"They were <b>awesome</b>!"
                },
                "toRecipients":[
                    {
                            "emailAddress":{
                            "address":"miriamg@sreeise.onmicrosoft.com"
                        }
                    }
                ]
            }),
        )
        .send();

    println!("{:#?}", draft_message_response);
}

pub fn delete_mail_folder_message() {
    let client = Graph::new(ACCESS_TOKEN);
    let delete_message_response = client
        .v1()
        .me()
        .mail_folder(MAIL_FOLDER_ID)
        .message(MESSAGE_ID)
        .delete_messages()
        .send();

    println!("{:#?}", delete_message_response);
}

fn add_mail_folder_message_attachment() {
    let client = Graph::new(ACCESS_TOKEN);

    let add_attachment_response = client
        .v1()
        .me()
        .mail()
        .mail_folder()
        .attachments()
        .add(
            MAIL_FOLDER_ID,
            MESSAGE_ID,
            &serde_json::json!({
                "@odata.type": "#microsoft.graph.fileAttachment",
                "name": "smile",
                "contentBytes": "R0lGODdhEAYEAA7"
            }),
        )
        .send();

    println!("{:#?}", add_attachment_response);
}

 */
