use graph_rs_sdk::prelude::*;

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";

static MESSAGE_ID: &str = "MESSAGE_ID";

static MAIL_FOLDER_ID: &str = "MAIL_FOLDER_ID_OR_WELL_KNOWN_FOLDER";

static ATTACHMENT_ID: &str = "ATTACHMENT_ID";

static CHILD_FOLDER_ID_1: &str = "CHILD_FOLDER_ID1";

static CHILD_FOLDER_ID_2: &str = "CHILD_FOLDER_ID1";

pub async fn get_child_folders_attachment() {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client
        .me()
        .mail_folder(MAIL_FOLDER_ID)
        .child_folder(CHILD_FOLDER_ID_1)
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

// You can keep calling the child_folder("id") method
// until you get to the child folder you want.
pub async fn get_child_folders_attachment_content() {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client
        .me()
        .mail_folder(MAIL_FOLDER_ID)
        .child_folder(CHILD_FOLDER_ID_1)
        .child_folder(CHILD_FOLDER_ID_2)
        .message(MESSAGE_ID)
        .attachment(ATTACHMENT_ID)
        .get_attachments_content()
        .send()
        .await
        .unwrap();

    println!("{response:#?}");

    let text = response.text().await.unwrap();
    println!("{text:#?}");
}
