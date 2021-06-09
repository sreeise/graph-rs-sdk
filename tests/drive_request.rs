use graph_error::{GraphError, GraphResult};
use graph_http::NextSession;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::ffi::OsString;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;
use test_tools::common::TestTools;
use test_tools::oauthrequest::DRIVE_THROTTLE_MUTEX;
use test_tools::oauthrequest::{Environment, OAuthTestClient};
use test_tools::support::cleanup::CleanUp;

fn test_folder_create_delete(folder_name: &str) {
    if let Some((id, client)) = OAuthTestClient::ClientCredentials.graph() {
        let folder: HashMap<String, serde_json::Value> = HashMap::new();
        let result = client
            .v1()
            .drive(&id)
            .create_root_folder(&serde_json::json!({
                "name": folder_name,
                "folder": folder,
                "@microsoft.graph.conflictBehavior": "fail"
            }))
            .send();

        if let Ok(response) = result {
            assert!(
                response.status() == 200 || response.status() == 201 || response.status() == 204
            );
            let item_id = response.body()["id"].as_str().unwrap();
            thread::sleep(Duration::from_secs(2));

            let result = client.v1().drive(&id).delete_items(item_id).send();

            TestTools::assert_success(&result, "delete folder (conflict behavior: fail)");
        } else if let Err(e) = result {
            panic!("Request error. Method: create folder with encoding. Path: root\nFolder Name: {:#?}\nError: {:#?}",
                   folder_name,
                   e
            );
        }
    }
}

#[test]
fn create_delete_folder() {
    let _lock = DRIVE_THROTTLE_MUTEX.lock().unwrap();
    test_folder_create_delete("ci_docs");
}

#[test]
fn create_delete_folder_path_encode() {
    let _lock = DRIVE_THROTTLE_MUTEX.lock().unwrap();
    test_folder_create_delete("special folder");
}

#[test]
fn list_versions_get_item() {
    let _lock = DRIVE_THROTTLE_MUTEX.lock().unwrap();
    if let Some((id, client)) = OAuthTestClient::ClientCredentials.graph() {
        let get_item_res = client
            .v1()
            .user(id.as_str())
            .drive()
            .get_items(":/copy_folder:")
            .send();

        if let Ok(res) = get_item_res {
            assert!(res.body()["id"].as_str().is_some());
            let item_id = res.body()["id"].as_str().unwrap();

            let versions_res = client
                .v1()
                .user(id.as_str())
                .drive()
                .list_item_versions(item_id)
                .send();

            TestTools::assert_success(&versions_res, "list version");
        } else if let Err(e) = get_item_res {
            panic!("Request Error. Method: drive get_item. Error: {:#?}", e);
        }
    }
}

#[test]
fn drive_check_in_out() {
    let _lock = DRIVE_THROTTLE_MUTEX.lock().unwrap();
    if Environment::is_local() {
        if let Some((id, client)) = OAuthTestClient::ClientCredentials.graph() {
            let result = client
                .v1()
                .drive(id.as_str())
                .check_out_item(":/test_check_out_document.docx:")
                .send();

            TestTools::assert_success(&result, "check_out");

            thread::sleep(Duration::from_secs(2));
            let result = client
                .v1()
                .drive(id.as_str())
                .check_in_item(
                    ":/test_check_out_document.docx:",
                    &serde_json::json!({
                        "comment": "test check in",
                    }),
                )
                .send();

            TestTools::assert_success(&result, "check_in");
        }
    }
}

#[test]
fn drive_download() {
    let _lock = DRIVE_THROTTLE_MUTEX.lock().unwrap();
    if let Some((id, client)) = OAuthTestClient::ClientCredentials.graph() {
        let file_location = "./test_files/test_document.docx";
        let mut clean_up = CleanUp::new(|| {
            let path = Path::new(file_location);
            if path.exists() {
                std::fs::remove_file(path).unwrap();
            }
        });

        clean_up.rm_files(file_location.into());

        let download = client
            .v1()
            .drive(id.as_str())
            .download(":/test_document.docx:", "./test_files");

        let req: GraphResult<PathBuf> = download.send();

        if let Ok(path_buf) = req {
            assert!(path_buf.exists());
        } else if let Err(e) = req {
            panic!("Request Error. Method: drive check_out. Error: {:#?}", e);
        }
    }
}

#[test]
fn drive_download_format() {
    let _lock = DRIVE_THROTTLE_MUTEX.lock().unwrap();
    if Environment::is_local() {
        if let Some((id, client)) = OAuthTestClient::ClientCredentials.graph() {
            let file_location = "./test_files/test_document.pdf";
            let mut clean_up = CleanUp::new(|| {
                let path = Path::new(file_location);
                if path.exists() {
                    std::fs::remove_file(path).unwrap();
                }
            });

            clean_up.rm_files(file_location.into());

            let download = client
                .v1()
                .drive(id.as_str())
                .download(":/test_document.docx:", "./test_files");

            download.format("pdf");
            download.rename(OsString::from("test_document.pdf"));
            let req: GraphResult<PathBuf> = download.send();

            if let Ok(path_buf) = req {
                assert!(path_buf.exists());
                assert_eq!(path_buf.extension(), Some(OsStr::new("pdf")));
                assert_eq!(path_buf.file_name(), Some(OsStr::new("test_document.pdf")));
            } else if let Err(e) = req {
                panic!("Request Error. Method: drive check_out. Error: {:#?}", e);
            }
        }
    }
}

#[test]
fn drive_update() {
    let _lock = DRIVE_THROTTLE_MUTEX.lock().unwrap();
    if let Some((id, client)) = OAuthTestClient::ClientCredentials.graph() {
        let req = client
            .v1()
            .drive(id.as_str())
            .update_items(
                ":/update_test_document.docx:",
                &serde_json::json!({
                    "name": "update_test.docx"
                }),
            )
            .send();

        if let Ok(response) = req {
            assert_eq!(response.body()["name"].as_str(), Some("update_test.docx"));
            thread::sleep(Duration::from_secs(2));

            let req = client
                .v1()
                .drive(id.as_str())
                .update_items(
                    ":/update_test.docx:",
                    &serde_json::json!({
                        "name": "update_test_document.docx"
                    }),
                )
                .send();

            if let Ok(response) = req {
                assert_eq!(
                    response.body()["name"].as_str(),
                    Some("update_test_document.docx")
                );
            } else if let Err(e) = req {
                panic!("Request Error. Method: drive update. Error: {:#?}", e);
            }
        } else if let Err(e) = req {
            panic!("Request Error. Method: drive check_out. Error: {:#?}", e);
        }
    }
}

#[test]
fn drive_upload_new_and_replace_and_delete() {
    let _lock = DRIVE_THROTTLE_MUTEX.lock().unwrap();
    if let Some((id, client)) = OAuthTestClient::ClientCredentials.graph() {
        let upload_res = client
            .v1()
            .drive(id.as_str())
            .upload_new(
                ":/test_upload_file.txt:",
                "./test_files/test_upload_file.txt",
            )
            .send();

        if let Ok(value) = upload_res {
            assert!(value.body()["id"].as_str().is_some());
            let item_id = value.body()["id"].as_str().unwrap();

            let mut file = OpenOptions::new()
                .write(true)
                .open("./test_files/test_upload_file.txt")
                .unwrap();

            file.write_all("Test Update File".as_bytes()).unwrap();
            file.sync_all().unwrap();

            thread::sleep(Duration::from_secs(2));
            let upload_replace = client
                .v1()
                .drive(id.as_str())
                .upload_replace(item_id, "./test_files/test_upload_file.txt")
                .send();

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
            let delete_res = client.v1().drive(id.as_str()).delete_items(item_id).send();

            if let Ok(response) = delete_res {
                assert!(
                    response.status() == 200 ||
                        response.status() == 201 ||
                        response.status() == 204
                );
            } else if let Err(e) = delete_res {
                panic!("Request Error. Method: drive delete. Error: {:#?}", e);
            }
        } else if let Err(e) = upload_res {
            panic!("Request Error. Method: drive upload. Error: {:#?}", e);
        }
    }
}

#[test]
fn drive_upload_session() {
    let _lock = DRIVE_THROTTLE_MUTEX.lock().unwrap();
    if let Some((id, client)) = OAuthTestClient::ClientCredentials.graph() {
        let upload = serde_json::json!({
            "@microsoft.graph.conflictBehavior": Some("fail".to_string())
        });

        let session = client
            .v1()
            .user(id.as_str())
            .drive()
            .create_upload_session(
                ":/upload_session_file.txt:",
                "./test_files/upload_session_file.txt",
                &upload,
            )
            .send();

        if let Ok(mut session) = session {
            let cancel_request = session.cancel();

            for next in session.into_iter() {
                match next {
                    Ok(NextSession::Next(response)) => {
                        assert!(!GraphError::is_error(response.status()));
                    },
                    Ok(NextSession::Done(response)) => {
                        assert!(!GraphError::is_error(response.status()));
                        let drive_item = response.body();
                        let drive_item_id =
                            drive_item["id"].as_str().unwrap_or_default().to_string();
                        thread::sleep(Duration::from_secs(3));

                        let delete_res = client
                            .v1()
                            .user(id.as_str())
                            .drive()
                            .delete_items(drive_item_id.as_str())
                            .send();

                        if let Ok(response) = delete_res {
                            assert!(
                                response.status() == 200 ||
                                    response.status() == 201 ||
                                    response.status() == 204
                            );
                        } else if let Err(e) = delete_res {
                            panic!("Request error. Upload session new. Error: {:#?}", e);
                        }
                        break;
                    },
                    Err(e) => {
                        let _ = cancel_request.send().unwrap();
                        panic!("Request error. Upload session new. Error: {:#?}", e);
                    },
                }
            }
        } else if let Err(e) = session {
            panic!("Request error. Upload session new. Error: {:#?}", e);
        }
    }
}

#[test]
pub fn get_file_from_encoded_folder_name() {
    let _lock = DRIVE_THROTTLE_MUTEX.lock().unwrap();
    if let Some((id, client)) = OAuthTestClient::ClientCredentials.graph() {
        let result = client
            .v1()
            .drive(&id)
            .get_items(":/encoding_test_files/spaced folder/test.txt:")
            .send();

        TestTools::assert_success(&result, "get_item (from percent encoded folder)");
    }
}

// Requests with /drive path (not selecting a specific drive with an id).

#[test]
pub fn get_drive_base() {
    let _lock = DRIVE_THROTTLE_MUTEX.lock().unwrap();
    if let Some((_id, client)) = OAuthTestClient::ClientCredentials.graph() {
        let result = client.v1().drives().get_drive().send();

        if let Ok(response) = result {
            assert!(
                response.status() == 200 || response.status() == 201 || response.status() == 204
            );
            let odata_context = response.body()["@odata.context"].as_str().clone().unwrap();
            assert_eq!(
                "https://graph.microsoft.com/v1.0/$metadata#drives/$entity",
                odata_context
            );
        } else if let Err(e) = result {
            panic!("Request error. DriveRequest GetDrive. Error: {:#?}", e);
        }
    }
}
