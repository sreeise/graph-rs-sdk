use graph_rs::prelude::*;
use test_tools::drive::assert_url_eq;

#[test]
pub fn lists_activities() {
    let client = Graph::new("");
    let _ = client.v1().sites("32p99453").lists().get("32p99453");
    assert_url_eq(&client, "/sites/32p99453/lists/32p99453");

    let _ = client.v1().sites("32p99453").lists().items().get("1s390sd", "1s390sd");
    assert_url_eq(&client, "/sites/32p99453/lists/1s390sd/items/1s390sd");

    let _ = client
        .v1()
        .sites("32p99453")
        .lists()
        .create(&serde_json::json!({}));
    assert_url_eq(&client, "/sites/32p99453/lists");
}
