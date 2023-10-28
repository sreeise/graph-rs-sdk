# OAuth Overview

There are two main types for building your chosen OAuth or OpenId Connect Flow.

- `PublicClientApplication`
- `ConfidentialClientApplication`


### Authorization Code Grant

The authorization code grant is considered a confidential client (except in the hybrid flow)
and we can get an access token by using the authorization code returned in the query of the URL 
on redirect after sign in is performed by the user.

```rust
use graph_rs_sdk::{
    Graph,
    oauth::ConfidentialClientApplication,
};

#[tokio::main]
async fn main() {
    let authorization_code = "<AUTH_CODE>";
    let client_id = "<CLIENT_ID>";
    let client_secret = "<CLIENT_SECRET>";
    let scope = vec!["<SCOPE>", "<SCOPE>"];
    let redirect_uri = "http://localhost:8080";

    let mut confidential_client = ConfidentialClientApplication::builder(client_id)
        .with_authorization_code(authorization_code) // returns builder type for AuthorizationCodeCredential
        .with_client_secret(client_secret)
        .with_scope(scope)
        .with_redirect_uri(redirect_uri)
        .unwrap()
        .build();

    let graph_client = Graph::from(confidential_client);

    let _response = graph_client
        .users()
        .list_user()
        .send()
        .await;
}
```
