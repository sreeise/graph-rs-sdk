use graph_rs::error::{GraphFailure, GraphRsError};
use graph_rs::prelude::*;
use std::thread;
use std::time::Duration;
use test_tools::oauthrequest::OAuthRequest;
use test_tools::oauthrequest::THROTTLE_MUTEX;

#[test]
fn list_get_notebooks_and_sections() {
    if OAuthRequest::is_appveyor() {
        return;
    }

    let _lock = THROTTLE_MUTEX.lock().unwrap();
    OAuthRequest::access_token_fn(|t| {
        if let Some((id, bearer)) = t {
            let client = Graph::new(bearer.as_str());

            let notebooks = client
                .v1()
                .users(id.as_str())
                .onenote()
                .notebooks()
                .list()
                .send();

            println!("{:#?}", notebooks);

            if let Ok(collection) = notebooks {
                let vec = collection.value().value().unwrap();

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
                    .v1()
                    .users(id.as_str())
                    .onenote()
                    .notebooks()
                    .get(notebook_id.as_str())
                    .send();

                if let Ok(notebook) = get_notebook {
                    assert_eq!("TestNotebook", notebook.value()["displayName"].as_str().unwrap());
                } else if let Err(e) = get_notebook {
                    panic!(
                        "Request error. Method: onenote notebooks get. Error: {:#?}",
                        e
                    );
                }

                let sections = client
                    .v1()
                    .users(id.as_str())
                    .onenote()
                    .notebooks()
                    .list_sections(notebook_id.as_str())
                    .send();

                if let Ok(collection) = sections {
                    let vec = collection.into_value().into_inner();
                    let section_name = vec[0]["displayName"].as_str().unwrap();
                    assert_eq!("TestSection", section_name);
                } else if let Err(e) = sections {
                    panic!(
                        "Request error. Method: onenote notebooks list sections. Error: {:#?}",
                        e
                    );
                }
            } else if let Err(e) = notebooks {
                panic!(
                    "Request error. Method: onenote notebooks list. Error: {:#?}",
                    e
                );
            }
        }
    });
}

#[test]
fn create_delete_page() {
    if OAuthRequest::is_appveyor() {
        return;
    }

    let _lock = THROTTLE_MUTEX.lock().unwrap();
    OAuthRequest::access_token_fn(|t| {
        if let Some((id, token)) = t {
            let client = Graph::new(token.as_str());

            let res = client
                .v1()
                .users(&id)
                .onenote()
                .create_page("./test_files/onenotepage.html")
                .send();

            if let Ok(page) = res {
                let page_id = page.value()["id"].as_str().unwrap();

                thread::sleep(Duration::from_secs(5));
                let delete_res = client
                    .v1()
                    .users(&id)
                    .onenote()
                    .pages()
                    .delete(page_id)
                    .send();

                if let Err(e) = delete_res {
                    panic!(
                        "Request error. Method onenote pages delete page: Error: {:#?}",
                        e
                    );
                }
            } else if let Err(e) = res {
                panic!("Request error. Method onenote create page. Error: {:#?}", e);
            }
        }
    });
}

#[test]
fn onenote_create_page_invalid_ext() {
    let client = Graph::new("");

    let response = client
        .v1()
        .me()
        .onenote()
        .create_page("./test_files/test_upload_file.txt")
        .send();

    if let Err(err) = response {
        match err {
            GraphFailure::GraphRsError(err) => match err {
                GraphRsError::InvalidFileExtension { requires, found } => {
                    assert_eq!("html", requires);
                    assert_eq!("txt", found);
                },
                _ => {
                    panic!("Unexpected error thrown: {}", err);
                },
            },
            _ => {
                panic!("Unexpected error thrown: {}", err);
            },
        }
    } else if let Ok(_) = response {
        panic!(
            "Unexpected successful response. GraphRsError::InvalidFileExtension should have thrown"
        );
    }
}

#[test]
fn onenote_sections_create_page_invalid_ext() {
    let client = Graph::new("");

    let response = client
        .v1()
        .me()
        .onenote()
        .sections()
        .create_page("id", "./test_files/test_upload_file.txt")
        .send();

    if let Err(err) = response {
        match err {
            GraphFailure::GraphRsError(err) => match err {
                GraphRsError::InvalidFileExtension { requires, found } => {
                    assert_eq!("html", requires);
                    assert_eq!("txt", found);
                },
                _ => {
                    panic!("Unexpected error thrown: {}", err);
                },
            },
            _ => {
                panic!("Unexpected error thrown: {}", err);
            },
        }
    } else if let Ok(_) = response {
        panic!(
            "Unexpected successful response. GraphRsError::InvalidFileExtension should have thrown"
        );
    }
}
