use graph_oauth::identity::{
    DeviceCodeCredential, PublicClientApplication, TokenCredentialExecutor,
};
use graph_oauth::oauth::DeviceCodeCredentialBuilder;
use graph_rs_sdk::oauth::{MsalTokenResponse, OAuthSerializer};
use graph_rs_sdk::GraphResult;
use std::time::Duration;
use warp::hyper::body::HttpBody;

// https://learn.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-device-code

static CLIENT_ID: &str = "<CLIENT_ID>";
static TENANT: &str = "<TENANT>";

// Make the call to get a device code from the user.
fn get_auth_call_for_device_code() {
    let mut public_client = PublicClientApplication::builder(CLIENT_ID)
        .with_device_code_builder()
        .with_scope(["User.Read"])
        .with_tenant(TENANT);
}

fn get_token(device_code: &str) {
    let mut public_client = PublicClientApplication::builder(CLIENT_ID)
        .with_device_code(device_code)
        .with_scope(["User.Read"])
        .with_tenant(TENANT)
        .build();

    let response = public_client.execute().unwrap();
    println!("{:#?}", response);

    let body: MsalTokenResponse = response.json().unwrap();

    println!("{:#?}", body);
}
