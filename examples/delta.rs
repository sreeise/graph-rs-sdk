use graph_error::GraphFailure;
use graph_http::ChannelResponse;
use graph_rs_sdk::prelude::*;

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";

#[tokio::main]
async fn main() -> Result<(), GraphFailure> {
    let client = Graph::new(ACCESS_TOKEN);
    let mut receiver = client
        .users()
        .delta()
        .paging()
        .channel::<serde_json::Value>()
        .await
        .unwrap();

    while let Some(channel_response) = receiver.recv().await {
        match channel_response {
            ChannelResponse::Next(result) => {
                match result {
                    Ok(response) => {
                        println!("response:\n{:#?}\n\n", response);
                    }
                    Err(err) => {
                        println!("GraphFailure: {:#?}", err);
                        break;
                    }
                }
            }
            ChannelResponse::Done => break,
        }
    }
    Ok(())
}
