use graph_rs::prelude::*;
use std::error::Error;
use std::thread;
use std::time::Duration;
use test_tools::oauthrequest::OAuthRequest;
use test_tools::oauthrequest::THROTTLE_MUTEX;

#[test]
fn list_and_get_messages() {
    if OAuthRequest::is_appveyor() {
        return;
    }

    let _lock = THROTTLE_MUTEX.lock().unwrap();
    OAuthRequest::access_token_fn(|t| {
        if let Some((id, bearer)) = t {
            let client = Graph::new(bearer.as_str());
            if let Ok(res) = client
                .v1()
                .users(id.as_str())
                .mail()
                .messages()
                .list()
                .value()
            {
                let value = res.value().clone();
                let value = value["value"][0].clone();
                let message_id = value["id"].as_str().unwrap();

                let get_req = client
                    .v1()
                    .users(id.as_str())
                    .mail()
                    .messages()
                    .get(message_id)
                    .value();

                if let Ok(response) = get_req {
                    println!("{:#?}", response);
                    let value = response.value().clone();
                    let m_id = value["id"].as_str().unwrap();
                    assert_eq!(m_id, message_id);
                } else if let Err(_) = get_req {
                    panic!("Request error. Method: mail messages get");
                }
            } else {
                panic!("Request error. Method: mail messages list");
            }
        }
    });
}

#[test]
fn mail_create_and_delete_message() {
    if OAuthRequest::is_appveyor() {
        return;
    }

    let _lock = THROTTLE_MUTEX.lock().unwrap();
    OAuthRequest::access_token_fn(|t| {
        if let Some((id, bearer)) = t {
            let client = Graph::new(bearer.as_str());
            let result = client
                .v1()
                .users(id.as_str())
                .mail()
                .messages()
                .create(&serde_json::json!({
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
                .value();

            if let Ok(message) = result {
                let message_id = message.value()["id"].as_str().unwrap();

                thread::sleep(Duration::from_secs(2));
                let delete_res = client
                    .v1()
                    .users(id.as_str())
                    .mail()
                    .messages()
                    .delete(message_id)
                    .send();
                if let Err(e) = delete_res {
                    panic!(
                        "Request error. Method: mail messages delete. Error: {:#?}",
                        e.description()
                    );
                }
            } else if let Err(e) = result {
                panic!(
                    "Request error. Method: mail messages create. Error: {:#?}",
                    e.description()
                );
            }
        }
    });
}
