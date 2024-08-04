use graph_rs_sdk::*;

pub async fn get_drive_item(item_id: &str) {
    let client = GraphClient::new("ACCESS_TOKEN");

    let response = client
        .me()
        .drive()
        .item(item_id)
        .get_items()
        .send()
        .await
        .unwrap();

    println!("{response:#?}");

    let drive_item: serde_json::Value = response.json().await.unwrap();
    println!("{drive_item:#?}");
}

// Or use one of the other locations that a drive could refer to
// such as drives, users, groups, and sites.
// The resource_id is the id for this location (sites, users, etc).
pub async fn get_sites_drive_item(item_id: &str, sites_id: &str) {
    let client = GraphClient::new("ACCESS_TOKEN");

    let response = client
        .site(sites_id)
        .drive()
        .item(item_id)
        .get_items()
        .send()
        .await
        .unwrap();

    println!("{response:#?}");

    let drive_item: serde_json::Value = response.json().await.unwrap();
    println!("{drive_item:#?}");
}
