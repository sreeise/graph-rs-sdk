use graph_rs_sdk::http::ChannelResponse;
use graph_rs_sdk::prelude::*;

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";

pub async fn channel_next_links() -> GraphResult<()> {
    let client = Graph::new(ACCESS_TOKEN);
    let mut receiver = client
        .users()
        .list_user()
        .paging()
        .channel::<serde_json::Value>()
        .await?;

    while let Some(channel_response) = receiver.recv().await {
        match channel_response {
            ChannelResponse::Next(result) => match result {
                Ok(response) => {
                    println!("response:\n{response:#?}\n\n");
                }
                Err(err) => {
                    println!("GraphFailure: {err:#?}");
                    break;
                }
            },
            ChannelResponse::Done => break,
        }
    }
    Ok(())
}
