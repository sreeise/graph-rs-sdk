use graph_rs_sdk::{
    error::IdentityResult,
    identity::{ClientCredentialsAuthorizationUrlParameters, ClientSecretCredential},
};
use url::Url;

// The client_id must be changed before running this example.
static CLIENT_ID: &str = "<CLIENT_ID>";
static REDIRECT_URI: &str = "http://localhost:8000/redirect";

// Paste the URL into a browser and log in to approve the admin consent.
fn get_admin_consent_url() -> IdentityResult<url::Url> {
    let auth_url_parameters =
        ClientCredentialsAuthorizationUrlParameters::new(CLIENT_ID, REDIRECT_URI)?;
    auth_url_parameters.url()
}

// Use the builder if you want to set a specific tenant, or a state, or set a specific Authority.
fn get_admin_consent_url_from_builder() -> IdentityResult<url::Url> {
    let url_builder = ClientSecretCredential::authorization_url_builder(CLIENT_ID)
        .with_redirect_uri(Url::parse(REDIRECT_URI).unwrap())
        .with_state("123")
        .with_tenant("tenant_id")
        .build();
    url_builder.url()
}
