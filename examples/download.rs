use rust_onedrive::drive::driveitem::DriveItem;
use rust_onedrive::drive::Download;
use rust_onedrive::drive::Drive;
use rust_onedrive::drive::EP;
use rust_onedrive::oauth::OAuth;
use rust_onedrive::transform::*;
use std::convert::TryFrom;

fn main() {
    let oauth = OAuth::from_file("./examples/example_files/web_oauth.json").unwrap();
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
    let path_buf = drive
        .download("./examples/example_files", &mut value)
        .unwrap();
    println!("{:#?}", path_buf.metadata());
}
