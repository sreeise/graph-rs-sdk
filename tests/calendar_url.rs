use graph_rs::prelude::*;
use test_tools::assert_url_eq;

#[test]
fn list_calendars() {
    let client = Graph::new("");
    client.v1().me().calendar().list();
    assert_url_eq(&client, "/me/calendars");

    client.v1().sites("32p99453").calendar().list();
    assert_url_eq(&client, "/sites/32p99453/calendars")
}

#[test]
fn get_calendars() {
    let client = Graph::new("");
    client.v1().me().calendar().get_default();
    assert_url_eq(&client, "/me/calendar");

    client.v1().users("32p99453").calendar().get_default();
    assert_url_eq(&client, "/users/32p99453/calendar");

    let client = Graph::new("");
    client.v1().me().calendar().get("2442");
    assert_url_eq(&client, "/me/calendars/2442");

    client.v1().users("32p99453").calendar().get("2442");
    assert_url_eq(&client, "/users/32p99453/calendars/2442");
}

#[test]
fn update_calendars() {
    let client = Graph::new("");
    client.v1().me().calendar().update_default(&String::new());
    assert_url_eq(&client, "/me/calendar");

    client
        .v1()
        .users("32p99453")
        .calendar()
        .update_default(&String::new());
    assert_url_eq(&client, "/users/32p99453/calendar");

    client
        .v1()
        .users("32p99453")
        .calendar()
        .update("2442", &String::new());
    assert_url_eq(&client, "/users/32p99453/calendars/2442");
}

#[test]
fn create_calendars() {
    let client = Graph::new("");
    client.v1().me().calendar().create(&String::new());
    assert_url_eq(&client, "/me/calendars");

    client
        .v1()
        .sites("32p99453")
        .calendar()
        .create(&String::new());
    assert_url_eq(&client, "/sites/32p99453/calendars")
}

#[test]
fn delete_calendars() {
    let client = Graph::new("");
    client.v1().me().calendar().delete("1234");
    assert_url_eq(&client, "/me/calendars/1234");

    client.v1().sites("32p99453").calendar().delete("1234");
    assert_url_eq(&client, "/sites/32p99453/calendars/1234")
}

#[test]
fn calendar_views() {
    let client = Graph::new("");
    client.v1().me().calendar().views().list_default_view("0", "1");
    assert_url_eq(&client, "/me/calendar/calendarView?startDateTime=0&endDateTime=1");

    client.v1().sites("32p99453").calendar().views().list_view("1234", "0", "1");
    assert_url_eq(&client, "/sites/32p99453/calendars/1234/calendarView?startDateTime=0&endDateTime=1")
}