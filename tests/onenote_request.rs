use graph_rs::prelude::*;
use test_tools::oauthrequest::OAuthRequest;
use std::error::Error;

#[test]
fn list_get_notebooks_and_sections() {
    OAuthRequest::access_token_fn(|t| {
        if let Some((id, bearer)) = t {
            let client = Graph::new(bearer.as_str());

            let notebooks = client.v1()
                .users(id.as_str())
                .onenote()
                .notebooks()
                .list()
                .value();

            if let Ok(collection) = notebooks {
                let name = collection.value()["value"][0]["displayName"].as_str().unwrap();
                assert_eq!(name, "TestNotebook");

                let notebook_id = collection.value()["value"][0]["id"].as_str().unwrap();
                let get_notebook = client.v1()
                    .users(id.as_str())
                    .onenote()
                    .notebooks()
                    .get(notebook_id)
                    .value();

                if let Ok(notebook) = get_notebook {
                    let notebook_name = notebook.value()["displayName"].as_str().unwrap();
                    assert_eq!("TestNotebook", notebook_name);
                } else if let Err(e) = get_notebook {
                    panic!("Request error. Method: onenote notebooks get. Error: {:#?}", e.description());
                }

                let sections = client.v1()
                    .users(id.as_str())
                    .onenote()
                    .notebooks()
                    .list_sections(notebook_id)
                    .value();

                if let Ok(collection) = sections {
                    let section_name = collection.value()["value"][0]["displayName"].as_str().unwrap();
                    assert_eq!("TestSection", section_name);
                } else if let Err(e) = sections {
                    panic!("Request error. Method: onenote notebooks list sections. Error: {:#?}", e.description());
                }

            } else if let Err(e) = notebooks {
                panic!("Request error. Method: onenote notebooks list. Error: {:#?}", e.description());
            }
        }});
}
