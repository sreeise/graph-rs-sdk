#![allow(dead_code)]

use bytes::BytesMut;
use graph_error::GraphResult;

mod cancel_upload_session;
mod channel_upload_session;
mod stream_upload_session;
mod upload_bytes_iterator;
mod upload_file_iterator;

#[tokio::main]
async fn main() -> GraphResult<()> {
    upload_bytes_iterator::upload_bytes(bytes::Bytes::new()).await?;
    upload_file_iterator::upload_file(std::fs::File::open("file.docx")?).await?;

    channel_upload_session::channel(tokio::fs::File::open("file.docx")).await?;
    stream_upload_session::stream(BytesMut::new()).await?;

    Ok(())
}
