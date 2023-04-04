use std::thread;
use std::time::Duration;
use test_tools::oauth_request::ASYNC_THROTTLE_MUTEX;
use test_tools::oauth_request::{Environment, OAuthTestClient};

#[tokio::test]
async fn list_and_get_messages() {
    if Environment::is_local() {
        let _ = ASYNC_THROTTLE_MUTEX.lock().await;
        if let Some((id, client)) = OAuthTestClient::ClientCredentials.graph_async().await {
            if let Ok(response) = client
                .user(id.as_str())
                .messages()
                .list_messages()
                .send()
                .await
            {
                let body: serde_json::Value = response.json().await.unwrap();
                let vec = body["value"].as_array().unwrap();
                let message_id = vec[0]["id"].as_str().unwrap();

                let response = client
                    .user(id.as_str())
                    .message(message_id)
                    .get_messages()
                    .send()
                    .await
                    .unwrap();

                assert!(response.status().is_success());
                let body: serde_json::Value = response.json().await.unwrap();
                let m_id = body["id"].as_str().unwrap();
                assert_eq!(m_id, message_id);
            } else {
                panic!("Request error. Method: mail messages list");
            }
        }
    }
}

#[tokio::test]
async fn mail_create_and_delete_message() {
    if Environment::is_local() {
        let _ = ASYNC_THROTTLE_MUTEX.lock().await;
        if let Some((id, mut client)) = OAuthTestClient::ClientCredentials.graph_async().await {
            let result = client
                .v1()
                .user(id.as_str())
                .messages()
                .create_messages(&serde_json::json!({
                    "subject":"Did you see last night's game?",
                    "importance":"Low",
                        "body":{
                            "contentType":"HTML",
                            "content":"They were <b>awesome</b>!"
                        },
                    "toRecipients":[
                            {
                                "emailAddress":{
                                    "address":"AdeleV@contoso.onmicrosoft.com"
                                }
                            }
                    ]
                }))
                .send()
                .await;

            if let Ok(response) = result {
                let body: serde_json::Value = response.json().await.unwrap();
                let message_id = body["id"].as_str().unwrap();

                thread::sleep(Duration::from_secs(2));
                let delete_res = client
                    .v1()
                    .user(id.as_str())
                    .message(message_id)
                    .delete_messages()
                    .send()
                    .await;
                if let Err(e) = delete_res {
                    panic!("Request error. Method: mail messages delete. Error: {e:#?}");
                }
            } else if let Err(e) = result {
                panic!("Request error. Method: mail messages create. Error: {e:#?}");
            }
        }
    }
}
