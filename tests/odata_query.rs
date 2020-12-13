use graph_rs_sdk::prelude::*;
use test_tools::assert_url_eq;

#[test]
fn select_query() {
    let client = Graph::new("token");

    client.v1().me().drive().get_drive().select(&["id", "name"]);
    assert_url_eq(&client, "/me/drive?select=id%2Cname");
}

#[test]
fn expand_query() {
    let client = Graph::new("token");

    client.v1().me().drive().get_drive().expand(&["users"]);
    assert_url_eq(&client, "/me/drive?expand=users");
}

#[test]
fn filter_query() {
    let client = Graph::new("token");

    client
        .v1()
        .me()
        .drive()
        .get_drive()
        .filter(&["startsWith(displayName,'j')"]);
    assert_url_eq(
        &client,
        "/me/drive?filter=startsWith%28displayName%2C%27j%27%29",
    );
}

#[test]
fn expand_filter_query() {
    let client = Graph::new("token");

    client
        .v1()
        .me()
        .drive()
        .get_drive()
        .expand(&["users"])
        .filter(&["name"]);
    assert_url_eq(&client, "/me/drive?expand=users&filter=name");
}
