use futures_util::stream::StreamExt;
use graph_rs_sdk::prelude::*;

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";

pub async fn stream_next_links() {
    let client = Graph::new(ACCESS_TOKEN);

    let mut stream = client
        .users()
        .list_user()
        .select(&["id", "userPrincipalName"])
        .top("5")
        .paging()
        .stream::<serde_json::Value>()
        .unwrap();

    while let Some(Ok(value)) = stream.next().await {
        println!("{value:?}");
    }
}
