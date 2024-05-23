use graph_rs_sdk::*;
use std::thread;
use std::time::Duration;
use test_tools::oauth_request::OAuthTestClient;

fn get_special_folder_id(user_id: &str, folder: &str, client: &Graph) -> GraphResult<String> {
    let response = client
        .user(user_id)
        .drive()
        .get_special(folder)
        .into_blocking()
        .send()?;

    let body: serde_json::Value = response.json()?;
    let parent_reference_id = body["id"].as_str().unwrap();
    Ok(parent_reference_id.into())
}

fn upload_file_reqwest_body(
    user_id: &str,
    parent_reference_id: &str,
    file_name: &str,
    local_file: &str,
    client: &Graph,
) -> GraphResult<reqwest::blocking::Response> {
    let file = std::fs::File::open(local_file)?;
    let body = reqwest::blocking::Body::from(file);

    client
        .drive(user_id)
        .item(parent_reference_id)
        .upload_items_content(file_name, body)
        .into_blocking()
        .send()
}

fn delete_file(
    user_id: &str,
    item_id: &str,
    client: &Graph,
) -> GraphResult<reqwest::blocking::Response> {
    client
        .user(user_id)
        .drive()
        .item(item_id)
        .delete_items()
        .into_blocking()
        .send()
}

fn get_file_content(
    user_id: &str,
    item_id: &str,
    client: &Graph,
) -> GraphResult<reqwest::blocking::Response> {
    client
        .user(user_id)
        .drive()
        .item(item_id)
        .get_items_content()
        .into_blocking()
        .send()
}

#[test]
fn upload_reqwest_body() {
    if let Some((id, client)) = OAuthTestClient::ClientCredentials.graph() {
        let local_file = "./test_files/test_upload_file_bytes.txt";
        let file_name = ":/test_upload_file_bytes.txt:";

        let parent_reference_id = get_special_folder_id(id.as_str(), "Documents", &client).unwrap();
        let upload_res = upload_file_reqwest_body(
            id.as_str(),
            parent_reference_id.as_str(),
            file_name,
            local_file,
            &client,
        );

        if let Ok(response) = upload_res {
            assert!(response.status().is_success());
            let body: serde_json::Value = response.json().unwrap();
            assert!(body["id"].as_str().is_some());
            let item_id = body["id"].as_str().unwrap();

            thread::sleep(Duration::from_secs(3));

            let response = get_file_content(id.as_str(), item_id, &client).unwrap();
            assert!(response.status().is_success());

            let text = response.text().unwrap();
            assert_eq!("Upload Bytes", text.trim());

            let delete_res = delete_file(id.as_str(), item_id, &client);

            if let Ok(response) = delete_res {
                assert!(response.status().is_success());
            } else if let Err(err) = delete_res {
                panic!("Request Error. Method: drive delete. Error: {err:#?}");
            }
        } else if let Err(err) = upload_res {
            panic!("Request Error. Method: drive upload. Error: {err:#?}");
        }
    }
}
