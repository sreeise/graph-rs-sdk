use rust_onedrive::drive::discovery::{
    GraphDiscovery, MicrosoftSigningKeysV1, MicrosoftSigningKeysV2,
};

fn main() {
    // Lists info such as the authorization and token urls, jwks uri, and response types supported.
    let signing_keys: MicrosoftSigningKeysV1 = GraphDiscovery::V1.signing_keys().unwrap();
    println!("{:#?}", signing_keys);

    let signing_keys2: MicrosoftSigningKeysV2 = GraphDiscovery::V2.signing_keys().unwrap();
    println!("{:#?}", signing_keys2);
}
