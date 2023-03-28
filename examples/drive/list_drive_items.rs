use graph_rs_sdk::prelude::*;

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";

pub async fn list_drive_items() {
    drive_root().await;
    drive_root_children().await;
    special_docs().await;
}

pub async fn drive_root() {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client.me().drive().get_root().send().await.unwrap();

    println!("{response:#?}");

    let drive_item: serde_json::Value = response.json().await.unwrap();
    println!("{drive_item:#?}");
}

pub async fn drive_root_children() {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client
        .me()
        .drive()
        .list_root_children()
        .send()
        .await
        .unwrap();

    println!("{response:#?}");

    let drive_item: serde_json::Value = response.json().await.unwrap();
    println!("{drive_item:#?}");
}

pub async fn special_docs() {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client
        .me()
        .drive()
        .get_special("documents")
        .send()
        .await
        .unwrap();

    println!("{response:#?}");

    let drive_item: serde_json::Value = response.json().await.unwrap();
    println!("{drive_item:#?}");
}
