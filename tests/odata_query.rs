use graph_rs_sdk::*;
use test_tools::oauth_request::OAuthTestClient;

#[test]
fn select_query() {
    let client = Graph::new("");

    assert_eq!(
        "https://graph.microsoft.com/v1.0/me/drive?%24select=id%2Cname".to_string(),
        client
            .me()
            .drive()
            .get_drive()
            .select(&["id", "name"])
            .url()
            .to_string()
    );
}

#[test]
fn expand_query() {
    let client = Graph::new("");

    assert_eq!(
        "https://graph.microsoft.com/v1.0/me/drive?%24expand=users".to_string(),
        client
            .me()
            .drive()
            .get_drive()
            .expand(&["users"])
            .url()
            .to_string()
    );
}

#[test]
fn filter_query() {
    let client = Graph::new("");

    assert_eq!(
        "https://graph.microsoft.com/v1.0/me/drive?%24filter=startsWith%28displayName%2C%27j%27%29"
            .to_string(),
        client
            .me()
            .drive()
            .get_drive()
            .filter(&["startsWith(displayName,'j')"])
            .url()
            .to_string()
    );
}

#[test]
fn expand_filter_query() {
    let client = Graph::new("");

    assert_eq!(
        "https://graph.microsoft.com/v1.0/me/drive?%24expand=users&%24filter=name".to_string(),
        client
            .me()
            .drive()
            .get_drive()
            .expand(&["users"])
            .filter(&["name"])
            .url()
            .to_string()
    );
}

#[tokio::test]
async fn filter_query_request_v1() {
    if let Some((_id, client)) = OAuthTestClient::ClientCredentials.graph_async().await {
        let result = client
            .users()
            .list_user()
            .filter(&["startswith(givenName, 'A')"])
            .send()
            .await;

        if let Ok(response) = result {
            let body: serde_json::Value = response.json().await.unwrap();
            let users = body["value"].as_array().unwrap();
            let found_user = users.iter().find(|user| {
                let name = user["displayName"].as_str().unwrap();
                name.eq("Adele Vance")
            });

            assert!(found_user.is_some());
        } else if let Err(e) = result {
            panic!("Request Error. Method: filter_query_request. Error: {e:#?}");
        }
    }
}

#[tokio::test]
async fn filter_query_request_beta() {
    if let Some((_id, mut client)) = OAuthTestClient::ClientCredentials.graph_async().await {
        let result = client
            .beta()
            .users()
            .list_user()
            .filter(&["startswith(givenName, 'A')"])
            .send()
            .await;

        if let Ok(response) = result {
            let body: serde_json::Value = response.json().await.unwrap();
            let users = body["value"].as_array().unwrap();
            let found_user = users.iter().find(|user| {
                let name = user["displayName"].as_str().unwrap();
                name.eq("Adele Vance")
            });

            assert!(found_user.is_some());
        } else if let Err(e) = result {
            panic!("Request Error. Method: filter_query_request. Error: {e:#?}");
        }
    }
}

#[tokio::test]
async fn order_by_query_request_v1() {
    if let Some((_id, client)) = OAuthTestClient::ClientCredentials.graph_async().await {
        let result = client
            .users()
            .list_user()
            .order_by(&["displayName"])
            .send()
            .await;

        if let Ok(response) = result {
            let body: serde_json::Value = response.json().await.unwrap();
            let users = body["value"].as_array().unwrap();
            let found_user = users.iter().find(|user| {
                let name = user["displayName"].as_str().unwrap();
                name.eq("Adele Vance")
            });

            assert!(found_user.is_some());
        } else if let Err(e) = result {
            panic!("Request Error. Method: filter_query_request. Error: {e:#?}");
        }
    }
}

#[tokio::test]
async fn order_by_request_beta() {
    if let Some((_id, mut client)) = OAuthTestClient::ClientCredentials.graph_async().await {
        let result = client
            .beta()
            .users()
            .list_user()
            .order_by(&["displayName"])
            .send()
            .await;

        if let Ok(response) = result {
            let body: serde_json::Value = response.json().await.unwrap();
            let users = body["value"].as_array().unwrap();
            let found_user = users.iter().find(|user| {
                let name = user["displayName"].as_str().unwrap();
                name.eq("Adele Vance")
            });

            assert!(found_user.is_some());
        } else if let Err(e) = result {
            panic!("Request Error. Method: filter_query_request. Error: {e:#?}");
        }
    }
}
