use graph_rs_sdk::groups::{GroupsApiClient, GroupsIdApiClient};
use graph_rs_sdk::prelude::*;

static RID: &str = "T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x";
static ID: &str = "b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI";

fn client() -> GroupsApiClient {
    Graph::new("").groups()
}
fn client_id(id: &str) -> GroupsIdApiClient {
    Graph::new("").group(id)
}

#[test]
fn groups_url() {
    assert_eq!(client().list_group().url().path(), "/v1.0/groups");
    assert_eq!(
        client_id(RID).get_group().url().path(),
        format!("/v1.0/groups/{}", RID)
    );
    assert_eq!(client().delta().url().path(), "/v1.0/groups/delta()");
    assert_eq!(
        client().create_group(&serde_json::json!({})).url().path(),
        "/v1.0/groups"
    );
    assert_eq!(
        client_id(RID).update_group(&String::new()).url().path(),
        format!("/v1.0/groups/{}", RID)
    );
    assert_eq!(
        client_id(RID).delete_group().url().path(),
        format!("/v1.0/groups/{}", RID)
    );
    assert_eq!(
        client_id(RID)
            .add_member(&serde_json::json!({}))
            .url()
            .path(),
        format!("/v1.0/groups/{}/members/$ref", RID)
    );
    assert_eq!(
        client_id(RID)
            .add_owner(&serde_json::json!({}))
            .url()
            .path(),
        format!("/v1.0/groups/{}/owners/$ref", RID)
    );
    assert_eq!(
        client_id(RID).subscribe_by_mail().url().path(),
        format!("/v1.0/groups/{}/subscribeByMail", RID)
    );
}

#[test]
fn group_conversation() {
    assert_eq!(
        client_id(RID)
            .conversations()
            .list_conversations()
            .url()
            .path(),
        format!("/v1.0/groups/{}/conversations", RID)
    );
    assert_eq!(
        client_id(RID).conversation(ID).list_threads().url().path(),
        format!("/v1.0/groups/{}/conversations/{}/threads", RID, ID)
    );
    assert_eq!(
        client_id(RID)
            .conversation(ID)
            .delete_conversations()
            .url()
            .path(),
        format!("/v1.0/groups/{}/conversations/{}", RID, ID)
    );
}

#[test]
fn group_conversation_posts() {
    assert_eq!(
        client_id(RID).thread(ID).posts().list_posts().url().path(),
        format!("/v1.0/groups/{}/threads/{}/posts", RID, ID)
    );
    assert_eq!(
        client_id(RID)
            .thread(ID)
            .post(ID)
            .reply(&serde_json::json!({}))
            .url()
            .path(),
        format!("/v1.0/groups/{}/threads/{}/posts/{}/reply", RID, ID, ID)
    );

    assert_eq!(
        client_id(RID)
            .conversation(ID)
            .thread(ID)
            .list_posts()
            .url()
            .path(),
        format!(
            "/v1.0/groups/{}/conversations/{}/threads/{}/posts",
            RID, ID, ID
        )
    );
    assert_eq!(
        client_id(RID)
            .conversation(ID)
            .thread(ID)
            .post(ID)
            .get_posts()
            .url()
            .path(),
        format!(
            "/v1.0/groups/{}/conversations/{}/threads/{}/posts/{}",
            RID, ID, ID, ID
        )
    );
    assert_eq!(
        client_id(RID)
            .conversation(ID)
            .thread(ID)
            .post(ID)
            .reply(&serde_json::json!({}))
            .url()
            .path(),
        format!(
            "/v1.0/groups/{}/conversations/{}/threads/{}/posts/{}/reply",
            RID, ID, ID, ID
        )
    );
}
