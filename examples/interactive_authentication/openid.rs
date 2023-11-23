use graph_rs_sdk::{
    oauth::{OpenIdCredential, ResponseMode, ResponseType},
    GraphClient,
};

async fn openid_authenticate(
    tenant_id: &str,
    client_id: &str,
    client_secret: &str,
    redirect_uri: &str,
) -> anyhow::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    pretty_env_logger::init();

    let (authorization_query_response, mut credential_builder) =
        OpenIdCredential::authorization_url_builder(client_id)
            .with_tenant(tenant_id)
            .with_scope(vec!["user.read", "offline_access"]) // Adds offline_access as a scope which is needed to get a refresh token.
            .with_response_mode(ResponseMode::Fragment)
            .with_response_type(vec![ResponseType::Code, ResponseType::IdToken])
            .with_redirect_uri(redirect_uri)?
            .with_interactive_authentication(Default::default())?;

    debug!("{authorization_query_response:#?}");

    let mut confidential_client = credential_builder.with_client_secret(client_secret).build();

    let client = GraphClient::from(&confidential_client);

    let response = client.users().list_user().send().await?;

    debug!("{response:#?}");
    let body: serde_json::Value = response.json().await?;
    debug!("{body:#?}");

    Ok(())
}
