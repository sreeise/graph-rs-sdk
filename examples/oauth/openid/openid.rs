use graph_rs_sdk::oauth::{
    ConfidentialClientApplication, IdToken, OpenIdAuthorizationUrlParameters, OpenIdCredential,
    Prompt, ResponseMode, ResponseType, Token, TokenCredentialExecutor, TokenRequest,
};
use graph_rs_sdk::{error::IdentityResult, Graph};
use url::Url;

// The client id and client secret must be changed before running this example.
static CLIENT_ID: &str = "";
static CLIENT_SECRET: &str = "";
static TENANT_ID: &str = "";

static REDIRECT_URI: &str = "http://localhost:8000/redirect";

// Use the form post response mode when listening on a server instead
// of the URL query because the the query does not get sent to servers.
fn openid_authorization_url() -> IdentityResult<Url> {
    Ok(OpenIdCredential::authorization_url_builder(CLIENT_ID)
        .with_tenant(TENANT_ID)
        //.with_default_scope()?
        .with_redirect_uri(REDIRECT_URI)?
        .with_response_mode(ResponseMode::FormPost)
        .with_response_type([ResponseType::IdToken, ResponseType::Code])
        .with_prompt(Prompt::SelectAccount)
        .with_state("1234")
        .with_scope(vec!["User.Read", "User.ReadWrite"])
        .build()
        .url()?)
}

// OpenIdCredential will automatically include the openid scope and therefore
// does not need to be added using with_scope
fn get_graph_client(mut id_token: IdToken) -> Graph {
    let mut confidential_client = ConfidentialClientApplication::builder(CLIENT_ID)
        .with_openid(id_token.code.unwrap(), CLIENT_SECRET)
        .with_tenant(TENANT_ID)
        .with_redirect_uri(REDIRECT_URI)
        .unwrap()
        .with_scope(vec!["User.Read", "User.ReadWrite"])
        .build();

    Graph::from(&confidential_client)
}
