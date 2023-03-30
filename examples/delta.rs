use futures_util::stream::StreamExt;
use graph_rs_sdk::error::GraphFailure;
use graph_rs_sdk::http::ChannelResponse;
use graph_rs_sdk::prelude::*;

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";

#[tokio::main]
async fn main() -> Result<(), GraphFailure> {
    channel().await?;
    delta_token().await?;
    stream_delta().await?;
    Ok(())
}

async fn channel() -> GraphResult<()> {
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

static DELTA_TOKEN: &str = "DELTA_TOKEN";

async fn delta_token() -> GraphResult<()> {
    let client = Graph::new(ACCESS_TOKEN);
    let mut receiver = client
        .users()
        .delta()
        .delta_token(DELTA_TOKEN)
        .paging()
        .channel::<serde_json::Value>()
        .await
        .unwrap();

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

async fn stream_delta() -> GraphResult<()> {
    let client = Graph::new(ACCESS_TOKEN);
    let mut stream = client
        .users()
        .delta()
        .paging()
        .stream::<serde_json::Value>()
        .unwrap();

    while let Some(result) = stream.next().await {
        match result {
            Ok(response) => {
                println!("{response:#?}");
                let body = response.into_body();
                println!("{body:#?}");
            }
            Err(err) => panic!("Error on stream users delta\n{err:#?}"),
        }
    }

    Ok(())
}
