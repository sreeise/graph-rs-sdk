use graph_rs_sdk::*;

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";

// If using the Users api:
static USER_ID: &str = "USER_ID";

pub async fn get_mailbox_settings() -> GraphResult<()> {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client
        .me()
        .mailbox_settings()
        .get_mailbox_settings()
        .send()
        .await?;

    println!("{response:#?}");

    let body: serde_json::Value = response.json().await?;
    println!("{body:#?}");

    Ok(())
}

pub async fn get_user_mailbox_settings() -> GraphResult<()> {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client
        .user(USER_ID)
        .mailbox_settings()
        .get_mailbox_settings()
        .send()
        .await?;

    println!("{response:#?}");

    let body: serde_json::Value = response.json().await?;
    println!("{body:#?}");

    Ok(())
}
