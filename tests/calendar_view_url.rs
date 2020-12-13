#[macro_use]
extern crate lazy_static;

use graph_http::BlockingHttpClient;
use graph_rs_sdk::prelude::*;
use test_tools::assert_url_eq;
use test_tools::common::TestTools;

lazy_static! {
    static ref ID_VEC: Vec<String> = TestTools::random_strings(4, 20);
}

pub fn graph() -> Graph<BlockingHttpClient> {
    Graph::new("")
}

#[test]
fn calendar_view_default() {
    let client = graph();

    client
        .v1()
        .me()
        .calendar_view(ID_VEC[0].as_str())
        .accept(&String::new());
    assert_url_eq(&client, &format!("/me/calendarView/{}/accept", ID_VEC[0]));

    client
        .v1()
        .me()
        .calendar_view(ID_VEC[0].as_str())
        .snooze_reminder(&String::new());
    assert_url_eq(
        &client,
        &format!("/me/calendarView/{}/snoozeReminder", ID_VEC[0]),
    );
}

#[test]
fn calendar_view_calendar_events() {
    let client = graph();

    client
        .v1()
        .me()
        .calendar_view(ID_VEC[0].as_str())
        .calendar()
        .event(ID_VEC[1].as_str())
        .get_events();
    assert_url_eq(
        &client,
        &format!(
            "/me/calendarView/{}/calendar/events/{}",
            ID_VEC[0], ID_VEC[1]
        ),
    );

    let client = graph();

    client
        .v1()
        .me()
        .calendar_view(ID_VEC[0].as_str())
        .calendar()
        .event(ID_VEC[1].as_str())
        .tentatively_accept(&String::new());
    assert_url_eq(
        &client,
        &format!(
            "/me/calendarView/{}/calendar/events/{}/tentativelyAccept",
            ID_VEC[0], ID_VEC[1]
        ),
    );
}

#[test]
fn calendar_group_calendar_view() {
    let client = graph();

    client
        .v1()
        .me()
        .calendar_group(ID_VEC[0].as_str())
        .calendar(ID_VEC[1].as_str())
        .calendar_views()
        .list_calendar_view();
    assert_url_eq(
        &client,
        &format!(
            "/me/calendarGroups/{}/calendars/{}/calendarView",
            ID_VEC[0], ID_VEC[1]
        ),
    );

    let client = graph();

    client
        .v1()
        .me()
        .calendar_group(ID_VEC[0].as_str())
        .calendar(ID_VEC[1].as_str())
        .calendar_view(ID_VEC[2].as_str())
        .attachment(ID_VEC[3].as_str())
        .get_attachments();
    assert_url_eq(
        &client,
        &format!(
            "/me/calendarGroups/{}/calendars/{}/calendarView/{}/attachments/{}",
            ID_VEC[0], ID_VEC[1], ID_VEC[2], ID_VEC[3]
        ),
    );
}

#[test]
fn attachments_upload_session() {
    let client = graph();

    client
        .v1()
        .me()
        .calendar_view(ID_VEC[0].as_str())
        .attachments()
        .create_upload_session("./test_files/test_upload_file.txt", &String::new());

    assert_url_eq(
        &client,
        &format!(
            "/me/calendarView/{}/attachments/createUploadSession",
            ID_VEC[0]
        ),
    );
}
