use graph_rs_sdk::identity::{
    ClientSecretCredential, DeviceCodeCredential, DeviceCodeCredentialBuilder,
    PublicClientApplication, Token, TokenCredentialExecutor,
};
use graph_rs_sdk::GraphResult;
use graph_rs_sdk::{identity::ConfidentialClientApplication, Graph};
use std::time::Duration;
use warp::hyper::body::HttpBody;

// https://learn.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-device-code

// Make the call to get a device code from the user.

// Poll the device code endpoint to get the code and a url that the user must
// go to in order to enter the code. Polling will continue until either the user
// has entered the code or a fatal error has occurred which causes polling to cease.
// Once a successful code has been entered the next time the device code endpoint
// is polled an access token is returned.
fn poll_device_code(client_id: &str, tenant: &str, scope: Vec<&str>) -> anyhow::Result<()> {
    let mut device_executor = PublicClientApplication::builder(client_id)
        .with_device_code_executor()
        .with_scope(scope)
        .with_tenant(tenant)
        .poll()?;

    while let Ok(response) = device_executor.recv() {
        println!("{:#?}", response);
    }

    Ok(())
}

fn get_token(device_code: &str, client_id: &str, tenant: &str, scope: Vec<&str>) {
    let mut public_client = PublicClientApplication::builder(client_id)
        .with_device_code(device_code)
        .with_scope(scope)
        .with_tenant(tenant)
        .build();

    let response = public_client.execute().unwrap();
    println!("{:#?}", response);

    let body: Token = response.json().unwrap();

    println!("{:#?}", body);
}
