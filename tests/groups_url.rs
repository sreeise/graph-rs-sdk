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
        format!("/v1.0/groups/{RID}")
    );
    assert_eq!(client().delta().url().path(), "/v1.0/groups/delta()");

    assert_eq!(
        client().create_group(&serde_json::json!({})).url().path(),
        "/v1.0/groups"
    );

    assert_eq!(
        client_id(RID).update_group(&String::new()).url().path(),
        format!("/v1.0/groups/{RID}")
    );

    assert_eq!(
        client_id(RID).delete_group().url().path(),
        format!("/v1.0/groups/{RID}")
    );

    assert_eq!(
        client_id(RID)
            .create_ref_members(&serde_json::json!({}))
            .url()
            .path(),
        format!("/v1.0/groups/{RID}/members/$ref")
    );

    assert_eq!(
        client_id(RID)
            .owners()
            .create_ref_owners(&serde_json::json!({}))
            .url()
            .path(),
        format!("/v1.0/groups/{RID}/owners/$ref")
    );

    assert_eq!(
        client_id(RID).subscribe_by_mail().url().path(),
        format!("/v1.0/groups/{RID}/subscribeByMail")
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
        format!("/v1.0/groups/{RID}/conversations")
    );
    assert_eq!(
        client_id(RID)
            .conversation(ID)
            .threads()
            .list_threads()
            .url()
            .path(),
        format!("/v1.0/groups/{RID}/conversations/{ID}/threads")
    );
    assert_eq!(
        client_id(RID)
            .conversation(ID)
            .delete_conversations()
            .url()
            .path(),
        format!("/v1.0/groups/{RID}/conversations/{ID}")
    );
}

#[test]
fn group_conversation_posts() {
    assert_eq!(
        client_id(RID).thread(ID).posts().list_posts().url().path(),
        format!("/v1.0/groups/{RID}/threads/{ID}/posts")
    );
    assert_eq!(
        client_id(RID)
            .thread(ID)
            .post(ID)
            .reply(&serde_json::json!({}))
            .url()
            .path(),
        format!("/v1.0/groups/{RID}/threads/{ID}/posts/{ID}/reply")
    );

    assert_eq!(
        client_id(RID)
            .conversation(ID)
            .thread(ID)
            .posts()
            .list_posts()
            .url()
            .path(),
        format!(
            "/v1.0/groups/{RID}/conversations/{ID}/threads/{ID}/posts"
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
            "/v1.0/groups/{RID}/conversations/{ID}/threads/{ID}/posts/{ID}"
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
            "/v1.0/groups/{RID}/conversations/{ID}/threads/{ID}/posts/{ID}/reply"
        )
    );
}

#[test]
pub fn groups_permission_grants() {
    let client = Graph::new("");
    assert_eq!(
        format!("/v1.0/groups/{RID}/permissionGrants/getByIds"),
        client
            .group(RID)
            .permission_grants()
            .get_by_ids(&String::new())
            .url()
            .path()
    );

    assert_eq!(
        format!(
            "/v1.0/groups/{RID}/permissionGrants/{ID}/getMemberGroups"
        ),
        client
            .group(RID)
            .permission_grant(ID)
            .get_member_groups(&String::new())
            .url()
            .path()
    );
}
