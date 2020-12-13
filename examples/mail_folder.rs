use graph_rs_sdk::prelude::*;

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";

static MESSAGE_ID: &str = "MESSAGE_ID";

static MAIL_FOLDER_ID: &str = "MAIL_FOLDER_ID_OR_WELL_KNOWN_FOLDER";

fn main() {
    create_mail_folder_message();
    create_mail_folder_draft_message();
    delete_mail_folder_message();
    add_mail_folder_message_attachment();
}

fn create_mail_folder_message() {
    let client = Graph::new(ACCESS_TOKEN);
    let response = client
        .v1()
        .me()
        .mail_folder(MAIL_FOLDER_ID)
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

fn create_mail_folder_draft_message() {
    let client = Graph::new(ACCESS_TOKEN);
    let response = client
        .v1()
        .me()
        .mail_folder("drafts")
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

pub fn delete_mail_folder_message() {
    let client = Graph::new(ACCESS_TOKEN);
    let response = client
        .v1()
        .me()
        .mail_folder(MAIL_FOLDER_ID)
        .message(MESSAGE_ID)
        .delete_messages()
        .send();

    println!("{:#?}", response);
}

fn add_mail_folder_message_attachment() {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client
        .v1()
        .me()
        .mail_folder(MAIL_FOLDER_ID)
        .message(MESSAGE_ID)
        .attachments()
        .create_attachments(&serde_json::json!({
            "@odata.type": "#microsoft.graph.fileAttachment",
            "name": "smile",
            "contentBytes": "R0lGODdhEAYEAA7"
        }))
        .send();

    println!("{:#?}", response);
}
