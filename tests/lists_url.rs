use graph_rs::prelude::*;
use test_tools::assert_url_eq;

#[test]
pub fn lists_activities() {
    let client = Graph::new("");
    let _ = client.v1().site("32p99453").list("32p99453").get_lists();
    assert_url_eq(&client, "/sites/32p99453/lists/32p99453");

    let _ = client
        .v1()
        .site("32p99453")
        .list("1s390sd")
        .item("32fafawe")
        .get_items();
    assert_url_eq(&client, "/sites/32p99453/lists/1s390sd/items/32fafawe");

    let _ = client
        .v1()
        .site("32p99453")
        .lists()
        .create_lists(&String::new());
    assert_url_eq(&client, "/sites/32p99453/lists");
}
