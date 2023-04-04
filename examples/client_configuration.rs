use graph_rs_sdk::header::HeaderMap;
use graph_rs_sdk::{Graph, GraphClientConfiguration};
use std::time::Duration;

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";

fn main() {
    let client_config = GraphClientConfiguration::new()
        .access_token(ACCESS_TOKEN)
        .timeout(Duration::from_secs(30))
        .default_headers(HeaderMap::default());

    let _ = Graph::from(client_config);
}
