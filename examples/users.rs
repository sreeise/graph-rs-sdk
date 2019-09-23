use graph_rs::prelude::*;
use graph_rs_types::complextypes::PasswordProfile;
use graph_rs_types::entitytypes::User;

// For more info on users see: https://docs.microsoft.com/en-us/graph/api/resources/user?view=graph-rest-1.0

// Work or school accounts must have the following permissions: User.ReadBasic.All,
// User.Read.All, User.ReadWrite.All, Directory.Read.All, Directory.ReadWrite.All,
// Directory.AccessAsUser.All

// Applications must have the following permissions: User.Read.All,
// User.ReadWrite.All, Directory.Read.All, Directory.ReadWrite.All

// Delegate (Personal microsoft accounts) are not supported in the Graph API.

fn main() {
    list_users();
    get_user();
    create_user();
    update_user();
    delete_user();
}

fn list_users() {
    let client = Graph::new("ACCESS_TOKEN");

    let collection: GraphResponse<Collection<User>> = client.v1().user().list().send().unwrap();
    println!("{:#?}", collection.value());
}

static USER_ID: &str = "USER_ID";

fn get_user() {
    let client = Graph::new("ACCESS_TOKEN");

    let user: GraphResponse<User> = client.v1().user().get().by_id(USER_ID).send().unwrap();

    println!("{:#?}", user.value());
}

fn create_user() {
    let client = Graph::new("ACCESS_TOKEN");

    // Create the users password profile.
    let mut password_profile: PasswordProfile = PasswordProfile::default();
    password_profile.force_change_password_next_sign_in = Some(false);
    password_profile.force_change_password_next_sign_in_with_mfa = Some(false);
    password_profile.password = Some("PASSWORD".into());

    // Create a user. The fields below are the minimum required values.
    let mut user: User = User::default();
    user.account_enabled = Some(true);
    user.display_name = Some("FirstName LastName".into());
    user.mail_nickname = Some("user".into());
    user.password_profile = Some(password_profile);
    user.user_principal_name = Some("user@domain.com".into());

    let user: GraphResponse<User> = client.v1().user().create(&user).send().unwrap();

    println!("{:#?}", user.value());
}

// Create a default user and update only the properties that
// need to be updated. Properties that are left alone
// will stay the same.
fn update_user() {
    let client = Graph::new("ACCESS_TOKEN");

    let mut user: User = User::default();
    user.business_phones = Some(vec!["888-888-8888".to_string()]);

    let response = client
        .v1()
        .user()
        .update(&user)
        .by_id(USER_ID)
        .send()
        .unwrap();

    println!("{:#?}", response);
}

fn delete_user() {
    let client = Graph::new("ACCESS_TOKEN");

    let response = client.v1().user().delete().by_id(USER_ID).send().unwrap();

    println!("{:#?}", response);
}
