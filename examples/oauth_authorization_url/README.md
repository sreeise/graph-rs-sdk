# Building an Authorization URL

The authorization request is the initial request to sign in where the user
is taken to the sign in page and enters their credentials.

If successful the user will be redirected back to your app and the authorization
code will be in the query of the URL.

## Examples


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
