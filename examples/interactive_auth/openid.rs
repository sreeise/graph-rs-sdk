use anyhow::anyhow;
use graph_rs_sdk::{
    http::Url,
    identity::interactive::WebViewAuthorizationEvent,
    identity::OpenIdCredentialBuilder,
    identity::{IntoCredentialBuilder, OpenIdCredential, ResponseMode, ResponseType},
    GraphClient,
};

// Use the into_credential_builder method to map the WebViewAuthorizationEvent to a
// CredentialBuilder result. The CredentialBuilder for openid will be the OpenIdCredentialBuilder.
// The into_credential_builder method transforms WebViewAuthorizationEvent::Authorized to a
// successful result.
//
// A WebViewAuthorizationEvent::Unauthorized and WebViewAuthorizationEvent::WindowClosed
// are returned as errors in the result: Result<(AuthorizationResponse, CredentialBuilder), WebViewError>
//
// The openid_authenticate2 method shows handling the WebViewAuthorizationEvent manually which is the
// default return type of using with_interactive_auth and provides better event handling.

async fn openid_authenticate(
    tenant_id: &str,
    client_id: &str,
    client_secret: &str,
    redirect_uri: &str,
) -> anyhow::Result<GraphClient> {
    std::env::set_var("RUST_LOG", "debug");
    pretty_env_logger::init();

    let (authorization_response, mut credential_builder) =
        OpenIdCredential::authorization_url_builder(client_id)
            .with_tenant(tenant_id)
            .with_scope(vec!["user.read", "offline_access", "profile", "email"]) // Adds offline_access as a scope which is needed to get a refresh token.
            .with_response_mode(ResponseMode::Fragment)
            .with_response_type(vec![ResponseType::Code, ResponseType::IdToken])
            .with_redirect_uri(Url::parse(redirect_uri)?)
            .with_interactive_auth(client_secret, Default::default())
            .into_credential_builder()?;

    debug!("{authorization_response:#?}");

    let confidential_client = credential_builder.build();

    Ok(GraphClient::from(&confidential_client))
}

async fn openid_authenticate2(
    tenant_id: &str,
    client_id: &str,
    client_secret: &str,
    redirect_uri: &str,
) -> anyhow::Result<GraphClient> {
    std::env::set_var("RUST_LOG", "debug");
    pretty_env_logger::init();

    let auth_event = OpenIdCredential::authorization_url_builder(client_id)
        .with_tenant(tenant_id)
        .with_scope(vec!["user.read", "offline_access", "profile", "email"]) // Adds offline_access as a scope which is needed to get a refresh token.
        .with_response_mode(ResponseMode::Fragment)
        .with_response_type(vec![ResponseType::Code, ResponseType::IdToken])
        .with_redirect_uri(Url::parse(redirect_uri)?)
        .with_interactive_auth(client_secret, Default::default())?;

    match auth_event {
        WebViewAuthorizationEvent::Authorized {
            authorization_response,
            credential_builder,
        } => {
            debug!("{authorization_response:#?}");
            let confidential_client = credential_builder.build();
            Ok(GraphClient::from(&confidential_client))
        }
        WebViewAuthorizationEvent::Unauthorized(authorization_response) => {
            debug!("{authorization_response:#?}");
            Err(anyhow!("error signing in"))
        }
        WebViewAuthorizationEvent::WindowClosed(reason) => {
            // The webview window closed before sign in was complete. The can happen due to various issues
            // but most likely it was either that the user closed the window before finishing sign in or
            // there was an error returned from Microsoft Graph.
            println!("{:#?}", reason);
            Err(anyhow!(reason))
        }
    }
}
