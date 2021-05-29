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
fn audit_logs_url() {
    let client = graph();

    client.v1().audit_logs().get_audit_log_root();
    assert_url_eq(&client, "/auditLogs");

    client.v1().audit_logs().list_sign_ins();
    assert_url_eq(&client, "/auditLogs/signIns");

    client
        .v1()
        .audit_logs()
        .update_sign_ins(ID_VEC[0].as_str(), &String::new());
    assert_url_eq(&client, &format!("/auditLogs/signIns/{}", ID_VEC[0]));
}
