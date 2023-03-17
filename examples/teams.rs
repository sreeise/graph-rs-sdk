use graph_rs_sdk::prelude::*;

// This example shows using the teams api

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";
static TEAMS_ID: &str = "TEAMS_ID";

#[tokio::main]
async fn main() {
    list_teams().await;
    get_teams().await;
}

async fn list_teams() {
    let client = Graph::new(ACCESS_TOKEN);
    let response = client.teams().list_team().send().await.unwrap();

    println!("{:#?}", response);
}

async fn get_teams() {
    let client = Graph::new(ACCESS_TOKEN);
    let response = client.team(TEAMS_ID).get_team().send().await.unwrap();

    println!("{:#?}", response);
}
