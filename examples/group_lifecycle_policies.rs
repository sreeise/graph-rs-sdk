use graph_error::GraphResult;
use graph_rs_sdk::prelude::Graph;

static ACCESS_TOKEN: &str = "<ACCESS_TOKEN>";

static GROUP_LIFECYCLE_POLICY_ID: &str = "<GROUP_ID>";

#[tokio::main]
async fn main() -> GraphResult<()> {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client
        .group_lifecycle_policies() // Won't be used.
        .list_group_lifecycle_policy()
        .send()
        .await;

    println!("{response:#?}");

    let response = client
        .group_lifecycle_policy(GROUP_LIFECYCLE_POLICY_ID)
        .get_group_lifecycle_policy()
        .send()
        .await?;

    println!("{response:#?}");

    let response = client
        .group_lifecycle_policies() // Won't be used.
        .create_group_lifecycle_policy(&serde_json::json!({
            "groupLifetimeInDays": 100,
            "managedGroupTypes": "Selected",
            "alternateNotificationEmails": "admin@contoso.com"
        }))
        .send()
        .await?;

    println!("{response:#?}");

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

    let response = client
        .group_lifecycle_policy(GROUP_LIFECYCLE_POLICY_ID)
        .add_group(&serde_json::json!({
            "groupId": "ffffffff-ffff-ffff-ffff-ffffffffffff"
        }))
        .send()
        .await?;

    println!("{response:#?}");

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
