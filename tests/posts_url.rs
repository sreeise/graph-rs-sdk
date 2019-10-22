use graph_rs::prelude::*;
use test_tools::assert_url_eq;

static RID: &str = "T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x";
static ID: &str = "b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI";

#[test]
fn conversation_thread_posts() {
    let client = Graph::new("");

    client.v1().groups(RID).thread_posts().list(ID);
    assert_url_eq(&client, format!("/groups/{}/threads/{}/posts", RID, ID));

    client.v1().groups(RID).thread_posts().get(ID, ID);
    assert_url_eq(
        &client,
        format!("/groups/{}/threads/{}/posts/{}", RID, ID, ID),
    );

    client
        .v1()
        .groups(RID)
        .thread_posts()
        .reply(ID, ID, &serde_json::json!({}));
    assert_url_eq(
        &client,
        format!("/groups/{}/threads/{}/posts/{}/reply", RID, ID, ID),
    );

    client
        .v1()
        .groups(RID)
        .thread_posts()
        .forward(ID, ID, &serde_json::json!({}));
    assert_url_eq(
        &client,
        format!("/groups/{}/threads/{}/posts/{}/forward", RID, ID, ID),
    );
}

#[test]
fn conversation_posts() {
    let client = Graph::new("");

    client.v1().groups(RID).conversation_posts().list(ID, ID);
    assert_url_eq(
        &client,
        format!("/groups/{}/conversations/{}/threads/{}/posts", RID, ID, ID),
    );

    client.v1().groups(RID).conversation_posts().get(ID, ID, ID);
    assert_url_eq(
        &client,
        format!(
            "/groups/{}/conversations/{}/threads/{}/posts/{}",
            RID, ID, ID, ID
        ),
    );

    client
        .v1()
        .groups(RID)
        .conversation_posts()
        .reply(ID, ID, ID, &serde_json::json!({}));
    assert_url_eq(
        &client,
        format!(
            "/groups/{}/conversations/{}/threads/{}/posts/{}/reply",
            RID, ID, ID, ID
        ),
    );

    client
        .v1()
        .groups(RID)
        .conversation_posts()
        .forward(ID, ID, ID, &serde_json::json!({}));
    assert_url_eq(
        &client,
        format!(
            "/groups/{}/conversations/{}/threads/{}/posts/{}/forward",
            RID, ID, ID, ID
        ),
    );
}
