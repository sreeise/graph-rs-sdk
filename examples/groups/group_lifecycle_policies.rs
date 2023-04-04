use graph_rs_sdk::*;

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";

static GROUP_LIFECYCLE_POLICY_ID: &str = "GROUP_LIFECYCLE_POLICY_ID";

pub async fn list_group_lifecycle_policies() -> GraphResult<()> {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client
        .group_lifecycle_policies()
        .list_group_lifecycle_policy()
        .send()
        .await?;

    println!("{response:#?}");

    let body: serde_json::Value = response.json().await.unwrap();
    println!("{body:#?}");

    Ok(())
}

static GROUP_ID: &str = "<GROUP_ID>";

pub async fn list_group_lifecycle_policies_as_group() -> GraphResult<()> {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client
        .group(GROUP_ID)
        .group_lifecycle_policies()
        .list_group_lifecycle_policy()
        .send()
        .await?;

    println!("{response:#?}");

    let body: serde_json::Value = response.json().await?;
    println!("{body:#?}");

    Ok(())
}

pub async fn get_group_lifecycle_policies() -> GraphResult<()> {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client
        .group_lifecycle_policy(GROUP_LIFECYCLE_POLICY_ID)
        .get_group_lifecycle_policy()
        .send()
        .await?;

    println!("{response:#?}");

    let body: serde_json::Value = response.json().await?;
    println!("{body:#?}");

    Ok(())
}

pub async fn create_group_lifecycle_policies() -> GraphResult<()> {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client
        .group_lifecycle_policies()
        .create_group_lifecycle_policy(&serde_json::json!({
            "groupLifetimeInDays": 100,
            "managedGroupTypes": "Selected",
            "alternateNotificationEmails": "admin@contoso.com"
        }))
        .send()
        .await?;

    println!("{response:#?}");

    Ok(())
}

pub async fn update_group_lifecycle_policies() -> GraphResult<()> {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client
        .group_lifecycle_policy(GROUP_LIFECYCLE_POLICY_ID)
        .update_group_lifecycle_policy(&serde_json::json!({
            "groupLifetimeInDays": 100,
            "managedGroupTypes": "Selected",
            "alternateNotificationEmails": "admin@contoso.com"
        }))
        .send()
        .await?;

    println!("{response:#?}");

    Ok(())
}

pub async fn add_group() -> GraphResult<()> {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client
        .group_lifecycle_policy(GROUP_LIFECYCLE_POLICY_ID)
        .add_group(&serde_json::json!({
            "groupId": "ffffffff-ffff-ffff-ffff-ffffffffffff"
        }))
        .send()
        .await?;

    println!("{response:#?}");

    let body: serde_json::Value = response.json().await?;
    println!("{body:#?}");

    Ok(())
}

pub async fn remove_group() -> GraphResult<()> {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client
        .group_lifecycle_policy(GROUP_LIFECYCLE_POLICY_ID)
        .remove_group(&serde_json::json!({
            "groupId": "ffffffff-ffff-ffff-ffff-ffffffffffff"
        }))
        .send()
        .await?;

    println!("{response:#?}");

    Ok(())
}
