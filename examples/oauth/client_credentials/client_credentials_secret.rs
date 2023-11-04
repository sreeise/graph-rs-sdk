// This example shows using client credentials being passed to the Graph client which will
// handle access token refresh automatically. This example requires that admin consent
// has been granted to your app beforehand. If you have not granted admin consent, see
// examples/client_credentials_admin_consent.rs for more info.

use graph_rs_sdk::{oauth::ConfidentialClientApplication, Graph};

// Replace client id, client secret, and tenant id with your own values.
static CLIENT_ID: &str = "<CLIENT_ID>";
static CLIENT_SECRET: &str = "<CLIENT_SECRET>";
static TENANT_ID: &str = "<TENANT_ID>";

pub async fn get_graph_client() -> Graph {
    let mut confidential_client_application = ConfidentialClientApplication::builder(CLIENT_ID)
        .with_client_secret(CLIENT_SECRET)
        .with_tenant(TENANT_ID)
        .build();

    Graph::from(&confidential_client_application)
}
