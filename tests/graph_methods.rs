use graph_rs_sdk::{client::Graph, GRAPH_URL, GRAPH_URL_BETA};

#[test]
fn graph_is_v1_or_beta() {
    let client = Graph::new("ACCESS_TOKEN");
    assert!(client.is_v1());

    let client = Graph::new("ACCESS_TOKEN");
    client.beta();
    assert!(client.is_beta());

    let client = Graph::new_async("ACCESS_TOKEN");
    assert!(client.is_v1());

    let client = Graph::new_async("ACCESS_TOKEN");
    client.beta();
    assert!(client.is_beta());

    client.beta().me().drive();

    client.url_ref(|url| {
        assert_eq!(url.to_string(), format!("{}/me", GRAPH_URL_BETA));
    });

    assert!(client.is_beta());

    client.v1().me().drive();

    client.url_ref(|url| {
        assert_eq!(url.to_string(), format!("{}/me", GRAPH_URL));
    });

    assert!(client.is_v1());
}
