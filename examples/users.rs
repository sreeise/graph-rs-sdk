use graph_rs_sdk::*;

// For more info on users see: https://docs.microsoft.com/en-us/graph/api/resources/user?view=graph-rest-1.0

// Work or school accounts must have the following permissions: User.ReadBasic.All,
// User.Read.All, User.ReadWrite.All, Directory.Read.All, Directory.ReadWrite.All,
// Directory.AccessAsUser.All

// Applications must have the following permissions: User.Read.All,
// User.ReadWrite.All, Directory.Read.All, Directory.ReadWrite.All

// Delegate (Personal microsoft accounts) are not supported in the Graph API.

static USER_ID: &str = "USER_ID";

#[tokio::main]
async fn main() {
    list_users().await.unwrap();
    get_user().await.unwrap();
    create_user().await;
    update_user().await;
    delete_user().await;
}

async fn list_users() -> GraphResult<()> {
    let client = Graph::new("ACCESS_TOKEN");

    let response = client.users().list_user().send().await?;

    println!("{response:#?}");

    let body: serde_json::Value = response.json().await?;
    println!("{body:#?}");

    Ok(())
}

async fn get_user() -> GraphResult<()> {
    let client = Graph::new("ACCESS_TOKEN");

    let response = client.user(USER_ID).get_user().send().await?;

    println!("{:#?}", &response);

    let body: serde_json::Value = response.json().await?;
    println!("{body:#?}");

    Ok(())
}

async fn create_user() {
    let client = Graph::new("ACCESS_TOKEN");

    // Create a password profile. Change the password below
    // to one that meets the Microsoft password requirements.
    let password_profile = serde_json::json!({
        "force_change_password_next_sign_in": false,
        "force_change_password_next_sign_in_with_mfa": false,
        "password": "PASSWORD",
    });

    // Create a user. The fields below are the minimum required values.
    let user = serde_json::json!({
        "account_enabled": true,
        "display_name": "FirstName LastName",
        "mail_nickname": "user",
        "password_profile": password_profile,
        "user_principal_name": "user@domain.com"
    });

    let response = client.users().create_user(&user).send().await.unwrap();

    println!("{:#?}", &response);
    let body: serde_json::Value = response.json().await.unwrap();
    println!("{body:#?}");
}

// Create a default user and update only the properties that
// need to be updated. Properties that are left alone
// will stay the same.
async fn update_user() {
    let client = Graph::new("ACCESS_TOKEN");

    let user = serde_json::json!({
        "business_phones": ["888-888-8888"]
    });

    let response = client
        .user(USER_ID)
        .update_user(&user)
        .send()
        .await
        .unwrap();

    println!("{response:#?}");
}

async fn delete_user() {
    let client = Graph::new("ACCESS_TOKEN");

    let response = client.user(USER_ID).delete_user().send().await.unwrap();

    println!("{response:#?}");
}
