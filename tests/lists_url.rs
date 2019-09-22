use graph_rs::prelude::*;
use test_tools::drive::assert_url_eq;

#[test]
pub fn lists_activities() {
    let client = Graph::new("");
    let _ = client.v1().me().lists().item_activity("32p99453");
    client.format_ord();
    assert_url_eq(&client, "/me/lists/32p99453/activities");

    let _ = client
        .v1()
        .drives()
        .lists()
        .item_activity("132534")
        .by_id("32p99453");
    assert_url_eq(&client, "/drives/32p99453/lists/132534/activities");

    let _ = client
        .v1()
        .sites()
        .lists()
        .item_activity("132534")
        .by_id("32p99453");
    assert_url_eq(&client, "/sites/32p99453/lists/132534/activities");
}
