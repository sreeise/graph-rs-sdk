use graph_rs::prelude::*;
use std::error::Error;
use test_tools::oauthrequest::OAuthRequest;
use test_tools::oauthrequest::THROTTLE_MUTEX;

#[test]
fn list_get_notebooks_and_sections() {
    if OAuthRequest::is_appveyor() {
        return;
    }

    OAuthRequest::access_token_fn(|t| {
        if let Some((id, bearer)) = t {
            let _lock = THROTTLE_MUTEX.lock().unwrap();
            let client = Graph::new(bearer.as_str());

            let notebooks = client
                .v1()
                .users(id.as_str())
                .onenote()
                .notebooks()
                .list()
                .value();

            if let Ok(collection) = notebooks {
                let vec = collection.value()["value"].as_array().unwrap();

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
                    .value();

                if let Ok(notebook) = get_notebook {
                    assert_eq!(
                        "TestNotebook",
                        notebook.value()["displayName"].as_str().unwrap()
                    );
                } else if let Err(e) = get_notebook {
                    panic!(
                        "Request error. Method: onenote notebooks get. Error: {:#?}",
                        e.description()
                    );
                }

                let sections = client
                    .v1()
                    .users(id.as_str())
                    .onenote()
                    .notebooks()
                    .list_sections(notebook_id.as_str())
                    .value();

                if let Ok(collection) = sections {
                    let section_name = collection.value()["value"][0]["displayName"]
                        .as_str()
                        .unwrap();
                    assert_eq!("TestSection", section_name);
                } else if let Err(e) = sections {
                    panic!(
                        "Request error. Method: onenote notebooks list sections. Error: {:#?}",
                        e.description()
                    );
                }
            } else if let Err(e) = notebooks {
                panic!(
                    "Request error. Method: onenote notebooks list. Error: {:#?}",
                    e.description()
                );
            }
        }
    });
}
