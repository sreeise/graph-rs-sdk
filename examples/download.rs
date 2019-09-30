use from_as::*;
use graph_rs::oauth::OAuth;
use graph_rs::prelude::*;
use std::convert::TryFrom;
use std::ffi::OsString;
use std::path::PathBuf;

static ITEM_ID: &str = "ITEM_ID";

fn main() {
    download();
    download_and_format("pdf");
    download_and_rename("FILE_NAME");
    download_by_path(":/Documents/item.txt:")
}

pub fn download() {
    // Get the access token from OAuth for the Drive API.
    let oauth: OAuth = OAuth::from_file("./examples/example_files/web_oauth.json").unwrap();
    let client = Graph::try_from(&oauth).unwrap();

    // Download the file. The file will be downloaded with the same name.
    let mut req = client
        .v1()
        .me()
        .drive()
        .download(ITEM_ID, "./examples/example_files")
        .unwrap();

    let path_buf: PathBuf = req.send().unwrap();
    println!("{:#?}", path_buf);
}

// You can convert a file to a different format using the download_format() method.
// There are 4 formats: glb, html, jpg, and pdf that an item can be converted to.
// This uses the PDF conversion which can be converted from: doc, docx, epub,
// eml, htm, html, md, msg, odp, ods, odt, pps, ppsx, ppt, pptx, rtf, tif, tiff, xls, xlsm, and xlsx.
//
// For more info on download formats see:
// https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitem_get_content_format?view=odsp-graph-online
pub fn download_and_format(extension: &str) {
    // Get the access token from OAuth for the Drive API.
    let oauth: OAuth = OAuth::from_file("./examples/example_files/web_oauth.json").unwrap();
    let client = Graph::try_from(&oauth).unwrap();

    let mut req = client
        .v1()
        .me()
        .drive()
        .download(ITEM_ID, "./examples/example_files")
        .unwrap();

    // Select the format.
    req.set_extension(extension);

    // Send the request and download the file.
    let path_buf: PathBuf = req.send().unwrap();

    println!("{:#?}", path_buf.metadata());
}

fn download_and_rename(name: &str) {
    // Get the access token from OAuth for the Drive API.
    let oauth: OAuth = OAuth::from_file("./examples/example_files/web_oauth.json").unwrap();
    let client = Graph::try_from(&oauth).unwrap();

    // Create the download request.
    let mut req = client
        .v1()
        .me()
        .drive()
        .download(ITEM_ID, "./examples/example_files")
        .unwrap();

    // Rename the file
    req.rename(OsString::from(name));

    // Send the request and download the file.
    let path_buf: PathBuf = req.send().unwrap();

    println!("{:#?}", path_buf.metadata());
}

// The path should always start with :/ and end with :
// such as :/Documents/item.txt:
fn download_by_path(path: &str) {
    let client = Graph::new("");

    // Create the download request.
    let mut req = client
        .v1()
        .me()
        .drive()
        .download(path, "./examples/example_files")
        .unwrap();

    // Send the request and download the file.
    let path_buf: PathBuf = req.send().unwrap();

    println!("{:#?}", path_buf.metadata());
}
