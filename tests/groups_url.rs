use graph_rs::prelude::*;
use test_tools::assert_url_eq;

static RID: &str = "T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x";
static ID: &str = "b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI";

#[test]
fn groups_common() {
    let client = Graph::new("");

    let _ = client.v1().groups(RID).list();
    assert_url_eq(&client, "/groups");

    let _ = client.v1().groups(RID).get();
    assert_url_eq(&client, format!("/groups/{}", RID));

    let _ = client.v1().groups(RID).delta();
    assert_url_eq(&client, "/groups/delta");

    let _ = client.v1().groups(RID).create(&serde_json::json!({}));
    assert_url_eq(&client, "/groups");

    let _ = client.v1().groups(RID).update(&serde_json::json!({}));
    assert_url_eq(&client, format!("/groups/{}", RID));

    let _ = client.v1().groups(RID).delete();
    assert_url_eq(&client, format!("/groups/{}", RID));
}

#[test]
fn groups_add_methods() {
    let client = Graph::new("");

    let _ = client.v1().groups(RID).add_favorite();
    assert_url_eq(&client, format!("/groups/{}/addFavorite", RID));

    let _ = client.v1().groups(RID).add_member(&serde_json::json!({}));
    assert_url_eq(&client, format!("/groups/{}/members/$ref", RID));

    let _ = client.v1().groups(RID).add_owner(&serde_json::json!({}));
    assert_url_eq(&client, format!("/groups/{}/owners/$ref", RID));
}

#[test]
fn groups_remove_methods() {
    let client = Graph::new("");

    let _ = client.v1().groups(RID).remove_favorite();
    assert_url_eq(&client, format!("/groups/{}/removeFavorite", RID));

    let _ = client.v1().groups(RID).remove_member(ID);
    assert_url_eq(&client, format!("/groups/{}/members/{}/$ref", RID, ID));

    let _ = client.v1().groups(RID).remove_owner(ID);
    assert_url_eq(&client, format!("/groups/{}/owners/{}/$ref", RID, ID));
}

#[test]
fn renew_group() {
    let client = Graph::new("");

    let _ = client.v1().groups(RID).renew();
    assert_url_eq(&client, format!("/groups/{}/renew", RID));
}

#[test]
fn subscribe_mail() {
    let client = Graph::new("");

    let _ = client.v1().groups(RID).subscribe_by_mail();
    assert_url_eq(&client, format!("/groups/{}/subscribeByMail", RID));

    let _ = client.v1().groups(RID).unsubscribe_by_mail();
    assert_url_eq(&client, format!("/groups/{}/unsubscribeByMail", RID));
}

#[test]
fn list_methods() {
    let client = Graph::new("");

    let _ = client.v1().groups(RID).list_members();
    assert_url_eq(&client, format!("/groups/{}/members", RID));

    let _ = client.v1().groups(RID).list_member_of();
    assert_url_eq(&client, format!("/groups/{}/memberOf", RID));

    let _ = client.v1().groups(RID).list_transitive_members();
    assert_url_eq(&client, format!("/groups/{}/transitiveMembers", RID));

    let _ = client.v1().groups(RID).list_transitive_member_of();
    assert_url_eq(&client, format!("/groups/{}/transitiveMemberOf", RID));

    let _ = client.v1().groups(RID).list_owners();
    assert_url_eq(&client, format!("/groups/{}/owners", RID));

    let _ = client.v1().groups(RID).list_photos();
    assert_url_eq(&client, format!("/groups/{}/photos", RID));
}

#[test]
fn group_lifecycle_policies() {
    let client = Graph::new("");

    let _ = client
        .v1()
        .group_lifecycle_policies()
        .list_group_lifecycle_policy();
    assert_url_eq(&client, "/groupLifecyclePolicies");

    let _ = client
        .v1()
        .group_lifecycle_policies()
        .get_group_lifecycle_policy(RID);
    assert_url_eq(&client, format!("/groupLifecyclePolicies/{}", RID));

    let _ = client
        .v1()
        .group_lifecycle_policies()
        .update_group_lifecycle_policy(RID, &serde_json::json!({}));
    assert_url_eq(&client, format!("/groupLifecyclePolicies/{}", RID));

    let _ = client
        .v1()
        .group_lifecycle_policies()
        .delete_group_lifecycle_policy(RID);
    assert_url_eq(&client, format!("/groupLifecyclePolicies/{}", RID));

    let _ = client
        .v1()
        .group_lifecycle_policies()
        .remove_group(RID, &serde_json::json!({}));
    assert_url_eq(
        &client,
        format!("/groupLifecyclePolicies/{}/removeGroup", RID),
    );

    let _ = client
        .v1()
        .group_lifecycle_policies()
        .add_group(RID, &serde_json::json!({}));
    assert_url_eq(&client, format!("/groupLifecyclePolicies/{}/addGroup", RID));
}

#[test]
fn group_conversation() {
    let client = Graph::new("");

    let _ = client.v1().groups(RID).conversations().list();
    assert_url_eq(&client, format!("/groups/{}/conversations", RID));

    let _ = client.v1().groups(RID).conversations().list_threads(ID);
    assert_url_eq(
        &client,
        format!("/groups/{}/conversations/{}/threads", RID, ID),
    );

    let _ = client
        .v1()
        .groups(RID)
        .conversations()
        .list_accepted_senders();
    assert_url_eq(&client, format!("/groups/{}/acceptedSenders", RID));

    let _ = client.v1().groups(RID).conversations().get(ID);
    assert_url_eq(&client, format!("/groups/{}/conversations/{}", RID, ID));

    let _ = client
        .v1()
        .groups(RID)
        .conversations()
        .create(&serde_json::json!({}));
    assert_url_eq(&client, format!("/groups/{}/conversations", RID));

    let _ = client
        .v1()
        .groups(RID)
        .conversations()
        .create_thread(ID, &serde_json::json!({}));
    assert_url_eq(
        &client,
        format!("/groups/{}/conversations/{}/threads", RID, ID),
    );

    let _ = client
        .v1()
        .groups(RID)
        .conversations()
        .create_accepted_sender(&serde_json::json!({}));
    assert_url_eq(&client, format!("/groups/{}/acceptedSenders/$ref", RID));

    let _ = client.v1().groups(RID).conversations().delete(ID);
    assert_url_eq(&client, format!("/groups/{}/conversations/{}", RID, ID));
}

#[test]
fn group_conversation_posts() {
    let client = Graph::new("");

    let _ = client
        .v1()
        .groups(RID)
        .conversations()
        .conversation_posts()
        .list(ID, ID);
    assert_url_eq(
        &client,
        format!("/groups/{}/conversations/{}/threads/{}/posts", RID, ID, ID),
    );

    let _ = client
        .v1()
        .groups(RID)
        .conversations()
        .conversation_posts()
        .get(ID, ID, ID);
    assert_url_eq(
        &client,
        format!(
            "/groups/{}/conversations/{}/threads/{}/posts/{}",
            RID, ID, ID, ID
        ),
    );

    let _ = client
        .v1()
        .groups(RID)
        .conversations()
        .conversation_posts()
        .reply(ID, ID, ID, &serde_json::json!({}));
    assert_url_eq(
        &client,
        format!(
            "/groups/{}/conversations/{}/threads/{}/posts/{}/reply",
            RID, ID, ID, ID
        ),
    );

    let _ = client
        .v1()
        .groups(RID)
        .conversations()
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

#[test]
fn group_conversation_thread_posts() {
    let client = Graph::new("");

    let _ = client
        .v1()
        .groups(RID)
        .conversations()
        .thread_posts()
        .list(ID);
    assert_url_eq(&client, format!("/groups/{}/threads/{}/posts", RID, ID));

    let _ = client
        .v1()
        .groups(RID)
        .conversations()
        .thread_posts()
        .get(ID, ID);
    assert_url_eq(
        &client,
        format!("/groups/{}/threads/{}/posts/{}", RID, ID, ID),
    );

    let _ = client
        .v1()
        .groups(RID)
        .conversations()
        .thread_posts()
        .reply(ID, ID, &serde_json::json!({}));
    assert_url_eq(
        &client,
        format!("/groups/{}/threads/{}/posts/{}/reply", RID, ID, ID),
    );

    let _ = client
        .v1()
        .groups(RID)
        .conversations()
        .thread_posts()
        .forward(ID, ID, &serde_json::json!({}));
    assert_url_eq(
        &client,
        format!("/groups/{}/threads/{}/posts/{}/forward", RID, ID, ID),
    );
}
