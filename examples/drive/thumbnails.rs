use graph_rs_sdk::prelude::*;

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";

pub async fn list_thumbnails() {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client
        .me()
        .drive()
        .item_by_path(":/") // Root folder
        .list_thumbnails()
        .send()
        .await
        .unwrap();

    println!("{response:#?}");

    let drive_item: serde_json::Value = response.json().await.unwrap();
    println!("{drive_item:#?}");
}
