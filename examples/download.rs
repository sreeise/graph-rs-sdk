use graph_rs_sdk::prelude::*;
use std::ffi::OsString;
use std::path::PathBuf;

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";
static ITEM_ID: &str = "ITEM_ID";

fn main() {
    download();
    download_and_format("pdf");
    download_and_rename("FILE_NAME");
    download_by_path(":/Documents/item.txt:")
}

pub fn download() {
    let client = Graph::new(ACCESS_TOKEN);

    // Download the file. The file will be downloaded with the same name.
    let download_client = client
        .v1()
        .me()
        .drive()
        .download(ITEM_ID, "./examples/example_files");

    let path_buf = download_client.send().unwrap();

    println!("{:#?}", path_buf);
}

// You can convert a file to a different format using the download_format() method.
// There are 4 formats: glb, html, jpg, and pdf that an item can be converted to.
// This uses the PDF conversion which can be converted from: doc, docx, epub,
// eml, htm, html, md, msg, odp, ods, odt, pps, ppsx, ppt, pptx, rtf, tif, tiff, xls, xlsm, and xlsx.
//
// For more info on download formats see:
// https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitem_get_content_format?view=odsp-graph-online
pub fn download_and_format(format: &str) {
    let client = Graph::new(ACCESS_TOKEN);

    let download_client = client
        .v1()
        .me()
        .drive()
        .download(ITEM_ID, "./examples/example_files");

    download_client.format(format);
    let path_buf: PathBuf = download_client.send().unwrap();

    println!("{:#?}", path_buf.metadata());
}

fn download_and_rename(name: &str) {
    let client = Graph::new(ACCESS_TOKEN);

    // Create the download request.
    let download_client = client
        .v1()
        .me()
        .drive()
        .download(ITEM_ID, "./examples/example_files");

    // // Rename the file or rename it after downloading using PathBuf.
    download_client.set_file_name(OsString::from(name));

    let path_buf: PathBuf = download_client.send().unwrap();

    println!("{:#?}", path_buf.metadata());
}

// The path should always start with :/ and end with :
// such as :/Documents/item.txt:
fn download_by_path(path: &str) {
    let client = Graph::new(ACCESS_TOKEN);

    // Create the download request.
    let download_client = client
        .v1()
        .me()
        .drive()
        .download(path, "./examples/example_files");

    let path_buf: PathBuf = download_client.send().unwrap();

    println!("{:#?}", path_buf.metadata());
}

// The default settings for downloading is to create
// any missing directory. You can change this by passing a
// download config. This will will fail if the directory does not exist.
#[allow(dead_code)]
fn download_with_config() {
    let client = Graph::new(ACCESS_TOKEN);

    let download_client = client
        .v1()
        .me()
        .drive()
        .download(ITEM_ID, "./example/example_files/download_dir");

    download_client.create_dir_all(false);

    let result = download_client.send();

    if let Ok(path_buf) = result {
        println!("{:#?}", path_buf);
    } else if let Err(e) = result {
        println!("{:#?}", e);
    }
}
