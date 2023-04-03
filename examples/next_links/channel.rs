use graph_rs_sdk::*;

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";

pub async fn channel_next_links() -> GraphResult<()> {
    let client = Graph::new(ACCESS_TOKEN);
    let mut receiver = client
        .users()
        .list_user()
        .paging()
        .channel::<serde_json::Value>()
        .await?;

    while let Some(result) = receiver.recv().await {
        match result {
            Ok(response) => {
                println!("response:\n{response:#?}\n\n");
            }
            Err(err) => {
                println!("GraphFailure: {err:#?}");
                break;
            }
        }
    }
    Ok(())
}
