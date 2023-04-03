use graph_rs_sdk::*;

static RID: &str = "T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x";
static ID: &str = "b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI";

#[test]
fn teams_channel_request() {
    let client = Graph::new("");

    assert_eq!(
        format!("/v1.0/teams/{RID}/channels"),
        client.team(RID).channels().list_channels().url().path()
    );

    assert_eq!(
        format!("/v1.0/teams/{RID}/channels/{ID}/tabs/{ID}"),
        client.team(RID).channel(ID).get_tabs(ID).url().path()
    );

    assert_eq!(
        format!("/v1.0/teams/{RID}/channels/{ID}/sharedWithTeams/{ID}"),
        client
            .team(RID)
            .channel(ID)
            .shared_with_team(ID)
            .get_shared_with_teams()
            .url()
            .path()
    );

    assert_eq!(
        format!("/v1.0/teams/{RID}/channels/{ID}/sharedWithTeams"),
        client
            .team(RID)
            .channel(ID)
            .shared_with_teams()
            .list_shared_with_teams()
            .url()
            .path()
    );

    assert_eq!(
        format!("/v1.0/teams/{RID}/channels/{ID}/messages"),
        client
            .team(RID)
            .channel(ID)
            .messages()
            .list_messages()
            .url()
            .path()
    );

    assert_eq!(
        format!("/v1.0/teams/{RID}/channels/{ID}/messages/{ID}"),
        client
            .team(RID)
            .channel(ID)
            .message(ID)
            .get_messages()
            .url()
            .path()
    );
}
