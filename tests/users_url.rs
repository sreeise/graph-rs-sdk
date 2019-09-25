use graph_rs::prelude::*;
use graph_rs::{GRAPH_URL, GRAPH_URL_BETA};
use graph_rs_types::entitytypes::User;

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

    client.v1().user().list();
    assert_url_eq(&client, "users".into());
}

#[test]
fn get_users() {
    let client = Graph::new("");

    client.v1().me().user().get();
    client.format_ord();
    assert_url_eq(&client, "me".into());

    client.v1().user().get().by_id(USER_ID);
    assert_url_eq(&client, format!("users/{}", USER_ID));
}

#[test]
fn create_users() {
    let client = Graph::new("");
    let user = User::default();

    client.v1().user().create(&user);
    assert_url_eq(&client, "users".into());
}

#[test]
fn delete_users() {
    let client = Graph::new("");
    client.v1().user().delete().by_id(USER_ID);
    assert_url_eq(&client, format!("users/{}", USER_ID));
}

#[test]
fn update_users() {
    let client = Graph::new("");
    let user = User::default();

    client.beta().me().user().update(&user).by_id(USER_ID);
    assert_url_beta_eq(&client, format!("users/{}", USER_ID));

    client.beta().user().update(&user).by_id(USER_ID);
    assert_url_beta_eq(&client, format!("users/{}", USER_ID));
}
