use rust_onedrive::oauth::graphdiscovery::{
    GraphDiscovery, MicrosoftSigningKeysV1, MicrosoftSigningKeysV2,
};
use rust_onedrive::oauth::OAuth;

fn main() {
    // Lists info such as the authorization and token urls, jwks uri, and response types supported.
    let signing_keys: MicrosoftSigningKeysV1 = GraphDiscovery::V1.signing_keys().unwrap();
    println!("{:#?}", signing_keys);

    let signing_keys2: MicrosoftSigningKeysV2 = GraphDiscovery::V2.signing_keys().unwrap();
    println!("{:#?}", signing_keys2);

    // You can also create an OAuth instance from the signing keys. OAuth will use
    // parameters such as the authorization and token urls. This can save some
    // configuration time when setting values for OAuth. However, this will disregard
    // all other parameters for the MicrosoftSigningKeys. Use this if you do not
    // need the other values.
    let oauth: OAuth = GraphDiscovery::V1.oauth().unwrap();
}
