#![allow(dead_code)]

mod attachments;
mod child_folders;
mod mail_folders;
mod messages;

#[tokio::main]
async fn main() {
    attachments::get_attachment().await;
    messages::list_messages().await;
    mail_folders::get_me_inbox_messages().await;
}
