use graph_rs::prelude::*;

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";

static MESSAGE_ID: &str = "MESSAGE_ID";

static MAIL_FOLDER_ID: &str = "MAIL_FOLDER_ID_OR_WELL_KNOWN_FOLDER";

static ATTACHMENT_ID: &str = "ATTACHMENT_ID";

fn main() {
    get_child_folders_attachment();
    get_child_folders_attachment_content();
}

// You can keep calling the child_folder("id") method
// until you get to the child folder you want.
fn get_child_folders_attachment() {
    let child_folder_id1 = "CHILD_FOLDER_ID1";
    let child_folder_id2 = "CHILD_FOLDER_ID2";

    let client = Graph::new(ACCESS_TOKEN);

    let response = client
        .v1()
        .me()
        .mail_folder(MAIL_FOLDER_ID)
        .child_folder(child_folder_id1)
        .child_folder(child_folder_id2)
        .message(MESSAGE_ID)
        .attachment(ATTACHMENT_ID)
        .get_attachments()
        .send();

    println!("{:#?}", response);
}

// You can keep calling the child_folder("id") method
// until you get to the child folder you want.
fn get_child_folders_attachment_content() {
    let child_folder_id1 = "CHILD_FOLDER_ID1";
    let child_folder_id2 = "CHILD_FOLDER_ID2";

    let client = Graph::new(ACCESS_TOKEN);

    let response = client
        .v1()
        .me()
        .mail_folder(MAIL_FOLDER_ID)
        .child_folder(child_folder_id1)
        .child_folder(child_folder_id2)
        .message(MESSAGE_ID)
        .attachment(ATTACHMENT_ID)
        .get_content()
        .send();

    println!("{:#?}", response);
}
