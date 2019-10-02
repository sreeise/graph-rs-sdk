use graph_error::GraphError;
use graph_rs::prelude::*;
use std::collections::HashMap;
use std::time::Duration;
use std::{env, thread};
use test_tools::ci::CI;

#[test]
fn drive_tests_on_ci() {
    if CI::is_travis() {
        let token = CI::request_access_token().unwrap();
        let t = token.bearer_token().clone();
        let id = env::var("TEST_APP_USER_ID").expect("Missing env USER_ID");
        get_recent(t, id.as_str());
        create_delete_folder(t, id.as_str());
    } else {
        CI::assert_not_travis();
    }
}

fn get_recent(token: &str, rid: &str) {
    let client = Graph::new(token);
    if let Ok(response) = client.v1().drives(rid).drive().recent().send() {
        assert!(response.value().len() > 0)
    } else {
        panic!("Request Error. Method: drive recent");
    }
}

#[test]
fn get_drive() {
    if CI::is_travis() {
        CI::test_credentials(|t| {
            if let Some((bearer, id)) = t {
                let client = Graph::new(bearer.as_str());
                if let Err(_) = client.v1().drives(id.as_str()).drive().root().send() {
                    panic!("Request Error. Method: drive root");
                }
            }
        });
    } else {
        CI::assert_not_travis();
    }
}

fn create_delete_folder(token: &str, rid: &str) {
    let client = Graph::new(token);
    let folder: HashMap<String, serde_json::Value> = HashMap::new();
    let create_folder_res = client
        .v1()
        .drives(rid)
        .drive()
        .create_folder(
            "",
            &serde_json::json!({
                "name": "ci_docs",
                "folder": folder,
                "@microsoft.graph.conflictBehavior": "rename"
            }),
        )
        .send();

    if let Ok(response) = create_folder_res {
        let id = response.value().id.clone().unwrap();
        thread::sleep(Duration::from_secs(2));

        let req = client.v1().drives(rid).drive().delete(id.as_str()).send();

        if let Ok(res) = req {
            assert!(!GraphError::is_error(res.response().status().as_u16()));
        } else if let Err(_) = req {
            panic!("Request error. Method: drive delete");
        }
    } else if let Err(_) = create_folder_res {
        panic!("Request error. Method: create folder");
    }
}

#[test]
fn root_children_and_list_versions() {
    if CI::is_travis() {
        CI::test_credentials(|t| {
            if let Some((bearer, id)) = t {
                let client = Graph::new(bearer.as_str());
                if let Ok(res) = client
                    .v1()
                    .drives(id.as_str())
                    .drive()
                    .root_children()
                    .send()
                {
                    let value = res.value().index(0).clone().unwrap();
                    let item_id = value.id.clone().unwrap();

                    if let Err(_) = client
                        .v1()
                        .drives(id.as_str())
                        .drive()
                        .list_versions(item_id.as_str())
                        .send()
                    {
                        panic!("Request Error. Method: list versions");
                    }
                } else {
                    panic!("Request Error. Method: drive root children");
                }
            }
        });
    } else {
        CI::assert_not_travis();
    }
}
