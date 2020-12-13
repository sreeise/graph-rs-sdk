use graph_error::GraphResult;
use graph_rs_sdk::prelude::*;

static ACCESS_TOKEN: &str = "<ACCESS_TOKEN>";

static GROUP_ID: &str = "<GROUP_ID>";

fn main() {}

#[allow(dead_code)]
fn get_or_list_groups() -> GraphResult<()> {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client.v1().group(GROUP_ID).get_group().send()?;

    println!("Group response: {:#?}", response);

    let response = client
        .v1()
        .groups() // Won't be used
        .list_group()
        .send()?;

    println!("List groups response: {:#?}", response);

    Ok(())
}

#[allow(dead_code)]
fn add_favorites_members_owners() -> GraphResult<()> {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client.v1().group(GROUP_ID).add_favorite().send()?;

    println!("Add favorite response: {:#?}", response);

    let directory_object_id = "<DIRECTORY_OBJECT_ID>";
    let response = client
        .v1()
        .group(GROUP_ID)
        .add_member(&serde_json::json!({
            "@odata.id":
                format!(
                    "https://graph.microsoft.com/v1.0/directoryObjects/{}",
                    directory_object_id
                )
        }))
        .send()?;

    println!("Add member response: {:#?}", response);

    let user_id = "<USER_ID>";
    let response = client
        .v1()
        .group(GROUP_ID)
        .add_owner(&serde_json::json!({
            "@odata.id": format!("https://graph.microsoft.com/v1.0/users/{}", user_id)
        }))
        .send()?;

    println!("Add owner response: {:#?}", response);

    Ok(())
}

#[allow(dead_code)]
fn create_update_delete_group() -> GraphResult<()> {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client
        .v1()
        .groups() // Won't be used.
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
        .send()?;

    println!("Create group: {:#?}", response);

    let response = client
        .v1()
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
        .send()?;

    println!("Update group: {:#?}", response);

    let response = client.v1().group(GROUP_ID).delete_group().send()?;

    println!("Delete group: {:#?}", response);

    Ok(())
}

#[allow(dead_code)]
fn list_methods() -> GraphResult<()> {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client.v1().group(GROUP_ID).list_members().send()?;

    println!("{:#?}", response);

    let response = client.v1().group(GROUP_ID).list_member_of().send()?;

    println!("{:#?}", response);

    let response = client
        .v1()
        .group(GROUP_ID)
        .list_transitive_members()
        .send()?;

    println!("{:#?}", response);

    let response = client
        .v1()
        .group(GROUP_ID)
        .list_transitive_member_of()
        .send()?;

    println!("{:#?}", response);

    let response = client.v1().group(GROUP_ID).list_owners().send()?;

    println!("{:#?}", response);

    let response = client.v1().group(GROUP_ID).list_photos().send()?;

    println!("{:#?}", response);

    Ok(())
}

#[allow(dead_code)]
fn remove_methods() -> GraphResult<()> {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client.v1().group(GROUP_ID).remove_favorite().send()?;

    println!("{:#?}", response);

    let member_id = "<MEMBER_ID>";
    let response = client
        .v1()
        .group(GROUP_ID)
        .remove_member(member_id)
        .send()?;

    println!("{:#?}", response);

    let owner_id = "<OWNER_ID>";
    let response = client.v1().group(GROUP_ID).remove_owner(owner_id).send()?;

    println!("{:#?}", response);

    Ok(())
}

static GROUP_LIFECYCLE_POLICY_ID: &str = "<GROUP_LIFECYCLE_POLICY_ID>";

#[allow(dead_code)]
fn lifecycle_policies() -> GraphResult<()> {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client
        .v1()
        .group_lifecycle_policies() // Won't be used.
        .list_group_lifecycle_policy()
        .send()?;

    println!("{:#?}", response);

    let response = client
        .v1()
        .group_lifecycle_policies()
        .get_group_lifecycle_policy(GROUP_LIFECYCLE_POLICY_ID)
        .send()?;

    println!("{:#?}", response);

    let response = client
        .v1()
        .group_lifecycle_policies() // Won't be used.
        .create_group_lifecycle_policy(&serde_json::json!({
            "groupLifetimeInDays": 100,
            "managedGroupTypes": "Selected",
            "alternateNotificationEmails": "admin@contoso.com"
        }))
        .send()?;

    println!("{:#?}", response);

    let response = client
        .v1()
        .group_lifecycle_policies()
        .update_group_lifecycle_policy(
            GROUP_LIFECYCLE_POLICY_ID,
            &serde_json::json!({
                "groupLifetimeInDays": 100,
                "managedGroupTypes": "Selected",
                "alternateNotificationEmails": "admin@contoso.com"
            }),
        )
        .send()?;

    println!("{:#?}", response);

    let response = client
        .v1()
        .group_lifecycle_policies()
        .add_group(
            GROUP_LIFECYCLE_POLICY_ID,
            &serde_json::json!({
                "groupId": "ffffffff-ffff-ffff-ffff-ffffffffffff"
            }),
        )
        .send()?;

    println!("{:#?}", response);

    let response = client
        .v1()
        .group_lifecycle_policies()
        .remove_group(
            GROUP_LIFECYCLE_POLICY_ID,
            &serde_json::json!({
                "groupId": "ffffffff-ffff-ffff-ffff-ffffffffffff"
            }),
        )
        .send()?;

    println!("{:#?}", response);

    Ok(())
}
