use graph_rs_sdk::prelude::*;

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";

static MESSAGE_ID: &str = "MESSAGE_ID";

static ATTACHMENT_ID: &str = "ATTACHMENT_ID";

// If using the Users api:
static USER_ID: &str = "USER_ID";

#[tokio::main]
async fn main() {
    list_messages().await;
    create_message().await;
    update_message().await;
    delete_message().await;
    add_attachment().await;
    get_attachment().await;
    get_attachment_content().await;
    send_mail().await;
}

async fn list_messages() {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client.me().messages().list_messages().send().await.unwrap();
    println!("{response:#?}");
}

async fn user_list_messages() {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client
        .user(USER_ID)
        .messages()
        .list_messages()
        .send()
        .await
        .unwrap();

    println!("{response:#?}");
}

async fn delete_message() {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client
        .me()
        .message(MESSAGE_ID)
        .delete_messages()
        .send()
        .await
        .unwrap();

    println!("{response:#?}");
}

async fn create_message() {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client
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
        .send()
        .await
        .unwrap();

    println!("{response:#?}");
}

async fn update_message() {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client
        .me()
        .message(MESSAGE_ID)
        .update_messages(&serde_json::json!({
            "subject": "subject-value",
                "body": {
                "contentType": "",
                "content": "content-value"
            },
            "inferenceClassification": "other"
        }))
        .send()
        .await
        .unwrap();

    println!("{response:#?}");
}

async fn send_mail() {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client
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
        .send()
        .await
        .unwrap();

    println!("{response:#?}");
}

async fn add_attachment() {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client
        .me()
        .message(MESSAGE_ID)
        .attachments()
        .create_attachments(&serde_json::json!({
            "@odata.type": "#microsoft.graph.fileAttachment",
            "name": "smile",
            "contentBytes": "R0lGODdhEAYEAA7"
        }))
        .send()
        .await
        .unwrap();

    println!("{response:#?}");
}

async fn get_attachment() {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client
        .me()
        .message(MESSAGE_ID)
        .attachment(ATTACHMENT_ID)
        .get_attachments()
        .send()
        .await
        .unwrap();

    println!("{response:#?}");
}

async fn get_attachment_content() {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client
        .me()
        .message(MESSAGE_ID)
        .attachment(ATTACHMENT_ID)
        .get_attachments_content()
        .send()
        .await
        .unwrap();

    println!("{response:#?}");
}
