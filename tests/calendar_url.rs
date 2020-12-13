use graph_rs_sdk::prelude::*;
use test_tools::assert_url_eq;

#[test]
fn list_calendars() {
    let client = Graph::new("");
    client.v1().me().calendars().list_calendars();
    assert_url_eq(&client, "/me/calendars");
}

#[test]
fn get_calendars() {
    let client = Graph::new("");
    client.v1().me().calendars().get_calendar();
    assert_url_eq(&client, "/me/calendar");

    client.v1().user("32p99453").calendars().get_calendar();
    assert_url_eq(&client, "/users/32p99453/calendar");

    let client = Graph::new("");
    client.v1().me().calendar("2442").get_calendars();
    assert_url_eq(&client, "/me/calendars/2442");

    client
        .v1()
        .user("32p99453")
        .calendar("2442")
        .get_calendars();
    assert_url_eq(&client, "/users/32p99453/calendars/2442");
}

#[test]
fn update_calendars() {
    let client = Graph::new("");
    client.v1().me().calendars().update_calendar(&String::new());
    assert_url_eq(&client, "/me/calendar");

    client
        .v1()
        .user("32p99453")
        .calendars()
        .update_calendar(&String::new());
    assert_url_eq(&client, "/users/32p99453/calendar");

    client
        .v1()
        .user("32p99453")
        .calendar("2442")
        .update_calendars(&String::new());
    assert_url_eq(&client, "/users/32p99453/calendars/2442");
}

#[test]
fn create_calendars() {
    let client = Graph::new("");
    client
        .v1()
        .me()
        .calendars()
        .create_calendars(&serde_json::json!({}));
    assert_url_eq(&client, "/me/calendars");
}

#[test]
fn calendar_views() {
    let client = Graph::new("");
    client
        .v1()
        .user("32p99453")
        .calendar("1234")
        .calendar_views()
        .list_calendar_view()
        .query("startDateTime", "0")
        .query("endDateTime", "1");
    assert_url_eq(
        &client,
        "/users/32p99453/calendars/1234/calendarView?startDateTime=0&endDateTime=1",
    )
}

#[test]
fn calendar_events() {
    let client = Graph::new("");
    client.v1().me().calendars().events().list_events();
    assert_url_eq(&client, "/me/calendar/events");

    client
        .v1()
        .me()
        .calendars()
        .events()
        .create_events(&String::new());
    assert_url_eq(&client, "/me/calendar/events");

    client
        .v1()
        .me()
        .calendar_groups()
        .calendar("0")
        .events()
        .list_events();
    assert_url_eq(&client, "/me/calendarGroup/calendars/0/events");

    client
        .v1()
        .me()
        .calendar_group("0")
        .calendar("1")
        .events()
        .list_events();
    assert_url_eq(&client, "/me/calendarGroups/0/calendars/1/events");
}
