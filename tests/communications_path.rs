#[macro_use]
extern crate lazy_static;

use graph_rs_sdk::*;
use test_tools::common::TestTools;

lazy_static! {
    static ref ID_VEC: Vec<String> = TestTools::random_strings(4, 20);
}

#[test]
fn communications_calls() {
    let client = Graph::new("");

    assert_eq!(
        "/v1.0/communications/calls".to_string(),
        client.communications().calls().list_calls().url().path()
    );

    assert_eq!(
        "/v1.0/communications/calls/logTeleconferenceDeviceQuality".to_string(),
        client
            .communications()
            .calls()
            .log_teleconference_device_quality(&String::new())
            .url()
            .path()
    );

    assert_eq!(
        format!("/v1.0/communications/calls/{}", ID_VEC[0]),
        client
            .communications()
            .call(ID_VEC[0].as_str())
            .get_calls()
            .url()
            .path()
    );

    assert_eq!(
        format!("/v1.0/communications/calls/{}", ID_VEC[0]),
        client
            .communications()
            .call(ID_VEC[0].as_str())
            .update_calls(&String::new())
            .url()
            .path()
    );
}

#[test]
fn communications_call_records() {
    let client = Graph::new("");

    assert_eq!(
        "/v1.0/communications/callRecords".to_string(),
        client
            .communications()
            .call_records()
            .list_call_records()
            .url()
            .path()
    );

    assert_eq!(
        format!("/v1.0/communications/callRecords/{}", ID_VEC[0]),
        client
            .communications()
            .call_record(ID_VEC[0].as_str())
            .get_call_records()
            .url()
            .path()
    );
}

#[test]
fn communications_call_record_sessions() {
    let client = Graph::new("");

    assert_eq!(
        format!("/v1.0/communications/callRecords/{}/sessions", ID_VEC[0]),
        client
            .communications()
            .call_record(ID_VEC[0].as_str())
            .sessions()
            .list_sessions()
            .url()
            .path()
    );

    assert_eq!(
        format!(
            "/v1.0/communications/callRecords/{}/sessions/{}",
            ID_VEC[0], ID_VEC[1]
        ),
        client
            .communications()
            .call_record(ID_VEC[0].as_str())
            .session(ID_VEC[1].as_str())
            .get_sessions()
            .url()
            .path()
    );

    assert_eq!(
        format!(
            "/v1.0/communications/callRecords/{}/sessions/{}/segments/{}",
            ID_VEC[0], ID_VEC[1], ID_VEC[2]
        ),
        client
            .communications()
            .call_record(ID_VEC[0].as_str())
            .session(ID_VEC[1].as_str())
            .get_segments(ID_VEC[2].as_str())
            .url()
            .path()
    );
}
