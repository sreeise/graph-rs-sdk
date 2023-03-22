#[macro_use]
extern crate lazy_static;

use graph_rs_sdk::prelude::*;
use test_tools::common::TestTools;

lazy_static! {
    static ref ID_VEC: Vec<String> = TestTools::random_strings(4, 20);
}

#[test]
fn calendar_view_default() {
    let client = Graph::new("");

    assert_eq!(
        format!("/v1.0/me/calendarView/{}/accept", ID_VEC[0]),
        client
            .me()
            .calendar_view(ID_VEC[0].as_str())
            .accept(&String::new())
            .url()
            .path()
    );

    assert_eq!(
        format!("/v1.0/me/calendarView/{}/snoozeReminder", ID_VEC[0]),
        client
            .me()
            .calendar_view(ID_VEC[0].as_str())
            .snooze_reminder(&String::new())
            .url()
            .path()
    );
}

#[test]
fn calendar_view_instances() {
    let client = Graph::new("");

    assert_eq!(
        format!("/v1.0/me/calendarView/{}/instances/$count", ID_VEC[0]),
        client
            .me()
            .calendar_view(ID_VEC[0].as_str())
            .instances()
            .get_instances_count()
            .url()
            .path()
    );

    assert_eq!(
        format!(
            "/v1.0/me/calendarView/{}/instances/{}/tentativelyAccept",
            ID_VEC[0], ID_VEC[1]
        ),
        client
            .me()
            .calendar_view(ID_VEC[0].as_str())
            .instance(ID_VEC[1].as_str())
            .tentatively_accept(&String::new())
            .url()
            .path()
    );
}

#[test]
fn calendar_group_calendar_view() {
    let client = Graph::new("");

    assert_eq!(
        format!(
            "/v1.0/me/calendarGroups/{}/calendars/{}/calendarView",
            ID_VEC[0], ID_VEC[1]
        ),
        client
            .me()
            .calendar_group(ID_VEC[0].as_str())
            .calendar(ID_VEC[1].as_str())
            .calendar_views()
            .list_calendar_view()
            .url()
            .path()
    );

    assert_eq!(
        format!(
            "/v1.0/me/calendarGroups/{}/calendars/{}/calendarView/{}/attachments/{}",
            ID_VEC[0], ID_VEC[1], ID_VEC[2], ID_VEC[3]
        ),
        client
            .me()
            .calendar_group(ID_VEC[0].as_str())
            .calendar(ID_VEC[1].as_str())
            .calendar_view(ID_VEC[2].as_str())
            .attachment(ID_VEC[3].as_str())
            .get_attachments()
            .url()
            .path()
    );
}

#[test]
fn attachments_upload_session() {
    let client = Graph::new("");

    assert_eq!(
        format!(
            "/v1.0/me/calendarView/{}/attachments/createUploadSession",
            ID_VEC[0]
        ),
        client
            .me()
            .calendar_view(ID_VEC[0].as_str())
            .attachments()
            .create_upload_session(&String::new())
            .url()
            .path()
    );
}
