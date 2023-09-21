#![allow(dead_code)]

#[macro_use]
extern crate serde;

use graph_rs_sdk::oauth::{
    AuthorizationCodeCertificateCredential, ConfidentialClientApplication, MsalTokenResponse, PKey,
    TokenCredentialExecutor, X509Certificate, X509,
};
use std::fs::File;
use std::io::Read;
use warp::Filter;

#[tokio::main]
async fn main() {
    start_server_main().await;
}

// Requires feature openssl be enabled for graph-rs-sdk or graph-oauth

// X509 certificates can be used for the auth code grant with
// a certificate (AuthorizationCodeCertificateCredential) and
// the client credentials grant with a certificate (ClientCertificateCredential).

// The example below shows using the authorization code grant with a certificate.

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

static REDIRECT_URI: &str = "http://localhost:8000/redirect";

static SCOPE: &str = "User.Read";

// The path to the public key file.
static CERTIFICATE_PATH: &str = "<CERTIFICATE_PATH>";

// The path to the private key file of the certificate.
static PRIVATE_KEY_PATH: &str = "<PRIVATE_KEY_PATH>";

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct AccessCode {
    code: String,
}

pub fn authorization_sign_in() {
    let url = AuthorizationCodeCertificateCredential::authorization_url_builder(CLIENT_ID)
        .with_tenant(TENANT)
        .with_redirect_uri(REDIRECT_URI)
        .with_scope(vec![SCOPE])
        .url()
        .unwrap();

    // web browser crate in dev dependencies will open to default browser in the system.
    webbrowser::open(url.as_str()).unwrap();
}

pub fn x509_certificate(client_id: &str, tenant_id: &str) -> anyhow::Result<X509Certificate> {
    // Use include_bytes!(file_path) if the files are local
    let mut cert_file = File::open(PRIVATE_KEY_PATH)?;
    let mut certificate: Vec<u8> = Vec::new();
    cert_file.read_to_end(&mut certificate)?;

    let mut private_key_file = File::open(CERTIFICATE_PATH)?;
    let mut private_key: Vec<u8> = Vec::new();
    private_key_file.read_to_end(&mut private_key)?;

    let cert = X509::from_pem(certificate.as_slice())?;
    let pkey = PKey::private_key_from_pem(private_key.as_slice())?;
    Ok(X509Certificate::new_with_tenant(
        client_id, tenant_id, cert, pkey,
    ))
}

// When the authorization code comes in on the redirect from sign in, call the get_credential
// method passing in the authorization code.
// Building AuthorizationCodeCertificateCredential will create a ConfidentialClientApplication
// which can be used to exchange the authorization code for an access token.
async fn handle_redirect(
    code_option: Option<AccessCode>,
) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    match code_option {
        Some(access_code) => {
            // Print out the code for debugging purposes.
            println!("{:#?}", access_code.code);

            let authorization_code = access_code.code;
            let x509 = x509_certificate(CLIENT_ID, TENANT).unwrap();

            let mut confidential_client = ConfidentialClientApplication::builder(CLIENT_ID)
                .with_authorization_code_x509_certificate(authorization_code, &x509)
                .unwrap()
                .with_tenant(TENANT)
                .with_scope(vec![SCOPE])
                .with_redirect_uri(REDIRECT_URI)
                .unwrap()
                .build();

            // Returns reqwest::Response
            let response = confidential_client.execute_async().await.unwrap();
            println!("{response:#?}");

            if response.status().is_success() {
                let mut msal_token: MsalTokenResponse = response.json().await.unwrap();
                msal_token.enable_pii_logging(true);

                // If all went well here we can print out the Access Token.
                println!("AccessToken: {:#?}", msal_token);
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

    let routes = warp::get()
        .and(warp::path("redirect"))
        .and(query)
        .and_then(handle_redirect);

    authorization_sign_in();

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}
