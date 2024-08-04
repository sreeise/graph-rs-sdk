use graph_rs_sdk::header::HeaderMap;
use graph_rs_sdk::{Graph, GraphClientConfiguration};
use std::time::Duration;

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";

fn main() {
    let client_config = GraphClientConfiguration::new()
        .access_token(ACCESS_TOKEN)
        .timeout(Duration::from_secs(30))
        .default_headers(HeaderMap::default())
        .retry(Some(10)) // retry 10 times if the request is not successful
        .concurrency_limit(Some(10)) // limit the number of concurrent requests on this client to 10
        .wait_for_retry_after_headers(true); // wait the amount of seconds specified by the Retry-After header of the response when we reach the throttling limits (429 Too Many Requests)

    let _ = Graph::from(client_config);
}
