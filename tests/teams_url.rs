use graph_http::BlockingHttpClient;
use graph_rs::prelude::*;
use test_tools::assert_url_eq;

static RID: &str = "T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x";
static ID: &str = "b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI";

fn get_graph() -> Graph<BlockingHttpClient> {
    Graph::new("")
}

#[test]
fn team_to_teams() {
    let client = get_graph();

    let _ = client.v1().teams().id(RID).get_channels(ID);
    assert_url_eq(&client, &format!("/teams/{}/channels/{}", RID, ID));
}

#[test]
fn team_request() {
    let client = get_graph();

    let _ = client.v1().teams().list_team();
    assert_url_eq(&client, "/teams");
}

#[test]
fn teams_channel_request() {
    let client = get_graph();

    let _ = client.v1().team(RID).list_channels();
    assert_url_eq(&client, &format!("/teams/{}/channels", RID));

    let _ = client.v1().team(RID).get_channels(ID);
    assert_url_eq(&client, &format!("/teams/{}/channels/{}", RID, ID));
}
