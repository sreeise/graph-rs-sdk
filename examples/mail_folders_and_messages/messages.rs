use graph_rs_sdk::*;

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";

static MESSAGE_ID: &str = "MESSAGE_ID";

static ATTACHMENT_ID: &str = "ATTACHMENT_ID";

// If using the Users api:
static USER_ID: &str = "USER_ID";

pub async fn list_messages() {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client.me().messages().list_messages().send().await.unwrap();
    println!("{response:#?}");
}

pub async fn user_list_messages() {
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

pub async fn delete_message() {
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

pub async fn create_message() {
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

pub async fn update_message() {
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

pub async fn send_mail() -> GraphResult<()> {
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
        .await?;

    println!("{response:#?}");

    Ok(())
}
