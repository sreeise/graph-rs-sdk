use graph_rs::prelude::*;
use graph_rs_types::entitytypes::DriveItem;

fn main() {
    // Get the drive item metadata based on its id.
    get_drive_item("ITEM_ID");
    get_sites_drive_item("ITEM_ID", "RESOURCE_ID");
}

fn get_drive_item(item_id: &str) {
    let graph = Graph::new("ACCESS_TOKEN");
    let drive_item: GraphResponse<DriveItem> = graph
        .v1()
        .me()
        .drive()
        .get_item()
        .by_id(item_id)
        .send()
        .unwrap();
    println!("{:#?}", drive_item);
}

// Or use one of the other locations that a drive could refer to
// such as drives, users, groups, and sites.
// The resource_id is the id for this location (sites, users, etc).
fn get_sites_drive_item(item_id: &str, resource_id: &str) {
    let graph = Graph::new("ACCESS_TOKEN");
    let drive_item: GraphResponse<DriveItem> = graph
        .v1()
        .sites()
        .drive()
        .get_item()
        .by_ids(item_id, resource_id)
        .send()
        .unwrap();
    println!("{:#?}", drive_item);
}
