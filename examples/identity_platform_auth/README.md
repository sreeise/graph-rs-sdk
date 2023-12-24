# Identity Overview

There are two main types for building your chosen OAuth or OpenId Connect Flow.

- `PublicClientApplication`
- `ConfidentialClientApplication`


## Table Of Contents

* [Credentials](#credentials)
  * [Authorization Code Grant](#authorization-code-grant)
  * [Client Credentials](#client-credentials)
    * [Client Secret Credential](#client-secret-credential)
  * [Environment Credentials](#environment-credentials)
    * [Client Secret Environment Credential](#client-secret-environment-credential)
    * [Resource Owner Password Credential](#resource-owner-password-credential)

## Credentials

### Authorization Code Grant

The authorization code grant is considered a confidential client (except in the hybrid flow)
and we can get an access token by using the authorization code returned in the query of the URL 
on redirect after sign in is performed by the user.

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
