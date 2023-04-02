#![allow(dead_code)]

use graph_rs_sdk::prelude::*;

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";

fn main() -> GraphResult<()> {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client.users().list_user().into_blocking().send()?;

    println!("{response:#?}");

    let body: serde_json::Value = response.json()?;
    println!("{body:#?}");

    Ok(())
}

fn paging_json() -> GraphResult<()> {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client
        .users()
        .delta()
        .into_blocking()
        .paging()
        .json::<serde_json::Value>()?;

    println!("{response:#?}");

    Ok(())
}

fn paging_channel() -> GraphResult<()> {
    let client = Graph::new(ACCESS_TOKEN);

    let receiver = client
        .users()
        .delta()
        .into_blocking()
        .paging()
        .channel::<serde_json::Value>()?;

    while let Some(result) = receiver.recv()? {
        let response = result?;
        println!("{response:#?}");
    }

    Ok(())
}
