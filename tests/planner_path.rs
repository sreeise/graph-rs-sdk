#[macro_use]
extern crate lazy_static;

use graph_rs_sdk::*;
use test_tools::common::TestTools;

lazy_static! {
    static ref ID_VEC: Vec<String> = TestTools::random_strings(4, 20);
}

#[test]
fn planner_get_plan() {
    let client = Graph::new("");

    assert_eq!(
        format!("/v1.0/planner/plans/{}", ID_VEC[0]),
        client
            .planner()
            .plan(ID_VEC[0].as_str())
            .get_plans()
            .url()
            .path()
    );
}

#[test]
fn user_get_plan() {
    let client = Graph::new("");

    assert_eq!(
        format!("/v1.0/users/{}/planner/plans/{}", ID_VEC[0], ID_VEC[1]),
        client
            .user(ID_VEC[0].as_str())
            .planner()
            .plan(ID_VEC[1].as_str())
            .get_plans()
            .url()
            .path()
    );
}

#[test]
fn groups_get_plan() {
    let client = Graph::new("");

    assert_eq!(
        format!("/v1.0/groups/{}/planner/plans/{}", ID_VEC[0], ID_VEC[1]),
        client
            .group(ID_VEC[0].as_str())
            .planner()
            .plan(ID_VEC[1].as_str())
            .get_plans()
            .url()
            .path()
    );
}
