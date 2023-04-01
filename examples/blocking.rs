use graph_rs_sdk::prelude::*;

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";

fn main() -> GraphResult<()> {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client.users().list_user().send()?;

    println!("{response:#?}");

    let body: serde_json::Value = response.json()?;
    println!("{body:#?}");

    Ok(())
}
