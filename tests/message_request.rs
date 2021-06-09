use std::thread;
use std::time::Duration;
use test_tools::oauthrequest::THROTTLE_MUTEX;
use test_tools::oauthrequest::{Environment, OAuthTestClient};

#[test]
fn list_and_get_messages() {
    if Environment::is_appveyor() {
        return;
    }

    let _lock = THROTTLE_MUTEX.lock().unwrap();
    if let Some((id, client)) = OAuthTestClient::ClientCredentials.graph() {
        if let Ok(res) = client
            .v1()
            .user(id.as_str())
            .messages()
            .list_messages()
            .send()
        {
            let vec = res.body()["value"].as_array().unwrap();
            let message_id = vec[0]["id"].as_str().unwrap();
            let get_req = client
                .v1()
                .user(id.as_str())
                .message(message_id)
                .get_messages()
                .send();

            if let Ok(response) = get_req {
                println!("{:#?}", response);
                let value = response.body().clone();
                let m_id = value["id"].as_str().unwrap();
                assert_eq!(m_id, message_id);
            } else if get_req.is_err() {
                panic!("Request error. Method: mail messages get");
            }
        } else {
            panic!("Request error. Method: mail messages list");
        }
    }
}

#[test]
fn mail_create_and_delete_message() {
    if Environment::is_appveyor() {
        return;
    }

    let _lock = THROTTLE_MUTEX.lock().unwrap();
    if let Some((id, client)) = OAuthTestClient::ClientCredentials.graph() {
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
            .send();

        if let Ok(message) = result {
            let message_id = message.body()["id"].as_str().unwrap();

            thread::sleep(Duration::from_secs(2));
            let delete_res = client
                .v1()
                .user(id.as_str())
                .message(message_id)
                .delete_messages()
                .send();
            if let Err(e) = delete_res {
                panic!(
                    "Request error. Method: mail messages delete. Error: {:#?}",
                    e
                );
            }
        } else if let Err(e) = result {
            panic!(
                "Request error. Method: mail messages create. Error: {:#?}",
                e
            );
        }
    }
}
