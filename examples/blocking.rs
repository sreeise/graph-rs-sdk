#![allow(dead_code)]

use graph_rs_sdk::http::ResponseBlockingExt;
use graph_rs_sdk::prelude::*;

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";

fn main() -> GraphResult<()> {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client.users().list_user().into_blocking().send()?;

    println!("{response:#?}");

    let body: serde_json::Value = response.json()?;
    println!("{body:#?}");

    Ok(())
}

fn paging_json() -> GraphResult<()> {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client
        .users()
        .delta()
        .into_blocking()
        .paging()
        .json::<serde_json::Value>()?;

    println!("{response:#?}");

    Ok(())
}

fn paging_channel() -> GraphResult<()> {
    let client = Graph::new(ACCESS_TOKEN);

    let receiver = client
        .users()
        .delta()
        .into_blocking()
        .paging()
        .channel::<serde_json::Value>()?;

    while let Some(result) = receiver.recv()? {
        let response = result?;
        println!("{response:#?}");
    }

    Ok(())
}

static USER_ID: &str = "USER_ID";

// The path where you wan to place the file in OneDrive
// including the file name. For the root folder just
// put the file name here like so: :/file.ext:
static ONEDRIVE_FILE: &str = ":/file.txt:";

static LOCAL_FILE: &str = "./file.txt";

fn upload_session_channel() -> GraphResult<()> {
    let client = Graph::new(ACCESS_TOKEN);

    let upload = serde_json::json!({
        "@microsoft.graph.conflictBehavior": Some("fail".to_string())
    });

    let response = client
        .drive(USER_ID)
        .item_by_path(ONEDRIVE_FILE)
        .create_upload_session(&upload)
        .into_blocking()
        .send()?;

    let file = std::fs::File::open(LOCAL_FILE)?;

    let mut upload_session_task = response.into_upload_session(file)?;
    let receiver = upload_session_task.channel()?;

    for result in receiver.iter() {
        let response = result?;
        println!("{response:#?}");
        let body: serde_json::Value = response.json().unwrap();
        println!("{body:#?}");
    }

    Ok(())
}

/// Use [`while let Some(result) = upload_session.next()`] or [`for result in upload_session {}`]
/// when using Iterator impl.
/// DO NOT use [`for result in upload_session.next()`] when using Iterator impl.
fn upload_session_iter() -> GraphResult<()> {
    let client = Graph::new(ACCESS_TOKEN);

    let upload = serde_json::json!({
        "@microsoft.graph.conflictBehavior": Some("fail".to_string())
    });

    let response = client
        .drive(USER_ID)
        .item_by_path(ONEDRIVE_FILE)
        .create_upload_session(&upload)
        .into_blocking()
        .send()?;

    let file = std::fs::File::open(LOCAL_FILE)?;

    let upload_session_task = response.into_upload_session(file)?;

    for result in upload_session_task {
        let response = result?;
        println!("{:#?}", response);
        let body: serde_json::Value = response.json().unwrap();
        println!("{:#?}", body);
    }

    Ok(())
}
