use graph_rs_sdk::*;

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";

static MESSAGE_ID: &str = "MESSAGE_ID";

static MAIL_FOLDER_ID: &str = "MAIL_FOLDER_ID_OR_WELL_KNOWN_FOLDER";

static USER_ID: &str = "USER_ID";

// Get the top 2 inbox messages for a user.
pub async fn get_user_inbox_messages() -> GraphResult<()> {
    let client = Graph::new(ACCESS_TOKEN);
    let response = client
        .user(USER_ID)
        .mail_folder("Inbox")
        .messages()
        .list_messages()
        .top("2")
        .send()
        .await?;

    println!("{response:#?}");

    let body: serde_json::Value = response.json().await?;
    println!("{body:#?}");

    Ok(())
}

// Get the top 2 inbox messages for a user.
pub async fn get_me_inbox_messages() {
    let client = Graph::new(ACCESS_TOKEN);
    let response = client
        .me()
        .mail_folder("Inbox")
        .messages()
        .list_messages()
        .top("2")
        .send()
        .await
        .unwrap();

    println!("{response:#?}");

    let body: serde_json::Value = response.json().await.unwrap();
    println!("{body:#?}");
}

pub async fn create_mail_folder_message() -> GraphResult<()> {
    let client = Graph::new(ACCESS_TOKEN);
    let response = client
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
        .send()
        .await?;

    println!("{response:#?}");

    Ok(())
}

pub async fn create_mail_folder_draft_message() {
    let client = Graph::new(ACCESS_TOKEN);
    let response = client
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
        .send()
        .await
        .unwrap();

    println!("{response:#?}");
}

pub async fn delete_mail_folder_message() {
    let client = Graph::new(ACCESS_TOKEN);
    let response = client
        .me()
        .mail_folder(MAIL_FOLDER_ID)
        .messages_id(MESSAGE_ID)
        .delete_messages()
        .send()
        .await
        .unwrap();

    println!("{response:#?}");
}

pub async fn add_mail_folder_message_attachment() {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client
        .me()
        .mail_folder(MAIL_FOLDER_ID)
        .messages_id(MESSAGE_ID)
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
