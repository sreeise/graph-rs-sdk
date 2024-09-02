// This example shows using client credentials being passed to the Graph client which will
// handle access token refresh automatically. This example requires that admin consent
// has been granted to your app beforehand. If you have not granted admin consent, see
// examples/client_credentials_admin_consent.rs for more info.

use graph_rs_sdk::{identity::ConfidentialClientApplication, GraphClient};

pub async fn build_client(client_id: &str, client_secret: &str, tenant: &str) -> GraphClient {
    let mut confidential_client_application = ConfidentialClientApplication::builder(client_id)
        .with_client_secret(client_secret)
        .with_tenant(tenant)
        .build();

    GraphClient::from(&confidential_client_application)
}
