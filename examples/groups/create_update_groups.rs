use graph_rs_sdk::*;

static ACCESS_TOKEN: &str = "<ACCESS_TOKEN>";

static GROUP_ID: &str = "<GROUP_ID>";

pub async fn create_group() -> GraphResult<()> {
    let client = GraphClient::new(ACCESS_TOKEN);

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

    let body: serde_json::Value = response.json().await?;
    println!("{body:#?}");

    Ok(())
}

pub async fn update_group() -> GraphResult<()> {
    let client = GraphClient::new(ACCESS_TOKEN);

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
        }))
        .send()
        .await?;

    println!("{response:#?}");

    Ok(())
}
