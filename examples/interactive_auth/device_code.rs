use graph_oauth::interactive::WebViewOptions;
use graph_oauth::PublicClientApplication;
use graph_rs_sdk::GraphClient;

// NOTE: Device code interactive auth does not work in async code.

// Device code interactive auth returns a polling executor in order to get the
// public client credential which you can pass to the GraphClient.

// The DeviceAuthorizationResponse returns the initial JSON response body
// that contains the device code that the user enters when logging in.

/// Example run:
/// ```rust,ignore
/// fn main() {
///     std::env::set_var("RUST_LOG", "debug");
///     pretty_env_logger::init();
///     let graph_client = auth("client-id", "tenant-id", vec!["User.Read"]).unwrap();
///     log::debug!("{:#?}", &graph_client);
/// }
/// ```
fn device_code_authenticate(
    client_id: &str,
    tenant: &str,
    scope: Vec<&str>,
) -> anyhow::Result<GraphClient> {
    let (device_authorization_response, mut interactive_auth_executor) =
        PublicClientApplication::builder(client_id)
            .with_tenant(tenant)
            .with_scope(scope)
            .with_device_code_executor()
            .with_interactive_auth(WebViewOptions::default())?;

    log::debug!("{:#?}", device_authorization_response);
    log::debug!(
        "To sign in, enter the code {:#?} to authenticate.",
        device_authorization_response.user_code
    );

    // After providing the code to the user to sign in run the executor `poll` method which
    // will poll for a response to the authentication and if successful return a
    // PublicClientApplication<DeviceCodeCredential>
    let public_client = interactive_auth_executor.poll()?;

    Ok(GraphClient::from(&public_client))
}
