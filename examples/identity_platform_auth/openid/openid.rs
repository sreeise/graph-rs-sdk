use graph_rs_sdk::identity::{ConfidentialClientApplication, IdToken};
use graph_rs_sdk::GraphClient;
use url::Url;

// OpenIdCredential will automatically include the openid scope
fn get_graph_client(
    tenant_id: &str,
    client_id: &str,
    client_secret: &str,
    redirect_uri: Url,
    scope: Vec<&str>,
    id_token: IdToken,
) -> GraphClient {
    let mut confidential_client = ConfidentialClientApplication::builder(client_id)
        .with_openid(id_token.code.unwrap(), client_secret)
        .with_tenant(tenant_id)
        .with_redirect_uri(redirect_uri)
        .with_scope(scope)
        .build();

    GraphClient::from(&confidential_client)
}
