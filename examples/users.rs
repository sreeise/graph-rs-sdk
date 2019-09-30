use graph_rs::prelude::*;

// For more info on users see: https://docs.microsoft.com/en-us/graph/api/resources/user?view=graph-rest-1.0

// Work or school accounts must have the following permissions: User.ReadBasic.All,
// User.Read.All, User.ReadWrite.All, Directory.Read.All, Directory.ReadWrite.All,
// Directory.AccessAsUser.All

// Applications must have the following permissions: User.Read.All,
// User.ReadWrite.All, Directory.Read.All, Directory.ReadWrite.All

// Delegate (Personal microsoft accounts) are not supported in the Graph API.

static USER_ID: &str = "USER_ID";

fn main() {
    list_users();
    get_user();
    create_user();
    update_user();
    delete_user();
}

fn list_users() {
    let client = Graph::new("ACCESS_TOKEN");

    let collection = client.v1().users(USER_ID).list().send().unwrap();
    println!("{:#?}", collection.value());
}

fn get_user() {
    let client = Graph::new("ACCESS_TOKEN");

    let user = client.v1().users(USER_ID).get().send().unwrap();

    println!("{:#?}", user.value());
}

fn create_user() {
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

    let user: serde_json::Value = client.v1().users("").create(&user).json().unwrap();

    println!("{:#?}", user);
}

// Create a default user and update only the properties that
// need to be updated. Properties that are left alone
// will stay the same.
fn update_user() {
    let client = Graph::new("ACCESS_TOKEN");

    let user = serde_json::json!({
        "business_phones": ["888-888-8888"]
    });

    let response = client.v1().users(USER_ID).update(&user).send().unwrap();

    println!("{:#?}", response);
}

fn delete_user() {
    let client = Graph::new("ACCESS_TOKEN");

    let response = client.v1().users(USER_ID).delete().send().unwrap();

    println!("{:#?}", response);
}
