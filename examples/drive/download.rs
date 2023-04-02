use graph_rs_sdk::http::FileConfig;
use graph_rs_sdk::prelude::*;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";

static ITEM_ID: &str = "ITEM_ID";

// File path in OneDrive starting from the root.
// The path should always start with :/ and end with : such as :/Documents/item.txt:
static ONEDRIVE_FILE_PATH: &str = ":/ONEDRIVE_FILE_PATH:";

pub async fn download_files() {
    download().await;
    download_and_format("pdf").await;
    download_and_rename("FILE_NAME").await;
    download_by_path(ONEDRIVE_FILE_PATH).await;
}

pub async fn download() {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client
        .me()
        .drive()
        .item(ITEM_ID)
        .get_items_content()
        .download(&FileConfig::new("./examples/example_files").create_directories(true))
        .await
        .unwrap();

    println!("{response:#?}");

    let path_buf = response.into_body();
    println!("{:#?}", path_buf.metadata());
}

pub async fn download_file_as_bytes() {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client
        .me()
        .drive()
        .item(ITEM_ID)
        .get_items_content()
        .send()
        .await
        .unwrap();

    let bytes = response.bytes().await.unwrap();
    println!("{bytes:#?}");
}

pub async fn download_file_as_string() {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client
        .me()
        .drive()
        .item(ITEM_ID)
        .get_items_content()
        .send()
        .await
        .unwrap();

    let bytes = response.text().await.unwrap();
    println!("{bytes:#?}");
}

// You can convert a file to a different format using the format() method.
// There are 4 formats: glb, html, jpg, and pdf that an item can be converted to.
// This uses the PDF conversion which can be converted from: doc, docx, epub,
// eml, htm, html, md, msg, odp, ods, odt, pps, ppsx, ppt, pptx, rtf, tif, tiff, xls, xlsm, and xlsx.
//
// For more info on download formats see:
// https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitem_get_content_format?view=odsp-graph-online
pub async fn download_and_format(format: &str) {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client
        .me()
        .drive()
        .item(ITEM_ID)
        .get_items_content()
        .format(format)
        .download(
            &FileConfig::new("./examples/example_files")
                .create_directories(true)
                .file_name(OsStr::new("file.pdf")),
        )
        .await
        .unwrap();

    println!("{response:#?}");

    let path_buf = response.into_body();
    println!("{:#?}", path_buf.metadata());
}

pub async fn download_and_rename(name: &str) {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client
        .me()
        .drive()
        .item(ITEM_ID)
        .get_items_content()
        .download(
            &FileConfig::new("./examples/example_files")
                .create_directories(true)
                .file_name(OsStr::new(name)),
        )
        .await
        .unwrap();

    println!("{response:#?}");

    let path_buf = response.into_body();
    println!("{:#?}", path_buf.metadata());
}

// The path should always start with :/ and end with :
// such as :/Documents/item.txt:
pub async fn download_by_path(path: &str) {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client
        .me()
        .drive()
        .item_by_path(path)
        .get_items_content()
        .download(
            &FileConfig::new("./examples/example_files")
                .create_directories(true)
                .file_name(OsStr::new("item.txt")),
        )
        .await
        .unwrap();

    println!("{response:#?}");

    let path_buf = response.into_body();
    println!("{:#?}", path_buf.metadata());
}

// The default settings for downloading is to NOT create any non-existing directory.
// You can change this by setting FileConfig with create directories to true.
// Any missing directory when this is not true will cause the request to fail.
#[allow(dead_code)]
pub async fn download_with_config() {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client
        .me()
        .drive()
        .item(ITEM_ID)
        .get_items_content()
        .download(
            &FileConfig::new("./examples/example_files").create_directories(true), // Create directories in the path if they do not exist.
        )
        .await
        .unwrap();

    println!("{response:#?}");

    let path_buf = response.into_body();
    println!("{:#?}", path_buf.metadata());
}
