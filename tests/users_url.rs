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

#[test]
fn list_users() {
    let client = Graph::new("");

    client.v1().users().user().list();
    assert_url_eq(&client, "users".into());

    client.v1().sites().user().list();
    assert_url_eq(&client, "users".into());

    client.beta().sites().user().list();
    assert_url_beta_eq(&client, "users".into());

    client.beta().users().user().list();
    assert_url_beta_eq(&client, "users".into());
}

#[test]
fn get_users() {
    let client = Graph::new("");

    client.v1().me().user().get();
    client.format_ord();
    assert_url_eq(&client, "me".into());

    client
        .v1()
        .users()
        .user()
        .get()
        .by_id("b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI");
    assert_url_eq(
        &client,
        "users/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI".into(),
    );

    client
        .v1()
        .sites()
        .user()
        .get()
        .by_id("b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI");
    assert_url_eq(
        &client,
        "users/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI".into(),
    );

    client
        .beta()
        .sites()
        .user()
        .get()
        .by_id("b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI");
    assert_url_beta_eq(
        &client,
        "users/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI".into(),
    );

    client
        .beta()
        .users()
        .user()
        .get()
        .by_id("b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI");
    assert_url_beta_eq(
        &client,
        "users/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI".into(),
    );
}

#[test]
fn create_users() {
    let client = Graph::new("");
    let user = User::default();

    client.v1().users().user().create(&user);
    assert_url_eq(&client, "users".into());

    client.v1().sites().user().create(&user);
    assert_url_eq(&client, "users".into());

    client.beta().sites().user().create(&user);
    assert_url_beta_eq(&client, "users".into());

    client.beta().users().user().create(&user);
    assert_url_beta_eq(&client, "users".into());
}

#[test]
fn delete_users() {
    let client = Graph::new("");

    client
        .v1()
        .users()
        .user()
        .delete()
        .by_id("b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI");
    assert_url_eq(
        &client,
        "users/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI".into(),
    );

    client
        .v1()
        .sites()
        .user()
        .delete()
        .by_id("b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI");
    assert_url_eq(
        &client,
        "users/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI".into(),
    );

    client
        .beta()
        .sites()
        .user()
        .delete()
        .by_id("b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI");
    assert_url_beta_eq(
        &client,
        "users/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI".into(),
    );

    client
        .beta()
        .users()
        .user()
        .delete()
        .by_id("b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI");
    assert_url_beta_eq(
        &client,
        "users/b!CbtYWrofwUGBJWnaJkNwoNrBLp_kC3RKklSXPwrdeP3yH8_qmH9xT5Y6RODPNfYI".into(),
    );
}

#[test]
fn update_users() {
    let client = Graph::new("");
    let user = User::default();

    client.v1().users().user().update(&user);
    assert_url_eq(&client, "users".into());

    client.v1().sites().user().update(&user);
    assert_url_eq(&client, "users".into());

    client.beta().sites().user().update(&user);
    assert_url_beta_eq(&client, "users".into());

    client.beta().users().user().update(&user);
    assert_url_beta_eq(&client, "users".into());
}
