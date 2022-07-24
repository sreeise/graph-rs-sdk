use graph_http::BlockingHttpClient;
use graph_rs_sdk::prelude::*;
use test_tools::assert_url_eq;

static RID: &str = "T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x";
static ID: &str = "b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI";

fn get_graph() -> Graph<BlockingHttpClient> {
    Graph::new("")
}

#[test]
fn teams_channel_request() {
    let client = get_graph();

    let _ = client.v1().team(RID).channels().list_channels();
    assert_url_eq(&client, &format!("/teams/{}/channels", RID));

    let _ = client.v1().team(RID).channel(ID).tab(ID).get_tabs();
    assert_url_eq(
        &client,
        &format!("/teams/{}/channels/{}/tabs/{}", RID, ID, ID),
    );

    let _ = client
        .v1()
        .team(RID)
        .channel(ID)
        .shared_with_team(ID)
        .get_shared_with_teams();
    assert_url_eq(
        &client,
        &format!("/teams/{}/channels/{}/sharedWithTeams/{}", RID, ID, ID),
    );

    let _ = client
        .v1()
        .team(RID)
        .channel(ID)
        .shared_with_teams()
        .list_shared_with_teams();
    assert_url_eq(
        &client,
        &format!("/teams/{}/channels/{}/sharedWithTeams", RID, ID),
    );

    let _ = client.v1().team(RID).channel(ID).messages().list_messages();
    assert_url_eq(&client, &format!("/teams/{}/channels/{}/messages", RID, ID));

    let _ = client.v1().team(RID).channel(ID).message(ID).get_messages();
    assert_url_eq(
        &client,
        &format!("/teams/{}/channels/{}/messages/{}", RID, ID, ID),
    );
}
