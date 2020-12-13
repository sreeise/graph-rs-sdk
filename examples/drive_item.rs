use graph_rs_sdk::prelude::*;

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";

fn main() {
    drive_root();
    drive_root_children();
    special_docs();
}

fn drive_root() {
    let client = Graph::new(ACCESS_TOKEN);

    let drive_item: GraphResponse<serde_json::Value> =
        client.v1().me().drive().get_root().send().unwrap();
    println!("{:#?}", drive_item);
}

fn drive_root_children() {
    let client = Graph::new(ACCESS_TOKEN);

    let drive_item = client
        .v1()
        .me()
        .drive()
        .list_root_children()
        .send()
        .unwrap();
    println!("{:#?}", drive_item);
}

fn special_docs() {
    let client = Graph::new(ACCESS_TOKEN);

    let drive_item = client
        .v1()
        .me()
        .drive()
        .get_special("documents")
        .send()
        .unwrap();
    println!("{:#?}", drive_item);
}
