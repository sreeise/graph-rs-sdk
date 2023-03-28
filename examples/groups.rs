use graph_error::GraphResult;
use graph_rs_sdk::prelude::*;

static ACCESS_TOKEN: &str = "<ACCESS_TOKEN>";

static GROUP_ID: &str = "<GROUP_ID>";

#[tokio::main]
async fn main() {}

#[allow(dead_code)]
async fn get_groups() -> GraphResult<()> {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client.group(GROUP_ID).get_group().send().await?;

    println!("{response:#?}");

    Ok(())
}

#[allow(dead_code)]
async fn list_groups() -> GraphResult<()> {
    let client = Graph::new(ACCESS_TOKEN);

    let response_handler = client.groups().list_group();
    println!("{:#?}", response.err());

    println!("{response:#?}");

    Ok(())
}

#[allow(dead_code)]
async fn create_update_delete_group() -> GraphResult<()> {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client
        .groups()
        .create_group(&serde_json::json!({
            "description": "Self help community for library",
            "displayName": "Library Assist",
            "groupTypes": [
                "Unified"
            ],
            "mailEnabled": true,
            "mailNickname": "library",
            "securityEnabled": false
        }))
        .send()
        .await?;

    println!("{response:#?}");

    let response = client
        .group(GROUP_ID)
        .update_group(&serde_json::json!({
            "description": "description-value",
            "displayName": "displayName-value",
            "groupTypes": [
                "groupTypes-value"
            ],
            "mail": "mail-value",
            "mailEnabled": true,
            "mailNickname": "mailNickname-value"
            }
        ))
        .send()
        .await?;

    println!("{response:#?}");

    Ok(())
}
