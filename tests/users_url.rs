use graph_rs_sdk::prelude::*;
//use test_tools::{assert_url_beta_eq, assert_url_eq};

static USER_ID: &str = "b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI";

#[test]
fn list_users() {
    let client = Graph::new("");

    client.users().list_user();
    assert_eq!(
        client.users().list_user().url().path(),
        "/v1.0/users".to_string()
    );
}

#[test]
fn get_users() {
    let client = Graph::new("");
    assert_eq!(client.me().get_user().url().path(), "/v1.0/me");
    assert_eq!(
        client.user(USER_ID).get_user().url().path(),
        format!("/v1.0/users/{USER_ID}")
    );
}

#[test]
fn create_users() {
    let client = Graph::new("");
    assert_eq!(
        client.users().create_user(&String::new()).url().path(),
        "/v1.0/users"
    );
}

#[test]
fn delete_users() {
    let client = Graph::new("");
    assert_eq!(
        client.user(USER_ID).delete_user().url().path(),
        format!("/v1.0/users/{USER_ID}")
    );
}

#[test]
fn update_users() {
    let mut client = Graph::new("");

    assert_eq!(
        format!("/beta/users/{}", USER_ID),
        client
            .beta()
            .user(USER_ID)
            .update_user(&String::new())
            .url()
            .path()
    );

    client.use_v1();

    assert_eq!(
        format!("/v1.0/users/{}", USER_ID),
        client
            .user(USER_ID)
            .update_user(&String::new())
            .url()
            .path()
    );
}
