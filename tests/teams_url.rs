use graph_http::BlockingHttpClient;
use graph_rs_sdk::prelude::*;
use test_tools::assert_url_eq;

static RID: &str = "T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x";
static ID: &str = "b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI";

fn get_graph() -> Graph<BlockingHttpClient> {
    Graph::new("")
}

#[test]
fn get_teams_request() {
    let client = get_graph();

    let _ = client.v1().teams().id(RID).get_team();
    assert_url_eq(&client, &format!("/teams/{}", RID));

    let _ = client.v1().team(RID).get_team();
    assert_url_eq(&client, &format!("/teams/{}", RID));
}

#[test]
fn list_teams_request() {
    let client = get_graph();

    let _ = client.v1().teams().list_team();
    assert_url_eq(&client, "/teams");
}

#[test]
fn teams_channel_request() {
    let client = get_graph();

    let _ = client.v1().team(RID).channels().list_channels();
    assert_url_eq(&client, &format!("/teams/{}/channels", RID));

    let _ = client.v1().team(RID).channel(ID).get_channels();
    assert_url_eq(&client, &format!("/teams/{}/channels/{}", RID, ID));
}

#[test]
fn teams_schedule_request() {
    let client = get_graph();

    let _ = client.v1().team(RID).schedule().get_schedule();
    assert_url_eq(&client, &format!("/teams/{}/schedule", RID));

    let _ = client.v1().team(RID).schedule().share(&String::new());
    assert_url_eq(
        &client,
        &format!("/teams/{}/schedule/microsoft.graph.share", RID),
    );
}

#[test]
fn teams_primary_channel_request() {
    let client = get_graph();

    let _ = client
        .v1()
        .team(RID)
        .primary_channel()
        .get_primary_channel();
    assert_url_eq(&client, &format!("/teams/{}/primaryChannel", RID));

    let _ = client.v1().team(RID).primary_channel().tab(ID).get_tabs();
    assert_url_eq(
        &client,
        &format!("/teams/{}/primaryChannel/tabs/{}", RID, ID),
    );

    let _ = client
        .v1()
        .team(RID)
        .primary_channel()
        .shared_with_team(ID)
        .get_shared_with_teams();
    assert_url_eq(
        &client,
        &format!("/teams/{}/primaryChannel/sharedWithTeams/{}", RID, ID),
    );

    let _ = client
        .v1()
        .team(RID)
        .primary_channel()
        .shared_with_teams()
        .list_shared_with_teams();
    assert_url_eq(
        &client,
        &format!("/teams/{}/primaryChannel/sharedWithTeams", RID),
    );

    let _ = client
        .v1()
        .team(RID)
        .primary_channel()
        .messages()
        .list_messages();
    assert_url_eq(&client, &format!("/teams/{}/primaryChannel/messages", RID));

    let _ = client
        .v1()
        .team(RID)
        .primary_channel()
        .message(ID)
        .get_messages();
    assert_url_eq(
        &client,
        &format!("/teams/{}/primaryChannel/messages/{}", RID, ID),
    );
}
