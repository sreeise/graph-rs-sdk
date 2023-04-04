use std::thread;
use std::time::Duration;
use test_tools::oauth_request::OAuthTestClient;

#[test]
fn drive() {
    if let Some((id, client)) = OAuthTestClient::ClientCredentials.graph() {
        let req = client
            .drive(id.as_str())
            .item_by_path(":/update_test_document.docx:")
            .update_items(&serde_json::json!({
                "name": "update_test.docx"
            }))
            .into_blocking()
            .send();

        if let Ok(response) = req {
            assert!(response.status().is_success());
            let body: serde_json::Value = response.json().unwrap();
            assert_eq!(body["name"].as_str(), Some("update_test.docx"));
            thread::sleep(Duration::from_secs(2));

            let req = client
                .drive(id.as_str())
                .item_by_path(":/update_test.docx:")
                .update_items(&serde_json::json!({
                    "name": "update_test_document.docx"
                }))
                .into_blocking()
                .send();

            if let Ok(response) = req {
                assert!(response.status().is_success());
                let body: serde_json::Value = response.json().unwrap();
                assert_eq!(body["name"].as_str(), Some("update_test_document.docx"));
            } else if let Err(e) = req {
                panic!("Request Error. Method: drive update. Error: {e:#?}");
            }
        } else if let Err(e) = req {
            panic!("Request Error. Method: drive check_out. Error: {e:#?}");
        }
    }
}
