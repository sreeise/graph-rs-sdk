# OAuth Overview

### Authorization Code Grant 

Getting the Confidential Client

```rust
use graph_rs_sdk::oauth::{
    ConfidentialClientApplication,
};

fn main() {
    let authorization_code = "<AUTH_CODE>";
    let client_id = "<CLIENT_ID>";
    let client_secret = "<CLIENT_SECRET>";
    let scope = vec!["<SCOPE>", "<SCOPE>"];

    let mut confidential_client = ConfidentialClientApplication::builder(client_id)
        .with_authorization_code(authorization_code)
        .with_client_secret(client_secret)
        .with_scope(SCOPE.clone())
        .with_redirect_uri(REDIRECT_URI)
        .unwrap()
        .build();
}
```
