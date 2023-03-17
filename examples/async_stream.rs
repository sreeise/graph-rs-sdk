use futures::StreamExt;
use graph_rs_sdk::prelude::*;

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";

#[tokio::main]
async fn main() {
    let client = Graph::new(ACCESS_TOKEN);
    let request = client.v1().users();

    let mut stream = request
        .list_user()
        .select(&["id", "userPrincipalName"])
        .top("5")
        .stream_next_links::<serde_json::Value>()
        .unwrap();

    while let Some(Ok(value)) = stream.next().await {
        println!("{value:?}");
    }
}
