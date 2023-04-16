use graph_rs_sdk::oauth::ClientCredentialsAuthorizationUrl;
/// # Example
/// ```
/// use graph_rs_sdk::*:
///
/// #[tokio::main]
/// async fn main() {
///   start_server_main().await;
/// }
/// ```
///
/// # Overview:
///
/// [Microsoft Client Credentials](https://docs.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-client-creds-grant-flow)
/// You can use the OAuth 2.0 client credentials grant specified in RFC 6749,
/// sometimes called two-legged OAuth, to access web-hosted resources by using the
/// identity of an application. This type of grant is commonly used for server-to-server
/// interactions that must run in the background, without immediate interaction with a user.
/// These types of applications are often referred to as daemons or service accounts.
///
/// This OAuth flow example requires signing in as an administrator for Azure, known as admin consent,
/// to approve your application to call Microsoft Graph Apis on behalf of a user. Admin consent
/// only has to be done once for a user. After admin consent is given, the oauth client can be
/// used to continue getting new access tokens programmatically.
use warp::Filter;

// This example shows getting the URL for the one time admin consent required
// for the client credentials flow.
// See https://learn.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-client-creds-grant-flow#request-the-permissions-from-a-directory-admin

// Once an admin has given consent the ClientSecretCredential can be
// used to get access tokens programmatically without any consent by a user
// or admin.

// Paste the URL into a browser and have the admin sign in and approve the admin consent.
fn example_authorization_credential() {
    let authorization_credential =
        ClientCredentialsAuthorizationUrl::new("<CLIENT_ID>", "<REDIRECT_URI>");
    let url = authorization_credential.url();
}

// Use the builder if you want to set a specific tenant, or a state, or set a specific Authority.
fn example_builder() {
    let mut builder = ClientCredentialsAuthorizationUrl::builder();
    let authorization_credential = builder
        .with_client_id("<CLIENT_ID>")
        .with_redirect_uri("<REDIRECT_URI>")
        .with_state("<STATE>")
        .with_tenant("<TENANT_ID>")
        .build();
    let url = authorization_credential.url().unwrap();
}

// -------------------------------------------------------------------------------------------------
// Full example with handling redirect:

// After admin consent has been granted see examples/client_credential.rs for how to
// programmatically get access tokens using the client credentials flow.

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ClientCredentialsResponse {
    admin_consent: bool,
    tenant: String,
}

async fn handle_redirect(
    client_credential_option: Option<ClientCredentialsResponse>,
) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    match client_credential_option {
        Some(client_credential_response) => {
            // Print out for debugging purposes.
            println!("{client_credential_response:#?}");

            // Generic login page response.
            Ok(Box::new(
                "Successfully Logged In! You can close your browser.",
            ))
        }
        None => Err(warp::reject()),
    }
}

// The client_id must be changed before running this example.
static CLIENT_ID: &str = "<CLIENT_ID>";
static REDIRECT_URI: &str = "http://localhost:8000/redirect";

/// # Example
/// ```
/// use graph_rs_sdk::*:
///
/// #[tokio::main]
/// async fn main() {
///   start_server_main().await;
/// }
/// ```
pub async fn start_server_main() {
    let query = warp::query::<ClientCredentialsResponse>()
        .map(Some)
        .or_else(|_| async {
            Ok::<(Option<ClientCredentialsResponse>,), std::convert::Infallible>((None,))
        });

    let routes = warp::get()
        .and(warp::path("redirect"))
        .and(query)
        .and_then(handle_redirect);

    // Get the oauth client and request a browser sign in
    let authorization_credential = ClientCredentialsAuthorizationUrl::new(CLIENT_ID, REDIRECT_URI);
    let url = authorization_credential.url().unwrap();

    // webbrowser crate in dev dependencies will open to default browser in the system.
    webbrowser::open(url.as_str()).unwrap();

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}
