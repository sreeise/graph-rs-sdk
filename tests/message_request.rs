use std::time::Duration;
use test_tools::oauth_request::{Environment, DEFAULT_CLIENT_CREDENTIALS_MUTEX3};

#[tokio::test]
async fn list_and_get_messages() {
    if Environment::is_local() {
        let test_client = DEFAULT_CLIENT_CREDENTIALS_MUTEX3.lock().await;
        if let Ok(response) = test_client
            .client
            .user(test_client.user_id.as_str())
            .messages()
            .list_messages()
            .send()
            .await
        {
            let body: serde_json::Value = response.json().await.unwrap();
            let vec = body["value"].as_array().unwrap();
            let message_id = vec[0]["id"].as_str().unwrap();

            let response = test_client
                .client
                .user(test_client.user_id.as_str())
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

#[tokio::test]
async fn mail_create_and_delete_message() {
    if Environment::is_local() {
        let mut test_client = DEFAULT_CLIENT_CREDENTIALS_MUTEX3.lock().await;
        let user_id = test_client.user_id.clone();
        let result = test_client
            .client
            .v1()
            .user(user_id.as_str())
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

            tokio::time::sleep(Duration::from_secs(1)).await;
            let delete_res = test_client
                .client
                .v1()
                .user(user_id.as_str())
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
