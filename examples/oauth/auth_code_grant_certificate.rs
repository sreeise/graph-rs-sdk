use graph_rs_sdk::oauth::{
    AccessToken, AuthCodeAuthorizationUrl, AuthorizationCodeCertificateCredential, ClientAssertion,
    ConfidentialClientApplication, PKey, Private, TokenRequest, X509,
};
use std::fs::File;
use std::io::Read;
use warp::Filter;

// This flow uses an X509 certificate for authorization. The public key should
// be uploaded to Azure Active Directory. In order to use the certificate
// flow the ClientAssertion struct can be used to generate the needed
// client assertion given an X509 certificate public key and private key.

// If you want the client to generate a client assertion for you it
// requires the openssl feature be enabled. There are two openssl
// exports provided in this library: X509 and Pkey (private key) that will
// be used to generate the client assertion. You only need to provide these
// to the library in order to generate the client assertion.

// You can use any way you want to get the public and private key. This example below uses
// File to get the contents of the X509 and private key, but if these files are local
// then consider using Rust's include_bytes macro which takes a local path to a file and returns the
// contents of that file as bytes. This is the expected format by X509 and Pkey in openssl.

static CLIENT_ID: &str = "<CLIENT_ID>";

// Only required for certain applications. Used here as an example.
static TENANT: &str = "<TENANT_ID>";

// The path to the public key file.
static CERTIFICATE_PATH: &str = "<CERTIFICATE_PATH>";

// The path to the private key file of the certificate.
static PRIVATE_KEY_PATH: &str = "<PRIVATE_KEY_PATH>";

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct AccessCode {
    code: String,
}

pub fn authorization_sign_in(client_id: &str, tenant_id: &str) {
    let auth_url_builder = AuthCodeAuthorizationUrl::builder()
        .with_client_id(client_id)
        .with_tenant(tenant_id)
        .with_redirect_uri("http://localhost:8080")
        .with_scope(vec!["User.Read"])
        .build();

    let url = auth_url_builder.url().unwrap();
    // web browser crate in dev dependencies will open to default browser in the system.
    webbrowser::open(url.as_str()).unwrap();
}

pub fn get_confidential_client(
    authorization_code: &str,
    client_id: &str,
    tenant_id: &str,
) -> anyhow::Result<ConfidentialClientApplication> {
    let mut cert_file = File::open(PRIVATE_KEY_PATH).unwrap();
    let mut certificate: Vec<u8> = Vec::new();
    cert_file.read_to_end(&mut cert);

    let mut private_key_file = File::open(CERTIFICATE_PATH).unwrap();
    let mut private_key: Vec<u8> = Vec::new();
    private_key_file.read_to_end(&mut cert);

    let cert = X509::from_pem(certificate.as_slice()).unwrap();
    let pkey = PKey::private_key_from_pem(private_key.as_slice()).unwrap();

    let signed_client_assertion =
        ClientAssertion::new_with_tenant(client_id, tenant_id, cert, pkey);

    let credentials = AuthorizationCodeCertificateCredential::builder()
        .with_authorization_code(authorization_code)
        .with_client_id(client_id)
        .with_tenant(tenant_id)
        .with_certificate(&signed_client_assertion)?
        .with_scope(vec!["User.Read"])
        .with_redirect_uri("http://localhost:8080")
        .build();

    Ok(ConfidentialClientApplication::from(credentials))
}

// When the authorization code comes in on the redirect from sign in, call the get_credential
// method passing in the authorization code. The AuthorizationCodeCertificateCredential can be passed
// to a confidential client application in order to exchange the authorization code
// for an access token.
async fn handle_redirect(
    code_option: Option<AccessCode>,
) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    match code_option {
        Some(access_code) => {
            // Print out the code for debugging purposes.
            println!("{:#?}", access_code.code);

            let mut confidential_client =
                get_confidential_client(access_code.code.as_str(), CLIENT_ID, TENANT).unwrap();

            // Returns reqwest::Response
            let response = confidential_client.get_token_async().await.unwrap();
            println!("{response:#?}");

            if response.status().is_success() {
                let access_token: AccessToken = response.json().await.unwrap();

                // If all went well here we can print out the Access Token.
                println!("AccessToken: {:#?}", access_token.bearer_token());
            } else {
                // See if Microsoft Graph returned an error in the Response body
                let result: reqwest::Result<serde_json::Value> = response.json().await;
                println!("{result:#?}");
                return Ok(Box::new("Error Logging In! You can close your browser."));
            }

            // Generic login page response.
            Ok(Box::new(
                "Successfully Logged In! You can close your browser.",
            ))
        }
        None => Err(warp::reject()),
    }
}

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
    let query = warp::query::<AccessCode>()
        .map(Some)
        .or_else(|_| async { Ok::<(Option<AccessCode>,), std::convert::Infallible>((None,)) });

    let routes = warp::get().and(query).and_then(handle_redirect);

    authorization_sign_in(CLIENT_ID, TENANT);

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}
