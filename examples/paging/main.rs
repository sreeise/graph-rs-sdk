#![allow(dead_code)]

use graph_rs_sdk::error::GraphResult;

mod channel;
mod delta;
mod stream;

/// There are different levels of support for paging Microsoft Graph APIs. See the documentation,
/// [Paging Microsoft Graph data in your app](https://learn.microsoft.com/en-us/graph/paging),
/// for more info on supported APIs and availability.

#[tokio::main]
async fn main() -> GraphResult<()> {
    stream::stream_next_links().await?;
    channel::channel_next_links().await?;
    delta::channel_delta().await?;
    delta::stream_delta().await?;
    Ok(())
}
