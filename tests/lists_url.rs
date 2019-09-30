use graph_rs::prelude::*;
use test_tools::drive::assert_url_eq;

#[test]
pub fn lists_activities() {
    let client = Graph::new("");
    let _ = client.v1().me().lists().activity("32p99453");
    assert_url_eq(&client, "/me/lists/32p99453/activities");

    let _ = client.v1().sites("32p99453").lists().activity("1s390sd");
    assert_url_eq(&client, "/sites/32p99453/lists/1s390sd/activities");

    let _ = client
        .v1()
        .sites("32p99453")
        .lists()
        .item_activity("1s390sd", "132534");
    assert_url_eq(&client, "/sites/32p99453/lists/1s390sd/activities/132534");
}
