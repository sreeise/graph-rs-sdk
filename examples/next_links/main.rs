mod stream;

use stream::*;

#[tokio::main]
async fn main() {
    stream_next_links().await;
}
