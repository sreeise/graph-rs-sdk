use graph_rs_sdk::prelude::*;
use test_tools::assert_url_eq;

#[test]
pub fn lists_activities() {
    let client = Graph::new("");

    assert_eq!(
        "/v1.0/sites/32p99453/lists/32p99453".to_string(),
        client
            .site("32p99453")
            .list("32p99453")
            .get_lists()
            .url()
            .path()
    );

    let _ = client
        .v1()
        .site("32p99453")
        .list("1s390sd")
        .item("32fafawe")
        .get_items();
    assert_url_eq(&client, "/sites/32p99453/lists/1s390sd/items/32fafawe");

    assert_eq!(
        "/v1.0/sites/32p99453/lists/32p99453".to_string(),
        client.site("32p99453").list("32p99453").url().path()
    );

    let _ = client
        .v1()
        .site("32p99453")
        .lists()
        .create_lists(&String::new());
    assert_url_eq(&client, "/sites/32p99453/lists");
}
