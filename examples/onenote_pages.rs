use graph_http::GraphResponse;
use graph_rs_sdk::prelude::*;
use std::ffi::OsString;
use std::str::FromStr;

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

fn main() {}

fn get_page_html_content() {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client
        .v1()
        .user(USER_ID)
        .onenote()
        .page(PAGE_ID)
        .content()
        .text()
        .unwrap();

    let html_string = response.body();
    println!("{:#?}", html_string);
}

fn download_page_as_html() {
    let client = Graph::new(ACCESS_TOKEN);

    let download_client = client
        .v1()
        .user(USER_ID)
        .onenote()
        .page(PAGE_ID)
        .content()
        .download(DOWNLOAD_PATH)
        .unwrap();

    download_client.rename(OsString::from_str(FILE_NAME).unwrap());

    let path_buf = download_client.send().unwrap();
    println!("{:#?}", path_buf);
}
