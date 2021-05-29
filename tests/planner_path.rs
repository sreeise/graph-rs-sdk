#[macro_use]
extern crate lazy_static;

use graph_rs_sdk::prelude::*;
use test_tools::{assert_url_eq, common::TestTools};

lazy_static! {
    static ref ID_VEC: Vec<String> = TestTools::random_strings(4, 20);
}

#[test]
fn planner_get_plan() {
    let client = Graph::new("");

    let _ = client.v1().planner().plan(ID_VEC[0].as_str()).get_plans();

    assert_url_eq(&client, format!("/planner/plans/{}", ID_VEC[0]));
}

#[test]
fn me_get_plan() {
    let client = Graph::new("");

    let _ = client
        .v1()
        .me()
        .planner()
        .plan(ID_VEC[0].as_str())
        .get_plans();

    assert_url_eq(&client, format!("/me/planner/plans/{}", ID_VEC[0]));
}

#[test]
fn user_get_plan() {
    let client = Graph::new("");

    let _ = client
        .v1()
        .user(ID_VEC[0].as_str())
        .planner()
        .plan(ID_VEC[1].as_str())
        .get_plans();

    assert_url_eq(
        &client,
        format!("/users/{}/planner/plans/{}", ID_VEC[0], ID_VEC[1]),
    );
}

#[test]
fn groups_get_plan() {
    let client = Graph::new("");

    let _ = client
        .v1()
        .group(ID_VEC[0].as_str())
        .planner()
        .plan(ID_VEC[1].as_str())
        .get_plans();

    assert_url_eq(
        &client,
        format!("/groups/{}/planner/plans/{}", ID_VEC[0], ID_VEC[1]),
    );
}
