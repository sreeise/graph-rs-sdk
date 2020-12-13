use graph_rs_sdk::prelude::*;
use test_tools::assert_url_eq;

static RID: &str = "T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x";
static ID: &str = "b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI";

#[test]
fn groups_common() {
    let client = Graph::new("");

    let _ = client.v1().groups().list_group();
    assert_url_eq(&client, "/groups");

    let _ = client.v1().group(RID).get_group();
    assert_url_eq(&client, format!("/groups/{}", RID));

    let _ = client.v1().groups().delta();
    assert_url_eq(&client, "/groups/delta()");

    let _ = client.v1().groups().create_group(&String::new());
    assert_url_eq(&client, "/groups");

    let _ = client.v1().group(RID).update_group(&String::new());
    assert_url_eq(&client, format!("/groups/{}", RID));

    let _ = client.v1().group(RID).delete_group();
    assert_url_eq(&client, format!("/groups/{}", RID));
}

#[test]
fn groups_add_methods() {
    let client = Graph::new("");

    let _ = client.v1().group(RID).add_favorite();
    assert_url_eq(&client, format!("/groups/{}/addFavorite", RID));

    let _ = client.v1().group(RID).add_member(&serde_json::json!({}));
    assert_url_eq(&client, format!("/groups/{}/members/$ref", RID));

    let _ = client.v1().group(RID).add_owner(&serde_json::json!({}));
    assert_url_eq(&client, format!("/groups/{}/owners/$ref", RID));
}

#[test]
fn groups_remove_methods() {
    let client = Graph::new("");

    let _ = client.v1().group(RID).remove_favorite();
    assert_url_eq(&client, format!("/groups/{}/removeFavorite", RID));

    let _ = client.v1().group(RID).remove_member(ID);
    assert_url_eq(&client, format!("/groups/{}/members/{}/$ref", RID, ID));

    let _ = client.v1().group(RID).remove_owner(ID);
    assert_url_eq(&client, format!("/groups/{}/owners/{}/$ref", RID, ID));
}

#[test]
fn renew_group() {
    let client = Graph::new("");

    let _ = client.v1().group(RID).renew();
    assert_url_eq(&client, format!("/groups/{}/renew", RID));
}

#[test]
fn subscribe_mail() {
    let client = Graph::new("");

    let _ = client.v1().group(RID).subscribe_by_mail();
    assert_url_eq(&client, format!("/groups/{}/subscribeByMail", RID));

    let _ = client.v1().group(RID).unsubscribe_by_mail();
    assert_url_eq(&client, format!("/groups/{}/unsubscribeByMail", RID));
}

#[test]
fn list_methods() {
    let client = Graph::new("");

    let _ = client.v1().group(RID).list_members();
    assert_url_eq(&client, format!("/groups/{}/members", RID));

    let _ = client.v1().group(RID).list_member_of();
    assert_url_eq(&client, format!("/groups/{}/memberOf", RID));

    let _ = client.v1().group(RID).list_transitive_members();
    assert_url_eq(&client, format!("/groups/{}/transitiveMembers", RID));

    let _ = client.v1().group(RID).list_transitive_member_of();
    assert_url_eq(&client, format!("/groups/{}/transitiveMemberOf", RID));

    let _ = client.v1().group(RID).list_owners();
    assert_url_eq(&client, format!("/groups/{}/owners", RID));

    let _ = client.v1().group(RID).list_photos();
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

    let _ = client.v1().group(RID).conversations().list_conversations();
    assert_url_eq(&client, format!("/groups/{}/conversations", RID));

    let _ = client.v1().group(RID).conversation(ID).list_threads();
    assert_url_eq(
        &client,
        format!("/groups/{}/conversations/{}/threads", RID, ID),
    );

    let _ = client.v1().group(RID).list_accepted_senders();
    assert_url_eq(&client, format!("/groups/{}/acceptedSenders", RID));

    let _ = client.v1().group(RID).conversation(ID).get_conversations();
    assert_url_eq(&client, format!("/groups/{}/conversations/{}", RID, ID));

    let _ = client
        .v1()
        .group(RID)
        .conversations()
        .create_conversations(&String::new());
    assert_url_eq(&client, format!("/groups/{}/conversations", RID));

    let _ = client
        .v1()
        .group(RID)
        .conversation(ID)
        .create_threads(&String::new());
    assert_url_eq(
        &client,
        format!("/groups/{}/conversations/{}/threads", RID, ID),
    );

    let _ = client
        .v1()
        .group(RID)
        .create_accepted_senders(&String::new());
    assert_url_eq(&client, format!("/groups/{}/acceptedSenders", RID));

    let _ = client
        .v1()
        .group(RID)
        .conversation(ID)
        .delete_conversations();
    assert_url_eq(&client, format!("/groups/{}/conversations/{}", RID, ID));
}

#[test]
fn group_conversation_posts() {
    let client = Graph::new("");

    let _ = client
        .v1()
        .group(RID)
        .conversation(ID)
        .thread(ID)
        .list_posts();
    assert_url_eq(
        &client,
        format!("/groups/{}/conversations/{}/threads/{}/posts", RID, ID, ID),
    );

    let _ = client
        .v1()
        .group(RID)
        .conversation(ID)
        .thread(ID)
        .post(ID)
        .get_posts();
    assert_url_eq(
        &client,
        format!(
            "/groups/{}/conversations/{}/threads/{}/posts/{}",
            RID, ID, ID, ID
        ),
    );

    let _ = client
        .v1()
        .group(RID)
        .conversation(ID)
        .thread(ID)
        .post(ID)
        .reply(&String::new());
    assert_url_eq(
        &client,
        format!(
            "/groups/{}/conversations/{}/threads/{}/posts/{}/reply",
            RID, ID, ID, ID
        ),
    );

    let _ = client
        .v1()
        .group(RID)
        .conversation(ID)
        .thread(ID)
        .post(ID)
        .forward(&String::new());
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

    let _ = client.v1().group(RID).thread(ID).posts().list_posts();
    assert_url_eq(&client, format!("/groups/{}/threads/{}/posts", RID, ID));

    let _ = client.v1().group(RID).thread(ID).post(ID).get_posts();
    assert_url_eq(
        &client,
        format!("/groups/{}/threads/{}/posts/{}", RID, ID, ID),
    );

    let _ = client
        .v1()
        .group(RID)
        .thread(ID)
        .post(ID)
        .reply(&String::new());
    assert_url_eq(
        &client,
        format!("/groups/{}/threads/{}/posts/{}/reply", RID, ID, ID),
    );

    let _ = client
        .v1()
        .group(RID)
        .thread(ID)
        .post(ID)
        .forward(&String::new());
    assert_url_eq(
        &client,
        format!("/groups/{}/threads/{}/posts/{}/forward", RID, ID, ID),
    );
}
