#![allow(dead_code)]

use graph_rs_sdk::{Graph, GraphResult, ODataQuery};

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";

fn main() {}

// https://learn.microsoft.com/en-us/graph/query-parameters?tabs=http

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
        .expand(&["children"])
        .send()
        .await?;

    Ok(())
}

async fn filter() -> GraphResult<()> {
    let client = Graph::new(ACCESS_TOKEN);

    let _ = client
        .users()
        .list_user()
        .filter(&["userType eq 'Member'"])
        .send()
        .await?;

    Ok(())
}

async fn order_by() -> GraphResult<()> {
    let client = Graph::new(ACCESS_TOKEN);

    let _ = client
        .users()
        .list_user()
        .order_by(&["displayName"])
        .send()
        .await?;

    Ok(())
}

async fn format() -> GraphResult<()> {
    let client = Graph::new(ACCESS_TOKEN);

    let _ = client.users().list_user().format("json").send().await?;

    Ok(())
}

async fn count() -> GraphResult<()> {
    let client = Graph::new(ACCESS_TOKEN);

    let _ = client.users().list_user().count("true").send().await?;

    Ok(())
}

async fn search() -> GraphResult<()> {
    let client = Graph::new(ACCESS_TOKEN);

    let _ = client.users().list_user().search("pizza").send().await?;

    Ok(())
}
