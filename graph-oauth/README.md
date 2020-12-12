# graph-oauth

OAuth2 for the graph-rs project.

See the project on [GitHub](https://github.com/sreeise/graph-rs).

### Authorization Flows and Microsoft Graph

    1. Token Flow - v1.0
    2. Code Flow - v1.0
    3. Authorization Code Grant - v1.0 and beta
    4. Open ID - v1.0 and beta
    5. Implicit - v1.0 and beta
    6. Client Credentials - v1.0 and beta
    7. Resource Owner Password Credentials - v1.0 and beta

Using authorization flows:
    
    let mut oauth = OAuth::new();
    oauth.authorization_url("https://login.micorosoft.com/oauth2.authorize");
    // Add scopes and token urls if needed.
    
    let mut request = oauth.build().authorization_code_grant();
    let access_token = request.access_token().send().unwrap();

For a better understanding see the example's directory.

For more information on Microsoft graph and OAuth 2.0 authorization flows see:
https://docs.microsoft.com/en-us/azure/active-directory/develop/v2-app-types
