use graph_rs::prelude::*;
use test_tools::drive::*;

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
    client.v1().me().calendar().get();
    assert_url_eq(&client, "/me/calendar");

    client.v1().users("32p99453").calendar().get();
    assert_url_eq(&client, "/users/32p99453/calendar")
}

#[test]
fn update_calendars() {
    let client = Graph::new("");
    client.v1().me().calendar().update(&String::new());
    assert_url_eq(&client, "/me/calendars");

    client
        .v1()
        .users("32p99453")
        .calendar()
        .update(&String::new());
    assert_url_eq(&client, "/users/32p99453/calendars")
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
    client.v1().me().calendar().delete().by_id("1234");
    assert_url_eq(&client, "/me/calendars/1234");

    client
        .v1()
        .sites("32p99453")
        .calendar()
        .create(&String::new())
        .by_id("1234");
    assert_url_eq(&client, "/sites/32p99453/calendars/1234")
}
