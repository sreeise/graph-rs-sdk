use graph_rs_sdk::prelude::*;
use test_tools::{assert_url_beta_eq, assert_url_eq};

static USER_ID: &str = "b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI";

#[test]
fn list_users() {
    let client = Graph::new("");

    client.v1().users().list_user();
    assert_url_eq(&client, "/users");
}

#[test]
fn get_users() {
    let client = Graph::new("");

    client.v1().me().get_user();
    assert_url_eq(&client, "/me");

    client.v1().user(USER_ID).get_user();
    assert_url_eq(&client, format!("/users/{}", USER_ID));
}

#[test]
fn create_users() {
    let client = Graph::new("");

    client.v1().users().create_user(&String::new());
    assert_url_eq(&client, "/users");
}

#[test]
fn delete_users() {
    let client = Graph::new("");
    client.v1().user(USER_ID).delete_user();
    assert_url_eq(&client, format!("/users/{}", USER_ID));
}

#[test]
fn update_users() {
    let client = Graph::new("");

    client.beta().user(USER_ID).update_user(&String::new());
    assert_url_beta_eq(&client, format!("/users/{}", USER_ID));

    client.beta().user(USER_ID).update_user(&String::new());
    assert_url_beta_eq(&client, format!("/users/{}", USER_ID));
}
