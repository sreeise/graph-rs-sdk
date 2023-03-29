use graph_rs_sdk::prelude::*;

// This example shows using the teams api

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";
static TEAMS_ID: &str = "TEAMS_ID";

// List teams may not be supported on v1.0 endpoint but is supported on beta.
pub async fn list_teams() {
    let mut client = Graph::new(ACCESS_TOKEN);
    let response = client.beta().teams().list_team().send().await.unwrap();

    println!("{response:#?}");
}

pub async fn get_teams() {
    let client = Graph::new(ACCESS_TOKEN);
    let response = client.team(TEAMS_ID).get_team().send().await.unwrap();

    println!("{response:#?}");
}
