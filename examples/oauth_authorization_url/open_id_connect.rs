use graph_oauth::oauth::OpenIdCredential;
use url::Url;

// Use your client id and client secret found in the Azure Portal
fn open_id_authorization_url(client_id: &str, client_secret: &str) -> anyhow::Result<Url> {
    Ok(OpenIdCredential::authorization_url_builder()?
        .with_client_id(client_id)
        .with_default_scope()?
        .extend_scope(vec!["Files.Read"])
        .build()
        .url()?)
}
