use graph_rs_sdk::prelude::*;

#[test]
fn list_calendars() {
    let client = Graph::new("");
    assert_eq!(
        "/v1.0/me/calendars".to_string(),
        client.me().calendars().list_calendars().url().path()
    );
}

#[test]
fn get_default_calendar() {
    let client = Graph::new("");

    assert_eq!(
        "/v1.0/me/calendar".to_string(),
        client.me().default_calendar().get_calendar().url().path()
    );

    assert_eq!(
        "/v1.0/users/32p99453/calendar".to_string(),
        client
            .user("32p99453")
            .default_calendar()
            .get_calendar()
            .url()
            .path()
    );
}

#[test]
fn get_calendars() {
    let client = Graph::new("");

    assert_eq!(
        "/v1.0/me/calendar".to_string(),
        client.me().default_calendar().get_calendar().url().path()
    );

    assert_eq!(
        "/v1.0/users/32p99453/calendar".to_string(),
        client
            .user("32p99453")
            .default_calendar()
            .get_calendar()
            .url()
            .path()
    );

    assert_eq!(
        "/v1.0/me/calendars/2442".to_string(),
        client.me().calendar("2442").get_calendars().url().path()
    );

    assert_eq!(
        "/v1.0/users/32p99453/calendars/2442".to_string(),
        client
            .user("32p99453")
            .calendar("2442")
            .get_calendars()
            .url()
            .path()
    );
}

#[test]
fn update_calendars() {
    let client = Graph::new("");

    assert_eq!(
        "/v1.0/me/calendar".to_string(),
        client
            .me()
            .default_calendar()
            .update_calendar(&String::new())
            .url()
            .path()
    );

    assert_eq!(
        "/v1.0/users/32p99453/calendars/2442".to_string(),
        client
            .user("32p99453")
            .calendar("2442")
            .update_calendars(&String::new())
            .url()
            .path()
    );
}

#[test]
fn create_calendars() {
    let client = Graph::new("");

    assert_eq!(
        "/v1.0/me/calendars".to_string(),
        client
            .me()
            .calendars()
            .create_calendars(&String::new())
            .url()
            .path()
    );
}

#[test]
fn calendar_events() {
    let client = Graph::new("");

    assert_eq!(
        "/v1.0/me/calendar/events".to_string(),
        client
            .me()
            .default_calendar()
            .events()
            .list_events()
            .url()
            .path()
    );

    assert_eq!(
        "/v1.0/me/calendar/events".to_string(),
        client
            .me()
            .default_calendar()
            .events()
            .create_events(&String::new())
            .url()
            .path()
    );

    assert_eq!(
        "/v1.0/me/calendarGroups/0/calendars/1/events".to_string(),
        client
            .me()
            .calendar_group("0")
            .calendar("1")
            .events()
            .list_events()
            .url()
            .path()
    );
}
