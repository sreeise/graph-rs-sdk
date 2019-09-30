use graph_rs::prelude::*;
use graph_rs::{GRAPH_URL, GRAPH_URL_BETA};

fn assert_url_eq(client: &Graph, path: String) {
    client.url_ref(|url| {
        assert_eq!(url.to_string(), format!("{}/{}", GRAPH_URL, path));
    });
}

fn assert_url_beta_eq(client: &Graph, path: String) {
    client.url_ref(|url| {
        assert_eq!(url.to_string(), format!("{}/{}", GRAPH_URL_BETA, path));
    });
}

static USER_ID: &str = "b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI";

#[test]
fn list_users() {
    let client = Graph::new("");

    client.v1().users("").list();
    assert_url_eq(&client, "users".into());
}

#[test]
fn get_users() {
    let client = Graph::new("");

    client.v1().me().get();
    assert_url_eq(&client, "me".into());

    client.v1().users(USER_ID).get();
    assert_url_eq(&client, format!("users/{}", USER_ID));
}

#[test]
fn create_users() {
    let client = Graph::new("");

    client.v1().users(USER_ID).create(&String::new());
    assert_url_eq(&client, "users".into());
}

#[test]
fn delete_users() {
    let client = Graph::new("");
    client.v1().users(USER_ID).delete();
    assert_url_eq(&client, format!("users/{}", USER_ID));
}

#[test]
fn update_users() {
    let client = Graph::new("");

    client.beta().users(USER_ID).update(&String::new());
    assert_url_beta_eq(&client, format!("users/{}", USER_ID));

    client.beta().users(USER_ID).update(&String::new());
    assert_url_beta_eq(&client, format!("users/{}", USER_ID));
}
