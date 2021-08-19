use graph_rs_sdk::prelude::*;
use test_tools::assert_url_eq;
use test_tools::oauthrequest::OAuthTestClient;

#[test]
fn select_query() {
    let client = Graph::new("token");

    client.v1().me().drive().get_drive().select(&["id", "name"]);
    assert_url_eq(&client, "/me/drive?%24select=id%2Cname");
}

#[test]
fn expand_query() {
    let client = Graph::new("token");

    client.v1().me().drive().get_drive().expand(&["users"]);
    assert_url_eq(&client, "/me/drive?%24expand=users");
}

#[test]
fn filter_query() {
    let client = Graph::new("token");

    client
        .v1()
        .me()
        .drive()
        .get_drive()
        .filter(&["startsWith(displayName,'j')"]);
    assert_url_eq(
        &client,
        "/me/drive?%24filter=startsWith%28displayName%2C%27j%27%29",
    );
}

#[test]
fn expand_filter_query() {
    let client = Graph::new("token");

    client
        .v1()
        .me()
        .drive()
        .get_drive()
        .expand(&["users"])
        .filter(&["name"]);
    assert_url_eq(&client, "/me/drive?%24expand=users&%24filter=name");
}

#[test]
fn filter_query_request_v1() {
    if let Some((_id, client)) = OAuthTestClient::ClientCredentials.graph() {
        let result = client
            .v1()
            .users()
            .list_user()
            .filter(&["startswith(givenName, 'A')"])
            .send();

        if let Ok(response) = result {
            let users = response.body()["value"].as_array().unwrap();
            let found_user = users.iter().find(|user| {
                let name = user["displayName"].as_str().unwrap();
                name.eq("Adele Vance")
            });

            if found_user.is_none() {
                panic!("Request Error. Method: filter_query_request. Error: Could not find displayName equal to Adele Vance");
            }
        } else if let Err(e) = result {
            panic!(
                "Request Error. Method: filter_query_request. Error: {:#?}",
                e
            );
        }
    }
}

#[test]
fn filter_query_request_beta() {
    if let Some((_id, client)) = OAuthTestClient::ClientCredentials.graph() {
        let result = client
            .beta()
            .users()
            .list_user()
            .filter(&["startswith(givenName, 'A')"])
            .send();

        if let Ok(response) = result {
            let users = response.body()["value"].as_array().unwrap();
            let found_user = users.iter().find(|user| {
                let name = user["displayName"].as_str().unwrap();
                name.eq("Adele Vance")
            });

            if found_user.is_none() {
                panic!("Request Error. Method: filter_query_request. Error: Could not find displayName equal to Adele Vance");
            }
        } else if let Err(e) = result {
            panic!(
                "Request Error. Method: filter_query_request. Error: {:#?}",
                e
            );
        }
    }
}

#[test]
fn order_by_query_request_v1() {
    if let Some((_id, client)) = OAuthTestClient::ClientCredentials.graph() {
        let result = client
            .v1()
            .users()
            .list_user()
            .order_by(&["displayName"])
            .send();

        if let Ok(response) = result {
            let users = response.body()["value"].as_array().unwrap();
            let found_user = users.iter().find(|user| {
                let name = user["displayName"].as_str().unwrap();
                name.eq("Adele Vance")
            });

            if found_user.is_none() {
                panic!("Request Error. Method: filter_query_request. Error: Could not find displayName equal to Adele Vance");
            }
        } else if let Err(e) = result {
            panic!(
                "Request Error. Method: filter_query_request. Error: {:#?}",
                e
            );
        }
    }
}

#[test]
fn order_by_request_beta() {
    if let Some((_id, client)) = OAuthTestClient::ClientCredentials.graph() {
        let result = client
            .beta()
            .users()
            .list_user()
            .order_by(&["displayName"])
            .send();

        if let Ok(response) = result {
            let users = response.body()["value"].as_array().unwrap();
            let found_user = users.iter().find(|user| {
                let name = user["displayName"].as_str().unwrap();
                name.eq("Adele Vance")
            });

            if found_user.is_none() {
                panic!("Request Error. Method: filter_query_request. Error: Could not find displayName equal to Adele Vance");
            }
        } else if let Err(e) = result {
            panic!(
                "Request Error. Method: filter_query_request. Error: {:#?}",
                e
            );
        }
    }
}
