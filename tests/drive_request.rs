use graph_error::GraphError;
use graph_rs::prelude::*;
use std::collections::HashMap;
use std::time::Duration;
use std::{env, thread};
use test_tools::ci::CI;

#[test]
fn drive_tests_on_ci() {
    if let Some(token) = CI::request_access_token() {
        let t = token.1.bearer_token().clone();
        get_drive(t, token.0.as_str());
        get_recent(t, token.0.as_str());
        create_delete_folder(t, token.0.as_str());
    }
}

fn get_recent(token: &str, rid: &str) {
    let client = Graph::new(token);
    if let Err(_) = client.v1().drives(rid).drive().recent().send() {
        panic!("Request Error. Method: drive recent");
    }
}

fn get_drive(token: &str, rid: &str) {
    let client = Graph::new(token);
    if let Err(_) = client.v1().drives(rid).drive().root().send() {
        panic!("Request Error. Method: drive root");
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
    CI::test_credentials(|t| {
        if let Some((id, bearer)) = t {
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
}
