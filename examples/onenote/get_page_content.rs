use graph_http::traits::ResponseExt;
use graph_rs_sdk::http::FileConfig;
use graph_rs_sdk::*;
use std::ffi::OsStr;

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";

// User id that you want to access onenote for.
static USER_ID: &str = "RESOURCE_ID";

// Onenote page id if you want to perform onenote page API requests.
static PAGE_ID: &str = "PARENT_ID";

// File path if you want to perform download requests for pages as HTML files.
static DOWNLOAD_PATH: &str = "DOWNLOAD_PATH";

// File name for a page when you want to perform download requests for pages as HTML files.
// Include the file extension such as .html
static FILE_NAME: &str = "FILE_NAME";

pub async fn get_page_html_content() {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client
        .user(USER_ID)
        .onenote()
        .page(PAGE_ID)
        .get_pages_content()
        .send()
        .await
        .unwrap();

    let html_string = response.text().await.unwrap();
    println!("{html_string:#?}");
}

pub async fn download_page_as_html() {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client
        .user(USER_ID)
        .onenote()
        .page(PAGE_ID)
        .get_pages_content()
        .send()
        .await
        .unwrap();

    let response2 = response
        .download(&FileConfig::new(DOWNLOAD_PATH).file_name(OsStr::new(FILE_NAME)))
        .await
        .unwrap();

    let path_buf = response2.body();

    println!("{path_buf:#?}");
}
