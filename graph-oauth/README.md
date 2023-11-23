# OAuth 2.0 and OpenID Connect Client For The Microsoft Identity Platform

Support for:

- OpenId, Auth Code Grant, Client Credentials, Device Code
- Automatic Token Refresh
- Interactive Authentication | features = [`interactive-auth`]
- Device Code Polling
- Authorization Using Certificates | features = [`openssl`]

Purpose built as OAuth client for Microsoft Graph and the [graph-rs-sdk](https://crates.io/crates/graph-rs-sdk) project.
This project can however be used outside [graph-rs-sdk](https://crates.io/crates/graph-rs-sdk) as an OAuth client
for Microsoft Identity Platform or by using [graph-rs-sdk](https://crates.io/crates/graph-rs-sdk).

## Table Of Contents

* [Overview](#overview)
* [Credentials](#credentials)
  * [Authorization Code Grant](#authorization-code-grant)
  * [Client Credentials](#client-credentials)
    * [Client Secret Credential](#client-secret-credential)
  * [Environment Credentials](#environment-credentials)
    * [Client Secret Environment Credential](#client-secret-environment-credential)
    * [Resource Owner Password Credential](#resource-owner-password-credential)
* [Automatic Token Refresh](#automatic-token-refresh)
* [Interactive Authentication](#interactive-authentication)

For async:

```toml
graph-oauth = "1.0.2"
tokio = { version = "1.25.0", features = ["full"] }
```

For blocking:

```toml
graph-oauth = "1.0.2"
```

### Feature Flags

- `native-tls`: Use the `native-tls` TLS backend (OpenSSL on *nix, SChannel on Windows, Secure Transport on macOS).
- `rustls-tls`: Use the `rustls-tls` TLS backend (cross-platform backend, only supports TLS 1.2 and 1.3).
- `interactive-auth`: Interactive Authentication using the [wry](https://github.com/tauri-apps/wry) crate to run web view on
  platforms that support it such as on a desktop.
- `openssl`: Use X509 Certificates from the openssl crate in the OAuth2 and OpenId Connect flows. 

Default features: `default=["native-tls"]`

These features enable the native-tls and rustls-tls features in the reqwest crate. 
For more info see the [reqwest](https://crates.io/crates/reqwest) crate.



## Overview

The following examples use the `anyhow` crate for its Result type. It is also recommended that users
of this crate use the `anyhow` crate for better error handling.

The crate is undergoing major development in order to support all or most scenarios in the
Microsoft Identity Platform where its possible to do so. The master branch on GitHub may have some
unstable features. Any version that is not a pre-release version of the crate is considered stable.

Use application builders to store your auth configuration and have the client
handle the access token requests for you.

There are two main types for building your chosen OAuth or OpenId Connect Flow.

- `PublicClientApplication`
- `ConfidentialClientApplication`

Once you have built a `ConfidentialClientApplication` or a `PublicClientApplication`
you can pass these to the graph client.

Automatic token refresh is also done by passing the `ConfidentialClientApplication` or the
`PublicClientApplication` to the `Graph` client.

For more extensive examples see the
[OAuth Examples](https://github.com/sreeise/graph-rs-sdk/tree/master/examples/oauth) in the examples/oauth
directory on [GitHub](https://github.com/sreeise/graph-rs-sdk).


```rust,ignore
let confidental_client: ConfidentialClientApplication<ClientSecretCredential> = ...

let graph_client = Graph::from(confidential_client);
```

## Credentials

### Authorization Code Grant

The authorization code grant is considered a confidential client (except in the hybrid flow)
and we can get an access token by using the authorization code returned in the query of the URL
on redirect after sign in is performed by the user.

Once you have the authorization code you can pass this to the client and the client
will perform the request to get an access token on the first graph api call that you make.

```rust
use graph_rs_sdk::{
    Graph,
    oauth::ConfidentialClientApplication,
};

async fn build_client(
  authorization_code: &str, 
  client_id: &str, 
  client_secret: &str, 
  redirect_uri: &str, 
  scope: Vec<&str>
) -> anyhow::Result<GraphClient> {
    let mut confidential_client = ConfidentialClientApplication::builder(client_id)
        .with_authorization_code(authorization_code) // returns builder type for AuthorizationCodeCredential
        .with_client_secret(client_secret)
        .with_scope(scope)
        .with_redirect_uri(redirect_uri)?
        .build();

  let graph_client = Graph::from(confidential_client);

  Ok(graph_client)
}
```

## Client Credentials

The OAuth 2.0 client credentials grant flow permits a web service (confidential client) to use its own credentials,
instead of impersonating a user, to authenticate when calling another web service. The grant specified in RFC 6749,
sometimes called two-legged OAuth, can be used to access web-hosted resources by using the identity of an application.
This type is commonly used for server-to-server interactions that must run in the background, without immediate
interaction with a user, and is often referred to as daemons or service accounts.

Client credentials flow requires a one time administrator acceptance
of the permissions for your apps scopes. To see an example of building the URL to sign in and accept permissions
as an administrator see [Admin Consent Example](https://github.com/sreeise/graph-rs-sdk/tree/master/examples/oauth/client_credentials/client_credentials_admin_consent.rs)

### Client Secret Credential

```rust
use graph_rs_sdk::{oauth::ConfidentialClientApplication, GraphClient};

pub async fn build_client(client_id: &str, client_secret: &str, tenant: &str) -> GraphClient {
    let mut confidential_client_application = ConfidentialClientApplication::builder(client_id)
        .with_client_secret(client_secret)
        .with_tenant(tenant)
        .build();

    GraphClient::from(&confidential_client_application)
}
```

### Environment Credentials

#### Client Secret Environment Credential

Environment Variables:

- AZURE_TENANT_ID (Optional/Recommended - puts the tenant id in the authorization url)
- AZURE_CLIENT_ID (Required)
- AZURE_CLIENT_SECRET (Required)

```rust
pub fn client_secret_credential() -> anyhow::Result<GraphClient> {
    let confidential_client = EnvironmentCredential::client_secret_credential()?;
    Ok(GraphClient::from(&confidential_client))
}
```

#### Resource Owner Password Credential

Environment Variables:

- AZURE_TENANT_ID (Optional - puts the tenant id in the authorization url)
- AZURE_CLIENT_ID (Required)
- AZURE_USERNAME (Required)
- AZURE_PASSWORD (Required)

```rust
pub fn username_password() -> anyhow::Result<GraphClient> {
    let public_client = EnvironmentCredential::resource_owner_password_credential()?;
    Ok(GraphClient::from(&public_client))
}
```

## Automatic Token Refresh

Using automatic token refresh requires getting a refresh token as part of the token response.
To get a refresh token you must include the `offline_access` scope.

Automatic token refresh is done by passing the `ConfidentialClientApplication` or the
`PublicClientApplication` to the `Graph` client.

If you are using the `client credentials` grant you do not need the `offline_access` scope.
Tokens will still be automatically refreshed as this flow does not require using a refresh token to get
a new access token.

```rust
async fn authenticate(client_id: &str, tenant: &str, redirect_uri: &str) {
  let scope = vec!["offline_access"];
  
  let mut credential_builder = ConfidentialClientApplication::builder(client_id)
          .auth_code_url_builder()
          .with_tenant(tenant)
          .with_scope(scope) // Adds offline_access as a scope which is needed to get a refresh token.
          .with_redirect_uri(redirect_uri)
          .url();
  // ... add any other parameters you need
}
```

## Interactive Authentication

Requires Feature `interactive_auth`

```toml
[dependencies]
graph-rs-sdk = { version = "...", features = ["interactive_auth"] }
```

Interactive Authentication uses the [wry](https://github.com/tauri-apps/wry) crate to run web view on
platforms that support it such as on a desktop.

```rust
use graph_rs_sdk::{oauth::AuthorizationCodeCredential, GraphClient};

async fn authenticate(
  tenant_id: &str,
  client_id: &str,
  client_secret: &str,
  redirect_uri: &str,
  scope: Vec<&str>,
) -> anyhow::Result<GraphClient> {
  std::env::set_var("RUST_LOG", "debug");
  pretty_env_logger::init();

  let (authorization_query_response, mut credential_builder) =
          AuthorizationCodeCredential::authorization_url_builder(client_id)
                  .with_tenant(tenant_id)
                  .with_scope(scope) // Adds offline_access as a scope which is needed to get a refresh token.
                  .with_redirect_uri(redirect_uri)
                  .with_interactive_authentication_for_secret(Default::default())
                  .unwrap();

  debug!("{authorization_query_response:#?}");

  let mut confidential_client = credential_builder.with_client_secret(client_secret).build();

  Ok(GraphClient::from(&confidential_client))
}

```
