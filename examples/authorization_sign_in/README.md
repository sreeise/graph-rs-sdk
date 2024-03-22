# Building an Authorization URL

The authorization request is the initial request to sign in where the user
is taken to the sign-in page and enters their credentials.

If successful, the user will be redirected back to your app and the authorization
code will be in the query of the URL.

## Examples

### Authorization Code Grant

* **Tenant**
  * Required. Defaults to `common` when not provided to the client and is automatically set by the client.
  * Definition: You can use the {tenant} value in the path of the request to control who can sign in to the application.
    The allowed values are common, organizations, consumers, and tenant identifiers.
* **Client Id** -
  * Required.
  * Definition: The Application (client) ID that the Azure portal – App registrations experience assigned to your app.
* **Response Type**
  * Required. Defaults to `code` in the client and is automatically set by the client.
  * Definition: Must include `code` for the authorization code flow. Can also include `id_token` or `token` if using the hybrid flow.
* **Redirect URI**
  * Required. Defaults to `http://localhost` in the client and is automatically set by the client.
  * Definition: The redirect_uri of your app, where authentication responses can be sent and received by your app. 
  It must exactly match one of the redirect URIs you registered in the portal, except it must be URL-encoded.
* **Scope**
  * Required: Not automatically set by the client. Callers will want to make sure at least one scope is provided to the client.
  * Definition: A space-separated list of scopes that you want the user to consent to.


```rust
use graph_rs_sdk::oauth::AuthorizationCodeCredential;

fn auth_code_flow_authorization_url(client_id: &str, redirect_uri: &str, scope: Vec<String>) {
  let url = AuthorizationCodeCredential::authorization_url_builder(client_id)
      .with_redirect_uri(redirect_uri)
      .with_scope(scope)
      .url()
      .unwrap();

  // web browser crate opens default browser.
  webbrowser::open(url.as_str()).unwrap();
}

```

### Authorization Code Grant With Proof Key For Code Exchange (PKCE)

```rust
use graph_rs_sdk::oauth::{AuthorizationCodeCredential, ProofKeyForCodeExchange};

fn auth_code_pkce_authorization_url(client_id: &str, redirect_uri: &str, scope: Vec<String>) {
  let pkce = ProofKeyForCodeExchange::generate().unwrap();
  
  let url = AuthorizationCodeCredential::authorization_url_builder(client_id)
      .with_redirect_uri(redirect_uri)
      .with_scope(scope)
      .with_pkce(&pkce)
      .url()
      .unwrap();
}
```

### [Authorization Code Hybrid Flow](https://learn.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-auth-code-flow#request-an-id-token-as-well-or-hybrid-flow)

Easy to use methods are provided for parameters that can be changed for the auth code hybrid flow such as ResponseType.

If your wanting to use openid connect consider using the `OpenIdCredential` which is a dedicated type that
is preconfigured to make it easy to perform the openid connect flow.

```rust
fn auth_code_flow_authorization_url(client_id: &str, redirect_uri: &str, scope: Vec<String>) {
  let url = AuthorizationCodeCredential::authorization_url_builder(client_id)
      .with_redirect_uri(redirect_uri)
      .with_scope(scope)
      .with_response_type([ResponseType::IdToken, ResponseType::Code])
      .url()
      .unwrap();
}

```


### OpenId Connect

#### Required Parameters (see [Send the sign-in request](https://learn.microsoft.com/en-us/azure/active-directory/develop/v2-protocols-oidc#send-the-sign-in-request) in the Microsoft Identity Platform Documentation):

The tenant, response type, scope, and nonce parameters all have default values that are automatically set by
the client. The scope parameter can and should include more than just the default value (openid).

* **Tenant**
  * Required. Defaults to `common` when not provided to the client and is automatically set by the client.
  * Definition: You can use the {tenant} value in the path of the request to control who can sign in to the application. 
    The allowed values are common, organizations, consumers, and tenant identifiers.
* **Client Id** - 
  * Required.
  * Definition: The Application (client) ID that the Azure portal – App registrations experience assigned to your app.
* **Response Type**
  * Required. Defaults to `id_token` in the client and is automatically set by the client.
  * Definition: Must include `id_token` for OpenID Connect sign-in.
* **Scope** 
  * Required. The scope `openid` is automatically set by the client.
  * Definition: A space-separated list of scopes. For OpenID Connect, it must include the scope openid, which translates to the 
    Sign you in permission in the consent UI. You might also include other scopes in this request for requesting consent.
* **Nonce**
  * Required. The client generates a nonce using the same secure cryptographic algorithm that is used for PKCE flows. You can also provide your own.
  * Definition: A value generated and sent by your app in its request for an ID token. 
  The same nonce value is included in the ID token returned to your app by the Microsoft identity platform. 
  To mitigate token replay attacks, your app should verify the nonce value in the ID token is the same value it sent 
  when requesting the token. The value is typically a unique, random string.

```rust
use graph_oauth::identity::OpenIdCredential;

fn open_id_authorization_url(client_id: &str, tenant: &str, redirect_uri: &str, scope: Vec<&str>) -> anyhow::Result<Url> {
    Ok(OpenIdCredential::authorization_url_builder()?
        .with_client_id(client_id)
        .with_tenant(tenant)
        .with_redirect_uri(redirect_uri)?
        .extend_scope(scope)
        .build()
        .url()?)
}
```
