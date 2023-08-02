// By default the AccessToken access_token (bearer token) and id_token fields
// are logged or printed to the console as [REDACTED] by the AccessToken Debug implementation.

// You can enable logging of these fields by setting the enable personally
// identifiable information field to true called enable_pii.

use graph_rs_sdk::oauth::MsalTokenResponse;

fn enable_pii_on_access_token(access_token: &mut MsalTokenResponse) {
    access_token.enable_pii_logging(true);
    println!("{access_token:#?}");
}
