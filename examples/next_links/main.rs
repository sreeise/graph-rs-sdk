#![allow(dead_code)]

use graph_rs_sdk::error::GraphResult;

mod channel;
mod delta;
mod stream;

#[tokio::main]
async fn main() -> GraphResult<()> {
    stream::stream_next_links().await?;
    channel::channel_next_links().await?;
    delta::channel_delta().await?;
    delta::stream_delta().await?;
    Ok(())
}
