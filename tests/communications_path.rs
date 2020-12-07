#[macro_use]
extern crate lazy_static;

use graph_http::BlockingHttpClient;
use graph_rs::prelude::*;
use test_tools::assert_url_eq;
use test_tools::common::TestTools;

lazy_static! {
    static ref ID_VEC: Vec<String> = TestTools::random_strings(4, 20);
}

pub fn graph() -> Graph<BlockingHttpClient> {
    Graph::new("")
}

#[test]
fn communications_calls() {
    let client = graph();

    client.v1().communications().calls().list_calls();

    assert_url_eq(&client, "/communications/calls");

    client
        .v1()
        .communications()
        .calls()
        .log_teleconference_device_quality(&String::new());

    assert_url_eq(
        &client,
        "/communications/calls/logTeleconferenceDeviceQuality",
    );

    client
        .v1()
        .communications()
        .call(ID_VEC[0].as_str())
        .get_calls();

    assert_url_eq(&client, &format!("/communications/calls/{}", ID_VEC[0]));

    client
        .v1()
        .communications()
        .call(ID_VEC[0].as_str())
        .update_calls(&String::new());

    assert_url_eq(&client, &format!("/communications/calls/{}", ID_VEC[0]));
}

#[test]
fn communications_call_records() {
    let client = graph();

    client.v1().communications().list_call_records();

    assert_url_eq(&client, "/communications/callRecords");

    client
        .v1()
        .communications()
        .get_call_records(ID_VEC[0].as_str());

    assert_url_eq(
        &client,
        &format!("/communications/callRecords/{}", ID_VEC[0]),
    );

    client
        .v1()
        .communications()
        .call_records()
        .list_call_records();

    assert_url_eq(&client, "/communications/callRecords");

    client
        .v1()
        .communications()
        .call_record(ID_VEC[0].as_str())
        .get_call_records();

    assert_url_eq(
        &client,
        &format!("/communications/callRecords/{}", ID_VEC[0]),
    );
}

#[test]
fn communications_call_record_sessions() {
    let client = graph();

    client
        .v1()
        .communications()
        .call_record(ID_VEC[0].as_str())
        .sessions()
        .list_sessions();

    assert_url_eq(
        &client,
        &format!("/communications/callRecords/{}/sessions", ID_VEC[0]),
    );

    client
        .v1()
        .communications()
        .call_record(ID_VEC[0].as_str())
        .session(ID_VEC[1].as_str())
        .get_sessions();

    assert_url_eq(
        &client,
        &format!(
            "/communications/callRecords/{}/sessions/{}",
            ID_VEC[0], ID_VEC[1]
        ),
    );

    client
        .v1()
        .communications()
        .call_record(ID_VEC[0].as_str())
        .session(ID_VEC[1].as_str())
        .get_segments(ID_VEC[2].as_str());

    assert_url_eq(
        &client,
        &format!(
            "/communications/callRecords/{}/sessions/{}/segments/{}",
            ID_VEC[0], ID_VEC[1], ID_VEC[2]
        ),
    );
}
