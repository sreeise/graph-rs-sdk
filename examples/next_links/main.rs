mod channel;
mod stream;

#[tokio::main]
async fn main() {
    stream::stream_next_links().await.unwrap();
    channel::channel_next_links().await.unwrap();
}
