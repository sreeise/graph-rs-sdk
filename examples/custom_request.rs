#![allow(dead_code)]

use graph_rs_sdk::http::{BodyRead, Method};
use graph_rs_sdk::prelude::*;

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";

static USER_ID: &str = "USER_ID";

// You can create create a custom request through the client
// by providing the http method, path, query, body, etc.

#[tokio::main]
async fn main() -> GraphResult<()> {
    let client = Graph::new(ACCESS_TOKEN);

    let user = serde_json::json!({
        "business_phones": ["888-888-8888"]
    });

    let _ = client
        .custom(
            Method::PATCH,
            Some(BodyRead::from_serialize(&user).unwrap()),
        )
        .extend_path(&["users", USER_ID])
        .send()
        .await?;

    // Is the same as calling the update users method;
    let _ = client.user(USER_ID).update_user(&user).send().await?;

    Ok(())
}

async fn list_users() -> GraphResult<()> {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client
        .custom(Method::GET, None)
        .extend_path(&["users"])
        .send()
        .await?;

    println!("{response:#?}");

    let body: serde_json::Value = response.json().await?;
    println!("{body:#?}");

    Ok(())
}
