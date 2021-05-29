#[macro_use]
extern crate lazy_static;

use graph_http::BlockingHttpClient;
use graph_rs_sdk::prelude::*;
use test_tools::{assert_url_eq, common::TestTools};

lazy_static! {
    static ref ID_VEC: Vec<String> = TestTools::random_strings(2, 20);
}

pub fn graph() -> Graph<BlockingHttpClient> {
    Graph::new("")
}

#[test]
fn app_catalogs_urls() {
    let client = graph();

    client.v1().app_catalogs().list_teams_apps();
    assert_url_eq(&client, "/appCatalogs/teamsApps");

    client
        .v1()
        .app_catalogs()
        .get_teams_apps(ID_VEC[0].as_str());
    assert_url_eq(&client, &format!("/appCatalogs/teamsApps/{}", ID_VEC[0]));
}

#[test]
fn team_apps_urls() {
    let client = graph();
    client
        .v1()
        .app_catalogs()
        .teams_apps()
        .list_app_definitions(ID_VEC[0].as_str());
    assert_url_eq(
        &client,
        &format!("/appCatalogs/teamsApps/{}/appDefinitions", ID_VEC[0]),
    );
}

#[test]
fn app_definition_urls() {
    let client = graph();

    client
        .v1()
        .app_catalogs()
        .teams_apps()
        .app_definitions()
        .get_bot(ID_VEC[0].as_str(), ID_VEC[1].as_str());

    assert_url_eq(
        &client,
        &format!(
            "/appCatalogs/teamsApps/{}/appDefinitions/{}/bot",
            ID_VEC[0], ID_VEC[1]
        ),
    );
}
