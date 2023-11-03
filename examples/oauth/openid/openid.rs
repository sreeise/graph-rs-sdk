use graph_rs_sdk::oauth::{ConfidentialClientApplication, IdToken};
use graph_rs_sdk::Graph;

// OpenIdCredential will automatically include the openid scope
fn get_graph_client(
    tenant_id: &str,
    client_id: &str,
    client_secret: &str,
    redirect_uri: &str,
    scope: Vec<&str>,
    id_token: IdToken,
) -> Graph {
    let mut confidential_client = ConfidentialClientApplication::builder(client_id)
        .with_openid(id_token.code.unwrap(), client_secret)
        .with_tenant(tenant_id)
        .with_redirect_uri(redirect_uri)
        .unwrap()
        .with_scope(scope)
        .build();

    Graph::from(&confidential_client)
}
