use graph_rs_sdk::oauth::{
    DeviceCodeCredential, DeviceCodeCredentialBuilder, PublicClientApplication, Token,
    TokenCredentialExecutor,
};
use graph_rs_sdk::GraphResult;
use std::time::Duration;
use warp::hyper::body::HttpBody;

// https://learn.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-device-code

static CLIENT_ID: &str = "<CLIENT_ID>";
static TENANT: &str = "<TENANT>";

// Make the call to get a device code from the user.

// Poll the device code endpoint to get the code and a url that the user must
// go to in order to enter the code. Polling will continue until either the user
// has entered the and an access token is returned or an error happens.
fn poll_device_code() {
    let mut device_executor = PublicClientApplication::builder(CLIENT_ID)
        .with_device_code_authorization_executor()
        .with_scope(vec!["User.Read"])
        .poll()
        .unwrap();

    while let Ok(response) = device_executor.recv() {
        println!("{:#?}", response);
    }
}

fn get_token(device_code: &str) {
    let mut public_client = PublicClientApplication::builder(CLIENT_ID)
        .with_device_code(device_code)
        .with_scope(["User.Read"])
        .with_tenant(TENANT)
        .build();

    let response = public_client.execute().unwrap();
    println!("{:#?}", response);

    let body: Token = response.json().unwrap();

    println!("{:#?}", body);
}
