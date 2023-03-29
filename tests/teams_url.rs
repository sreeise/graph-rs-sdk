use graph_rs_sdk::prelude::*;

static RID: &str = "T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x";
static ID: &str = "b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI";

#[test]
fn get_teams_request() {
    let client = Graph::new("");

    assert_eq!(
        format!("/v1.0/teams/{RID}"),
        client.teams().id(RID).get_team().url().path()
    );
    assert_eq!(
        format!("/v1.0/teams/{RID}"),
        client.team(RID).get_team().url().path()
    );
}

#[test]
fn teams_channel_request() {
    let client = Graph::new("");

    assert_eq!(
        format!("/v1.0/teams/{RID}/channels"),
        client.team(RID).channels().list_channels().url().path()
    );
    assert_eq!(
        format!("/v1.0/teams/{RID}/channels/{ID}"),
        client.team(RID).channel(ID).get_channels().url().path()
    );
}

#[test]
fn teams_primary_channel_request() {
    let client = Graph::new("");

    assert_eq!(
        format!("/v1.0/teams/{}/primaryChannel", RID),
        client.team(RID).primary_channel().get_primary_channel().url().path()
    );

    assert_eq!(
        format!("/v1.0/teams/{}/primaryChannel/tabs/{}", RID, ID),
        client.team(RID).primary_channel().get_tabs(ID).url().path()
    );

    assert_eq!(
        format!("/v1.0/teams/{}/primaryChannel/sharedWithTeams/{}", RID, ID),
        client.team(RID)
            .primary_channel()
            .shared_with_team(ID)
            .get_shared_with_teams()
            .url().path()
    );

    assert_eq!(
        format!("/v1.0/teams/{}/primaryChannel/sharedWithTeams", RID),
        client.team(RID)
            .primary_channel()
            .shared_with_teams()
            .list_shared_with_teams()
            .url().path()
    );

    assert_eq!(
        format!("/v1.0/teams/{}/primaryChannel/messages", RID),
        client.team(RID)
            .primary_channel()
            .messages()
            .list_messages()
            .url().path()
    );

    assert_eq!(
        format!("/v1.0/teams/{}/primaryChannel/messages/{}", RID, ID),
        client.team(RID)
            .primary_channel()
            .message(ID)
            .get_messages()
            .url().path()
    );
}
