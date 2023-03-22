use futures_util::SinkExt;
use graph_rs_sdk::prelude::*;
use graph_rs_sdk::{GRAPH_URL, GRAPH_URL_BETA};
use test_tools::oauthrequest::OAuthTestClient;

#[test]
pub fn batch_url() {
    let client = Graph::new("");

    assert_eq!(
        "/v1.0/$batch".to_string(),
        client.batch(&serde_json::json!({})).url().path()
    );

    assert_eq!(
        "/beta/$batch".to_string(),
        client.beta().batch(&serde_json::json!({})).url().path()
    );
}

#[tokio::test]
pub async fn batch_request() {
    if let Some((id, client)) = OAuthTestClient::ClientCredentials.graph_async().await {
        let mut one = false;
        let mut two = false;
        let mut three = false;
        let mut four = false;
        let mut five = false;

        let json = serde_json::json!({
            "requests": [
                {
                    "id": "1",
                    "method": "GET",
                    "url": format!("/users/{}/drive", id.as_str())
                },
                {
                    "id": "2",
                    "method": "GET",
                    "url": format!("/users/{}/drive/root", id.as_str())
                },
                {
                    "id": "3",
                    "method": "GET",
                    "url": format!("/users/{}/drive/recent", id.as_str())
                },
                {
                    "id": "4",
                    "method": "GET",
                    "url": format!("/users/{}/drive/root/children", id.as_str())
                },
                {
                    "id": "5",
                    "method": "GET",
                    "url": format!("/users/{}/drive/special/documents", id.as_str())
                },
            ]
        });

        let mut response = client.batch(&json).send().await.unwrap();

        let body: serde_json::Value = response.json().await.unwrap();

        for v in body["responses"].as_array().unwrap().iter() {
            match v["id"].as_str().unwrap().as_bytes() {
                b"1" => {
                    one = true;
                }
                b"2" => {
                    two = true;
                }
                b"3" => {
                    three = true;
                }
                b"4" => {
                    four = true;
                }
                b"5" => {
                    five = true;
                }
                _ => {}
            }
        }

        assert!(one);
        assert!(two);
        assert!(three);
        assert!(four);
        assert!(five);
    }
}
