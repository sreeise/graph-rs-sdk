#![allow(dead_code)]

mod delete_page;
mod get_page_content;
mod upload_page_content;

#[tokio::main]
async fn main() {
    get_page_content::get_page_html_content().await;
    get_page_content::download_page_as_html().await;
    upload_page_content::upload_page_content().await;
    delete_page::delete_page().await;
}
