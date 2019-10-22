use graph_rs::prelude::*;
use test_tools::{assert_url_beta_eq, assert_url_eq};

static USER_ID: &str = "b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI";

#[test]
fn list_users() {
    let client = Graph::new("");

    client.v1().users("").list();
    assert_url_eq(&client, "/users");
}

#[test]
fn get_users() {
    let client = Graph::new("");

    client.v1().me().get();
    assert_url_eq(&client, "/me");

    client.v1().users(USER_ID).get();
    assert_url_eq(&client, format!("/users/{}", USER_ID));
}

#[test]
fn create_users() {
    let client = Graph::new("");

    client.v1().users(USER_ID).create(&String::new());
    assert_url_eq(&client, "/users");
}

#[test]
fn delete_users() {
    let client = Graph::new("");
    client.v1().users(USER_ID).delete();
    assert_url_eq(&client, format!("/users/{}", USER_ID));
}

#[test]
fn update_users() {
    let client = Graph::new("");

    client.beta().users(USER_ID).update(&String::new());
    assert_url_beta_eq(&client, format!("/users/{}", USER_ID));

    client.beta().users(USER_ID).update(&String::new());
    assert_url_beta_eq(&client, format!("/users/{}", USER_ID));
}
