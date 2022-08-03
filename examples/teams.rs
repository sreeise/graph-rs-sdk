use graph_rs_sdk::prelude::*;

// This example shows using the teams api

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";
static TEAMS_ID: &str = "TEAMS_ID";

fn main() {
    list_teams();
    get_teams();
}

fn list_teams() {
    let client = Graph::new(ACCESS_TOKEN);
    let response = client.v1().teams().list_team().send().unwrap();

    println!("{:#?}", response);
}

fn get_teams() {
    let client = Graph::new(ACCESS_TOKEN);
    let response = client.v1().team(TEAMS_ID).get_team().send().unwrap();

    println!("{:#?}", response);
}
