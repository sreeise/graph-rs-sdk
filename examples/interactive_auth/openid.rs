use graph_rs_sdk::{
    identity::{OpenIdCredential, ResponseMode, ResponseType},
    GraphClient,
};
use url::Url;

async fn openid_authenticate(
    tenant_id: &str,
    client_id: &str,
    client_secret: &str,
    redirect_uri: &str,
) -> anyhow::Result<GraphClient> {
    std::env::set_var("RUST_LOG", "debug");
    pretty_env_logger::init();

    let (authorization_query_response, mut credential_builder) =
        OpenIdCredential::authorization_url_builder(client_id)
            .with_tenant(tenant_id)
            .with_scope(vec!["user.read", "offline_access", "profile", "email"]) // Adds offline_access as a scope which is needed to get a refresh token.
            .with_response_mode(ResponseMode::Fragment)
            .with_response_type(vec![ResponseType::Code, ResponseType::IdToken])
            .with_redirect_uri(Url::parse(redirect_uri)?)
            .with_interactive_auth(client_secret, Default::default())?
            .into_result()?;

    debug!("{authorization_query_response:#?}");

    let confidential_client = credential_builder.build();

    Ok(GraphClient::from(&confidential_client))
}
