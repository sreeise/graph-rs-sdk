#[macro_use]
extern crate lazy_static;

use graph_rs_sdk::prelude::*;
use test_tools::common::TestTools;

lazy_static! {
    static ref ID_VEC: Vec<String> = TestTools::random_strings(2, 20);
}

pub fn graph() -> GraphV2 {
    GraphV2::new("")
}

#[test]
fn app_catalogs_urls() {
    let client = graph();

    assert_eq!(
        "/v1.0/appCatalogs/teamsApps".to_string(),
        client.app_catalogs().list_teams_apps().url().path()
    );

    assert_eq!(
        format!("/v1.0/appCatalogs/teamsApps/{}", ID_VEC[0]),
        client
            .app_catalogs()
            .get_teams_apps(ID_VEC[0].as_str())
            .url()
            .path()
    );
}

#[test]
fn team_apps_urls() {
    let client = graph();

    assert_eq!(
        format!("/v1.0/appCatalogs/teamsApps/{}/appDefinitions", ID_VEC[0]),
        client
            .app_catalogs()
            .list_app_definitions(ID_VEC[0].as_str())
            .url()
            .path()
    );
}

#[test]
fn app_definition_urls() {
    let client = graph();

    assert_eq!(
        format!(
            "/v1.0/appCatalogs/teamsApps/{}/appDefinitions/{}/bot",
            ID_VEC[0], ID_VEC[1]
        ),
        client
            .app_catalogs()
            .get_bot(ID_VEC[0].as_str(), ID_VEC[1].as_str())
            .url()
            .path()
    );
}
