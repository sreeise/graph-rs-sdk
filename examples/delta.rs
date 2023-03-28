use graph_error::GraphFailure;
use graph_rs_sdk::prelude::*;

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";

#[tokio::main]
async fn main() -> Result<(), GraphFailure> {
    let client = Graph::new(ACCESS_TOKEN);
    let mut delta_recv = client
        .users()
        .delta()
        .channel_next_links::<serde_json::Value>()
        .await
        .unwrap();

    loop {
        match delta_recv.recv().await {
            Some(next_link_response) => {
                if let Some(err) = next_link_response.err() {
                    panic!("Error {err:#?}");
                }
                assert!(next_link_response.is_success());
            }
            None => {
                println!("Got None");
                break;
            }
        }
    }
    Ok(())
}
