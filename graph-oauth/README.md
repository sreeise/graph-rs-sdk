# graph-oauth

OAuth2 for rust-onedrive.

### Stable and Unstable Features

The v1.0 OneDrive REST API token and code flows for Microsoft Graph and Microsoft Accounts
are stable and will not undergo any major changes.

To use these two authorization flows:
    
    let oauth = OAuth::token_flow();
    // and
    let oauth = OAuth::code_flow();

For more information on the token and code flows see:

https://docs.microsoft.com/en-us/onedrive/developer/rest-api/getting-started/msa-oauth?view=odsp-graph-online


https://docs.microsoft.com/en-us/onedrive/developer/rest-api/getting-started/msa-oauth?view=odsp-graph-online

All other authorization flows are considered unstable and may undergo substantial changes.
These include the open id, client credentials, and authorization code grants for the v2.0
Microsoft identity platform.