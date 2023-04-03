use graph_rs_sdk::*;

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";

static MESSAGE_ID: &str = "MESSAGE_ID";

static ATTACHMENT_ID: &str = "ATTACHMENT_ID";

// If using the Users api:
static USER_ID: &str = "USER_ID";

pub async fn add_attachment() {
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

pub async fn get_attachment() {
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

    let body: serde_json::Value = response.json().await.unwrap();
    println!("{body:#?}");
}

pub async fn get_attachment_content() {
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

    let body: serde_json::Value = response.json().await.unwrap();
    println!("{body:#?}");
}

static MAIL_FOLDER_ID: &str = "MAIL_FOLDER_ID";

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
