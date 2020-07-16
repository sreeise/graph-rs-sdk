use graph_rs::prelude::*;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use test_tools::oauthrequest::OAuthRequest;
use test_tools::oauthrequest::DRIVE_THROTTLE_MUTEX;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;

#[tokio::test]
async fn create_delete_folder_async() {
    let _lock = DRIVE_THROTTLE_MUTEX.lock().unwrap();
    if let Some((id, token)) = OAuthRequest::request_access_token_async().await {
        let client = Graph::new_async(token.bearer_token());
        let folder: HashMap<String, serde_json::Value> = HashMap::new();
        let create_folder_res = client
            .v1()
            .drives(id.as_str())
            .drive()
            .create_folder(
                "",
                &serde_json::json!({
                    "name": "ci_docs_async",
                    "folder": folder,
                    "@microsoft.graph.conflictBehavior": "rename"
                }),
            )
            .send()
            .await;

        if let Ok(response) = create_folder_res {
            let item_id = response.body()["id"].as_str().unwrap();
            tokio::time::delay_for(Duration::from_secs(2)).await;

            let req = client.v1().drives(id).drive().delete(item_id).send().await;

            if let Ok(res) = req {
                assert!(res.error().is_none());
            } else if let Err(e) = req {
                panic!("Request error. Method: drive delete. Error: {:#?}", e);
            }
        } else if let Err(e) = create_folder_res {
            panic!("Request error. Method: create folder. Error: {:#?}", e);
        }
    }
}

#[tokio::test]
async fn drive_upload_new_and_replace_and_delete() {
    let _lock = DRIVE_THROTTLE_MUTEX.lock().unwrap();
    if let Some((id, token)) = OAuthRequest::request_access_token_async().await {
        let client = Graph::new_async(token.bearer_token());
        let upload_res = client
            .v1()
            .drives(id.as_str())
            .drive()
            .upload_new(
                ":/test_upload_file_async.txt:",
                "./test_files/test_upload_file_async.txt",
            )
            .send()
            .await;

        if let Ok(value) = upload_res {
            assert!(value.body()["id"].as_str().is_some());
            let item_id = value.body()["id"].as_str().unwrap();

            let mut file = OpenOptions::new()
                .write(true)
                .open("./test_files/test_upload_file_async.txt")
                .await
                .unwrap();

            file.write_all("Test Update File".as_bytes()).await.unwrap();
            file.sync_all().await.unwrap();

            tokio::time::delay_for(Duration::from_secs(2)).await;
            let upload_replace = client
                .v1()
                .drives(id.as_str())
                .drive()
                .upload_replace(item_id, "./test_files/test_upload_file_async.txt")
                .send()
                .await;

            if let Ok(value) = upload_replace {
                let item_id2 = value.body()["id"].as_str().unwrap();
                assert_eq!(item_id, item_id2);
            } else if let Err(e) = upload_replace {
                panic!(
                    "Request Error. Method: drive upload replace. Error: {:#?}",
                    e
                );
            }

            thread::sleep(Duration::from_secs(2));
            let delete_res = client
                .v1()
                .drives(id.as_str())
                .drive()
                .delete(item_id)
                .send()
                .await;

            if let Ok(result) = delete_res {
                assert!(result.error().is_none());
            } else if let Err(e) = delete_res {
                panic!("Request Error. Method: drive delete. Error: {:#?}", e);
            }
        } else if let Err(e) = upload_res {
            panic!("Request Error. Method: drive upload. Error: {:#?}", e);
        }
    }
}
