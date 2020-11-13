use graph_rs::prelude::*;
use test_tools::assert_url_eq;

static RID: &str = "T5Y6RODPNfYICbtYWrofwUGBJWnaJkNwH9x";
static ID: &str = "b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI";

#[test]
fn default_calendar() {
    let client = Graph::new("");

    client
        .v1()
        .me()
        .attachments()
        .calendars()
        .get_default(ID, ID);
    assert_url_eq(&client, format!("/me/events/{}/attachments/{}", ID, ID));

    client
        .v1()
        .user(RID)
        .attachments()
        .calendars()
        .get_default(ID, ID);
    assert_url_eq(
        &client,
        format!("/users/{}/events/{}/attachments/{}", RID, ID, ID),
    );

    client
        .v1()
        .user(RID)
        .attachments()
        .calendars()
        .delete_default(ID, ID);
    assert_url_eq(
        &client,
        format!("/users/{}/events/{}/attachments/{}", RID, ID, ID),
    );

    client
        .v1()
        .user(RID)
        .attachments()
        .calendars()
        .default_content(ID, ID);
    assert_url_eq(
        &client,
        format!("/users/{}/events/{}/attachments/{}/$value", RID, ID, ID),
    );
}

#[test]
fn calendar() {
    let client = Graph::new("");

    client.v1().me().attachments().calendars().get(ID, ID, ID);
    assert_url_eq(
        &client,
        format!("/me/calendar/{}/events/{}/attachments/{}", ID, ID, ID),
    );

    client
        .v1()
        .user(RID)
        .attachments()
        .calendars()
        .get(ID, ID, ID);
    assert_url_eq(
        &client,
        format!(
            "/users/{}/calendar/{}/events/{}/attachments/{}",
            RID, ID, ID, ID
        ),
    );

    client
        .v1()
        .user(RID)
        .attachments()
        .calendars()
        .delete(ID, ID, ID);
    assert_url_eq(
        &client,
        format!(
            "/users/{}/calendar/{}/events/{}/attachments/{}",
            RID, ID, ID, ID
        ),
    );

    client
        .v1()
        .user(RID)
        .attachments()
        .calendars()
        .content(ID, ID, ID);
    assert_url_eq(
        &client,
        format!(
            "/users/{}/calendar/{}/events/{}/attachments/{}/$value",
            RID, ID, ID, ID
        ),
    );
}

#[test]
fn default_calendar_groups() {
    let client = Graph::new("");

    client
        .v1()
        .me()
        .attachments()
        .calendar_groups()
        .get_default(ID, ID, ID);
    assert_url_eq(
        &client,
        format!(
            "/me/calendargroup/calendars/{}/events/{}/attachments/{}",
            ID, ID, ID
        ),
    );

    client
        .v1()
        .user(RID)
        .attachments()
        .calendar_groups()
        .get_default(ID, ID, ID);
    assert_url_eq(
        &client,
        format!(
            "/users/{}/calendargroup/calendars/{}/events/{}/attachments/{}",
            RID, ID, ID, ID
        ),
    );

    client
        .v1()
        .user(RID)
        .attachments()
        .calendar_groups()
        .delete_default(ID, ID, ID);
    assert_url_eq(
        &client,
        format!(
            "/users/{}/calendargroup/calendars/{}/events/{}/attachments/{}",
            RID, ID, ID, ID
        ),
    );

    client
        .v1()
        .user(RID)
        .attachments()
        .calendar_groups()
        .default_content(ID, ID, ID);
    assert_url_eq(
        &client,
        format!(
            "/users/{}/calendargroup/calendars/{}/events/{}/attachments/{}/$value",
            RID, ID, ID, ID
        ),
    );
}

#[test]
fn calendar_groups() {
    let client = Graph::new("");

    client
        .v1()
        .me()
        .attachments()
        .calendar_groups()
        .get(ID, ID, ID, ID);
    assert_url_eq(
        &client,
        format!(
            "/me/calendargroups/{}/calendars/{}/events/{}/attachments/{}",
            ID, ID, ID, ID
        ),
    );

    client
        .v1()
        .user(RID)
        .attachments()
        .calendar_groups()
        .get(ID, ID, ID, ID);
    assert_url_eq(
        &client,
        format!(
            "/users/{}/calendargroups/{}/calendars/{}/events/{}/attachments/{}",
            RID, ID, ID, ID, ID
        ),
    );

    client
        .v1()
        .user(RID)
        .attachments()
        .calendar_groups()
        .delete(ID, ID, ID, ID);
    assert_url_eq(
        &client,
        format!(
            "/users/{}/calendargroups/{}/calendars/{}/events/{}/attachments/{}",
            RID, ID, ID, ID, ID
        ),
    );

    client
        .v1()
        .user(RID)
        .attachments()
        .calendar_groups()
        .content(ID, ID, ID, ID);
    assert_url_eq(
        &client,
        format!(
            "/users/{}/calendargroups/{}/calendars/{}/events/{}/attachments/{}/$value",
            RID, ID, ID, ID, ID
        ),
    );
}

#[test]
fn messages() {
    let client = Graph::new("");

    client.v1().me().attachments().messages().get(ID, ID);
    assert_url_eq(&client, format!("/me/messages/{}/attachments/{}", ID, ID));

    client.v1().me().mail().messages().attachments().get(ID, ID);
    assert_url_eq(&client, format!("/me/messages/{}/attachments/{}", ID, ID));

    client.v1().user(RID).attachments().messages().get(ID, ID);
    assert_url_eq(
        &client,
        format!("/users/{}/messages/{}/attachments/{}", RID, ID, ID),
    );

    client
        .v1()
        .user(RID)
        .attachments()
        .messages()
        .delete(ID, ID);
    assert_url_eq(
        &client,
        format!("/users/{}/messages/{}/attachments/{}", RID, ID, ID),
    );

    client
        .v1()
        .user(RID)
        .attachments()
        .messages()
        .content(ID, ID);
    assert_url_eq(
        &client,
        format!("/users/{}/messages/{}/attachments/{}/$value", RID, ID, ID),
    );
}

#[test]
fn mail_folder() {
    let client = Graph::new("");

    client.v1().me().attachments().mail_folder().get(ID, ID, ID);
    assert_url_eq(
        &client,
        format!("/me/mailFolders/{}/messages/{}/attachments/{}", ID, ID, ID),
    );

    client
        .v1()
        .me()
        .mail()
        .mail_folder()
        .attachments()
        .get(ID, ID, ID);
    assert_url_eq(
        &client,
        format!("/me/mailFolders/{}/messages/{}/attachments/{}", ID, ID, ID),
    );

    client
        .v1()
        .user(RID)
        .attachments()
        .mail_folder()
        .get(ID, ID, ID);
    assert_url_eq(
        &client,
        format!(
            "/users/{}/mailFolders/{}/messages/{}/attachments/{}",
            RID, ID, ID, ID
        ),
    );

    client
        .v1()
        .user(RID)
        .attachments()
        .mail_folder()
        .delete(ID, ID, ID);
    assert_url_eq(
        &client,
        format!(
            "/users/{}/mailFolders/{}/messages/{}/attachments/{}",
            RID, ID, ID, ID
        ),
    );

    client
        .v1()
        .user(RID)
        .attachments()
        .mail_folder()
        .content(ID, ID, ID);
    assert_url_eq(
        &client,
        format!(
            "/users/{}/mailFolders/{}/messages/{}/attachments/{}/$value",
            RID, ID, ID, ID
        ),
    );

    client
        .v1()
        .user(RID)
        .attachments()
        .mail_folder()
        .child_folder(ID, &vec![ID, ID], ID, ID);
    assert_url_eq(
        &client,
        format!(
            "/users/{}/mailFolders/{}/childFolders/{}/childFolders/{}/messages/{}/attachments/{}",
            RID, ID, ID, ID, ID, ID
        ),
    );

    client
        .v1()
        .user(RID)
        .attachments()
        .mail_folder()
        .child_folder_content(ID, &vec![ID, ID, ID], ID, ID);
    assert_url_eq(
        &client,
        format!(
            "/users/{}/mailFolders/{}/childFolders/{}/childFolders/{}/childFolders/{}/messages/{}/attachments/{}/$value",
            RID, ID, ID, ID, ID, ID, ID
        ),
    );
}

#[test]
fn group_thread_and_conversation() {
    let client = Graph::new("");

    client.v1().me().attachments().thread_posts().list(ID, ID);
    assert_url_eq(
        &client,
        format!("/me/threads/{}/posts/{}/attachments", ID, ID),
    );

    client
        .v1()
        .me()
        .attachments()
        .conversation_posts()
        .list(ID, ID, ID);
    assert_url_eq(
        &client,
        format!(
            "/me/conversations/{}/threads/{}/posts/{}/attachments",
            ID, ID, ID
        ),
    );

    client
        .v1()
        .groups(RID)
        .thread_posts()
        .attachments()
        .list(ID, ID);
    assert_url_eq(
        &client,
        format!("/groups/{}/threads/{}/posts/{}/attachments", RID, ID, ID),
    );

    client
        .v1()
        .groups(RID)
        .conversation_posts()
        .attachments()
        .list(ID, ID, ID);
    assert_url_eq(
        &client,
        format!(
            "/groups/{}/conversations/{}/threads/{}/posts/{}/attachments",
            RID, ID, ID, ID
        ),
    );

    client
        .v1()
        .groups(RID)
        .thread_posts()
        .attachments()
        .get(ID, ID, ID);
    assert_url_eq(
        &client,
        format!(
            "/groups/{}/threads/{}/posts/{}/attachments/{}",
            RID, ID, ID, ID
        ),
    );

    client
        .v1()
        .groups(RID)
        .conversation_posts()
        .attachments()
        .get(ID, ID, ID, ID);
    assert_url_eq(
        &client,
        format!(
            "/groups/{}/conversations/{}/threads/{}/posts/{}/attachments/{}",
            RID, ID, ID, ID, ID
        ),
    );

    client
        .v1()
        .groups(RID)
        .thread_posts()
        .attachments()
        .delete(ID, ID, ID);
    assert_url_eq(
        &client,
        format!(
            "/groups/{}/threads/{}/posts/{}/attachments/{}",
            RID, ID, ID, ID
        ),
    );

    client
        .v1()
        .groups(RID)
        .conversation_posts()
        .attachments()
        .delete(ID, ID, ID, ID);
    assert_url_eq(
        &client,
        format!(
            "/groups/{}/conversations/{}/threads/{}/posts/{}/attachments/{}",
            RID, ID, ID, ID, ID
        ),
    );

    client
        .v1()
        .groups(RID)
        .thread_posts()
        .attachments()
        .content(ID, ID, ID);
    assert_url_eq(
        &client,
        format!(
            "/groups/{}/threads/{}/posts/{}/attachments/{}/$value",
            RID, ID, ID, ID
        ),
    );

    client
        .v1()
        .groups(RID)
        .conversation_posts()
        .attachments()
        .content(ID, ID, ID, ID);
    assert_url_eq(
        &client,
        format!(
            "/groups/{}/conversations/{}/threads/{}/posts/{}/attachments/{}/$value",
            RID, ID, ID, ID, ID
        ),
    );
}
