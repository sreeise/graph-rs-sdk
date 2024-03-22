use futures::StreamExt;
use std::collections::VecDeque;
use test_tools::oauth_request::{Environment, DEFAULT_CLIENT_CREDENTIALS_MUTEX4};

#[tokio::test]
async fn paging_all() {
    if Environment::is_appveyor() {
        return;
    }

    let test_client = DEFAULT_CLIENT_CREDENTIALS_MUTEX4.lock().await;
    let mut vec = test_client
        .client
        .users()
        .delta()
        .paging()
        .json::<serde_json::Value>()
        .await
        .unwrap();

    assert!(!vec.is_empty());
    for response in vec.iter() {
        assert!(response.status().is_success())
    }

    let response = vec.pop_back().unwrap();
    let body = response.into_body().unwrap();
    assert!(body["@odata.deltaLink"].as_str().is_some())
}

#[tokio::test]
async fn paging_stream() {
    if Environment::is_local() {
        let test_client = DEFAULT_CLIENT_CREDENTIALS_MUTEX4.lock().await;
        let mut stream = test_client
            .client
            .users()
            .delta()
            .paging()
            .stream::<serde_json::Value>()
            .unwrap();

        let mut deque = VecDeque::new();
        while let Some(result) = stream.next().await {
            match result {
                Ok(response) => {
                    assert!(response.status().is_success());
                    let body = response.into_body().unwrap();
                    deque.push_back(body);
                }
                Err(err) => panic!("Error on stream users delta\n{err:#?}"),
            }
        }

        assert!(deque.len() >= 2);
        let last = deque.pop_back().unwrap();
        assert!(last["@odata.deltaLink"].as_str().is_some());

        for body in deque.iter() {
            assert!(body["@odata.nextLink"].as_str().is_some());
            assert!(body["@odata.deltaLink"].as_str().is_none());
        }
    }
}
