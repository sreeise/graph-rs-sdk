use rust_onedrive::drive::driveitem::DriveItem;
use rust_onedrive::drive::event::DownloadFormat;
use rust_onedrive::drive::Drive;
use rust_onedrive::drive::Item;
use rust_onedrive::drive::EP;
use rust_onedrive::from_to::*;
use rust_onedrive::oauth::OAuth;
use std::convert::TryFrom;
use std::path::PathBuf;

fn main() {
    let oauth = OAuth::from_json_file("./examples/example_files/web_oauth.json").unwrap();
    let mut drive = Drive::try_from(oauth).unwrap();
    let drive_item: DriveItem = drive.drive_root_child().unwrap();
    // DriveItem stores a Vec consisting of Values that are resources in a users drive
    // such as documents, folders, etc.
    println!("{:#?}", drive_item);

    // To download an item, pick one of the resources in Vec<Value>.
    // If the download is successful, a PathBuf is returned. Note, that the
    // download method is not optimized for large files or folders with many
    // files. This feature is still being worked on.
    let mut value = drive_item.value_idx(0);
    let path_buf: PathBuf = drive
        .download("./examples/example_files", &mut value)
        .unwrap();
    println!("{:#?}", path_buf.metadata());
}

// Set the name of the file you want to download here. The file must be in
// the root of your drive. This is the same place where your normal Documents,
// Attachments, Pictures, Desktop, etc. folders are.
static DRIVE_FILE: &str = "YOUR_DRIVE_FILE_NAME";

pub fn download_from_root() {
    // Get the access token from OAuth for the Drive API.
    let oauth: OAuth = OAuth::from_json_file("./examples/example_files/web_oauth.json").unwrap();
    let mut drive = Drive::try_from(oauth).unwrap();

    // Call the API. drive_root_child is the files in the users main documents folder.
    let mut drive_item: DriveItem = drive.drive_root_child().unwrap();
    // Save the metadata of the files.
    drive_item
        .to_json_file("./examples/example_files/drive_root_child.json")
        .unwrap();

    // Get the Values vec that lists the files.
    let mut value = drive_item.find_by_name(DRIVE_FILE).unwrap();

    // Download the file. The file will be downloaded with the same name.
    let path_buf: PathBuf = drive
        .download("./examples/example_files", &mut value)
        .unwrap();
    println!("{:#?}", path_buf);
}

// You can convert a file to a different format using the download_format() method.
// There are 4 formats: glb, html, jpg, and pdf that an item can be converted to.
// There are several that they can be converted from. The method takes the
// download directory, the value to download, and the type of conversion using
// the enum DownloadFormat. Just like the previous example, this one below also assumes
// that the file is in the same place where your normal Documents, Attachments,
// Pictures, Desktop, etc. are.
//
// This uses the PDF conversion which can be converted from: doc, docx, epub,
// eml, htm, html, md, msg, odp, ods, odt, pps, ppsx, ppt, pptx, rtf, tif, tiff, xls, xlsm, and xlsx.
//
// For more info on download formats see:
// https://docs.microsoft.com/en-us/onedrive/developer/rest-api/api/driveitem_get_content_format?view=odsp-graph-online
pub fn download_from_root_and_format() {
    // Get the access token from OAuth for the Drive API.
    let oauth: OAuth = OAuth::from_json_file("./examples/example_files/web_oauth.json").unwrap();
    let mut drive = Drive::try_from(oauth).unwrap();

    // Call the API. drive_root_child is the files in the users main documents folder.
    let mut drive_item: DriveItem = drive.drive_root_child().unwrap();

    // Save the metadata of the files.
    drive_item
        .to_json_file("./examples/example_files/drive_root_child.json")
        .unwrap();

    // Get the Values vec that lists the files.
    let mut value = drive_item.find_by_name(DRIVE_FILE).unwrap();

    // Download the file. Currently, the file may or may not be downloaded with the same
    // name when downloading by format. This is being worked on in issue #51.
    let path_buf: PathBuf = drive
        .download_format("./examples/example_files", &mut value, DownloadFormat::PDF)
        .unwrap();
    println!("{:#?}", path_buf.metadata());
}
