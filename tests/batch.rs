use graph_rs::prelude::*;
use graph_rs::{GRAPH_URL, GRAPH_URL_BETA};
use std::error::Error;
use test_tools::oauthrequest::OAuthRequest;

#[test]
pub fn batch_url() {
    let client = Graph::new("");

    client.v1().batch(&serde_json::json!({}));

    client.url_ref(|url| {
        assert_eq!(url.to_string(), format!("{}/{}", GRAPH_URL, "$batch"));
    });

    client.beta().batch(&serde_json::json!({}));

    client.url_ref(|url| {
        assert_eq!(url.to_string(), format!("{}/{}", GRAPH_URL_BETA, "$batch"));
    });
}

#[test]
pub fn batch_request() {
    OAuthRequest::access_token_fn(|t| {
        if let Some((id, bearer)) = t {
            let client = Graph::new(bearer.as_str());

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
                    }
                ]
            });

            let recv = client.v1().batch(&json).value();

            loop {
                match recv.recv() {
                    Ok(delta) => match delta {
                        Delta::Next(response) => {
                            let value = response.value().clone();
                            for v in value["responses"].as_array().unwrap().iter() {
                                match v["id"].as_str().unwrap().as_bytes() {
                                    b"1" => {
                                        one = true;
                                    },
                                    b"2" => {
                                        two = true;
                                    },
                                    b"3" => {
                                        three = true;
                                    },
                                    b"4" => {
                                        four = true;
                                    },
                                    b"5" => {
                                        five = true;
                                    },
                                    _ => {},
                                }
                            }
                        },
                        Delta::Done(err) => {
                            if let Some(err) = err {
                                panic!("Request Error. Method: drive batch - received error on Delta::Done. Error: {:#?}", err.description());
                            } else {
                                break;
                            }
                        },
                    },
                    Err(e) => {
                        panic!(
                            "Request error. Method: batch. Error: {:#?}",
                            e.description()
                        );
                    },
                }
            }

            assert!(one);
            assert!(two);
            assert!(three);
            assert!(four);
            assert!(five);
        }
    });
}
