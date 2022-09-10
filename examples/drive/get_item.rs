use graph_rs_sdk::prelude::*;

pub fn get_drive_item(item_id: &str) {
    let graph = Graph::new("ACCESS_TOKEN");
    let drive_item = graph.v1().me().drive().get_items(item_id).send().unwrap();
    println!("{:#?}", drive_item);
}

// Or use one of the other locations that a drive could refer to
// such as drives, users, groups, and sites.
// The resource_id is the id for this location (sites, users, etc).
pub fn get_sites_drive_item(item_id: &str, sites_id: &str) {
    let graph = Graph::new("ACCESS_TOKEN");
    let drive_item: GraphResponse<serde_json::Value> = graph
        .v1()
        .site(sites_id)
        .drive()
        .get_items(item_id)
        .send()
        .unwrap();
    println!("{:#?}", drive_item);
}
