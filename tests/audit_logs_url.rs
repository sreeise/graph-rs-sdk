#[macro_use]
extern crate lazy_static;

use graph_rs_sdk::prelude::*;
use test_tools::common::TestTools;

lazy_static! {
    static ref ID_VEC: Vec<String> = TestTools::random_strings(2, 20);
}

#[test]
fn audit_logs_url() {
    let client = Graph::new("");

    assert_eq!(
        "/v1.0/auditLogs".to_string(),
        client.audit_logs().get_audit_log_root().url().path()
    );

    assert_eq!(
        "/v1.0/auditLogs/signIns".to_string(),
        client.audit_logs().list_sign_ins().url().path()
    );
    assert_eq!(
        format!("/v1.0/auditLogs/signIns/{}", ID_VEC[0]),
        client
            .audit_logs()
            .update_sign_ins(ID_VEC[0].as_str(), &String::new())
            .url()
            .path()
    );
}
