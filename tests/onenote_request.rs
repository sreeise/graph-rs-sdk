use graph_http::FileConfig;

use graph_rs_sdk::header::{HeaderValue, CONTENT_TYPE};

use std::ffi::{OsStr};


use std::thread;
use std::time::Duration;

use test_tools::oauthrequest::ASYNC_THROTTLE_MUTEX;
use test_tools::oauthrequest::{Environment, OAuthTestClient};
use test_tools::support::cleanup::{AsyncCleanUp};

#[tokio::test]
async fn list_get_notebooks_and_sections() {
    if Environment::is_appveyor() {
        return;
    }

    let _lock = ASYNC_THROTTLE_MUTEX.lock().await;
    if let Some((id, client)) = OAuthTestClient::ClientCredentials.graph_async().await {
        let notebooks = client
            .user(id.as_str())
            .onenote()
            .notebooks()
            .list_notebooks()
            .send()
            .await;

        if let Ok(response) = notebooks {
            let body: serde_json::Value = response.json().await.unwrap();
            let vec = body["value"].as_array().unwrap();

            let mut found_test_notebook = false;
            let mut notebook_id = String::new();
            for value in vec.iter() {
                if value["displayName"].as_str().unwrap().eq("TestNotebook") {
                    found_test_notebook = true;
                    notebook_id.push_str(value["id"].as_str().unwrap());
                }
            }

            assert!(found_test_notebook);
            let get_notebook = client
                .user(id.as_str())
                .onenote()
                .notebook(notebook_id.as_str())
                .get_notebooks()
                .send()
                .await;

            if let Ok(notebook_response) = get_notebook {
                let body: serde_json::Value = notebook_response.json().await.unwrap();
                assert_eq!("TestNotebook", body["displayName"].as_str().unwrap());
            } else if let Err(e) = get_notebook {
                panic!(
                    "Request error. Method: onenote notebooks get. Error: {e:#?}"
                );
            }

            let result = client
                .user(id.as_str())
                .onenote()
                .notebook(notebook_id.as_str())
                .sections()
                .list_sections()
                .send()
                .await;

            if let Ok(response) = result {
                let body: serde_json::Value = response.json().await.unwrap();
                let vec = body["value"].as_array().unwrap();
                let section_name = vec[0]["displayName"].as_str().unwrap();
                assert_eq!("TestSection", section_name);
            } else if let Err(e) = result {
                panic!(
                    "Request error. Method: onenote notebooks list sections. Error: {e:#?}"
                );
            }
        } else if let Err(e) = notebooks {
            panic!(
                "Request error. Method: onenote notebooks list. Error: {e:#?}"
            );
        }
    }
}

#[tokio::test]
async fn create_delete_page_from_file() {
    if Environment::is_appveyor() {
        return;
    }

    let _lock = ASYNC_THROTTLE_MUTEX.lock().await;
    if let Some((id, client)) = OAuthTestClient::ClientCredentials.graph_async().await {
        let res = client
            .user(&id)
            .onenote()
            .pages()
            .create_pages(&FileConfig::new("./test_files/one_note_page.html"))
            .header(CONTENT_TYPE, HeaderValue::from_static("text/html"))
            .send()
            .await;

        if let Ok(response) = res {
            assert!(response.status().is_success());
            let body: serde_json::Value = response.json().await.unwrap();
            let page_id = body["id"].as_str().unwrap();

            thread::sleep(Duration::from_secs(4));
            let delete_res = client
                .user(&id)
                .onenote()
                .page(page_id)
                .delete_pages()
                .send()
                .await;

            if let Err(e) = delete_res {
                panic!(
                    "Request error. Method onenote pages delete page: Error: {e:#?}"
                );
            }
        } else if let Err(e) = res {
            panic!("Request error. Method onenote create page. Error: {e:#?}");
        }
    }
}

#[tokio::test]
async fn download_page() {
    if Environment::is_appveyor() {
        return;
    }

    let _lock = ASYNC_THROTTLE_MUTEX.lock().await;
    if let Some((user_id, client)) = OAuthTestClient::ClientCredentials.graph_async().await {
        let file_location = "./test_files/downloaded_page.html";
        let mut clean_up = AsyncCleanUp::new_remove_existing(file_location);
        clean_up.rm_files(file_location.into());

        let res = client
            .user(&user_id)
            .onenote()
            .pages()
            .create_pages(&FileConfig::new("./test_files/one_note_page.html"))
            .header(CONTENT_TYPE, HeaderValue::from_static("text/html"))
            .send()
            .await;

        if let Ok(response) = res {
            assert!(response.status().is_success());
            thread::sleep(Duration::from_secs(4));
            let body: serde_json::Value = response.json().await.unwrap();
            println!("Page: \n{:#?}\n", &body);
            let page_id = body["id"].as_str().unwrap();

            let result = client
                .user(&user_id)
                .onenote()
                .page(page_id)
                .get_pages_content()
                .download(
                    &FileConfig::new("./test_files").file_name(OsStr::new("downloaded_page.html")),
                )
                .await;

            if let Err(e) = result {
                panic!("Request error. Method onenote page download page | get content -> download page. Error: {e:#?}");
            } else if let Ok(path_buf) = result {
                assert!(path_buf.exists());
            }

            thread::sleep(Duration::from_secs(4));
            let response = client
                .user(&user_id)
                .onenote()
                .page(page_id)
                .delete_pages()
                .send()
                .await
                .expect("onenote delete pages from page id");

            assert!(response.status().is_success());
        } else if let Err(e) = res {
            panic!("Request error. Method onenote create page (download page test) | 01 get content -> download page. Error: {e:#?}");
        }
    }
}
