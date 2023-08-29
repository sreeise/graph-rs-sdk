# OAuth Overview

### Authorization Code Grant 

Getting the Confidential Client

```rust
use graph_rs_sdk::oauth::{
    AuthorizationCodeCredential, ConfidentialClientApplication,
};

fn main() {
    let authorization_code = "<AUTH_CODE>";
    let client_id = "<CLIENT_ID>";
    let client_secret = "<CLIENT_SECRET>";

    let auth_code_credential = AuthorizationCodeCredential::builder(authorization_code)
        .with_client_id(client_id)
        .with_client_secret(client_secret)
        .with_scope(vec!["files.read", "offline_access"])
        .with_redirect_uri("http://localhost:8000/redirect")?
        .build();

    let confidential_client = ConfidentialClientApplication::from(auth_code_credential);
}
```
