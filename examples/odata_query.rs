#![allow(dead_code)]

use graph_rs_sdk::prelude::{Graph, GraphResult, ODataQuery};

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";

fn main() {}

async fn custom_path() -> GraphResult<()> {
    let client = Graph::new(ACCESS_TOKEN);

    let _ = client
        .users()
        .list_user()
        .extend_path(&["delta"])
        .send()
        .await?;

    Ok(())
}

async fn top() -> GraphResult<()> {
    let client = Graph::new(ACCESS_TOKEN);

    let _ = client.users().list_user().top("5").send().await?;

    Ok(())
}

async fn skip() -> GraphResult<()> {
    let client = Graph::new(ACCESS_TOKEN);

    let _ = client.users().list_user().skip("2").send().await?;

    Ok(())
}

async fn expand() -> GraphResult<()> {
    let client = Graph::new(ACCESS_TOKEN);

    let _ = client
        .users()
        .list_user()
        .expand(&["expand"])
        .send()
        .await?;

    Ok(())
}

async fn filter() -> GraphResult<()> {
    let client = Graph::new(ACCESS_TOKEN);

    let _ = client
        .users()
        .list_user()
        .filter(&["expand"])
        .send()
        .await?;

    Ok(())
}

async fn order_by() -> GraphResult<()> {
    let client = Graph::new(ACCESS_TOKEN);

    let _ = client
        .users()
        .list_user()
        .order_by(&["expand"])
        .send()
        .await?;

    Ok(())
}
