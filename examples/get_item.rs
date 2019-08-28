use rust_onedrive::prelude::*;

fn main() {
    // Get the drive item metadata based on its id.
    get_drive_item("ITEM_ID");
    get_sites_drive_item("ITEM_ID", "RESOURCE_ID");
}

fn get_drive_item(item_id: &str) {
    let drive = Drive::new("ACCESS TOKEN");
    let mut request = drive.v1().me().get_item().by_id(item_id);
    let drive_item: DriveItem = request.send().unwrap();
    println!("{:#?}", drive_item);
}

// Or use one of the other locations that a drive could refer to
// such as drives, users, groups, and sites.
// The resource_id is the id for this location (sites, users, etc).
fn get_sites_drive_item(item_id: &str, resource_id: &str) {
    let drive = Drive::new("ACCESS_TOKEN");
    let mut request = drive.v1().sites().get_item().by_id(item_id, resource_id);
    let drive_item: DriveItem = request.send().unwrap();
    println!("{:#?}", drive_item);
}
