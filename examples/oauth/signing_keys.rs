use graph_rs_sdk::oauth::graphdiscovery::{
    GraphDiscovery, MicrosoftSigningKeysV1, MicrosoftSigningKeysV2,
};
use graph_rs_sdk::oauth::OAuth;

#[allow(dead_code)]
fn get_signing_keys() {
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
    let _oauth: OAuth = GraphDiscovery::V1.oauth().unwrap();
}

#[allow(dead_code)]
fn tenant_discovery() {
    let _oauth: OAuth = GraphDiscovery::Tenant("<YOUR_TENANT_ID>".into())
        .oauth()
        .unwrap();
}

// Using async
#[allow(dead_code)]
async fn async_keys_discovery() {
    let signing_keys: MicrosoftSigningKeysV1 =
        GraphDiscovery::V1.async_signing_keys().await.unwrap();
    println!("{signing_keys:#?}");

    let signing_keys2: MicrosoftSigningKeysV2 =
        GraphDiscovery::V2.async_signing_keys().await.unwrap();
    println!("{signing_keys2:#?}");
}

#[allow(dead_code)]
async fn async_tenant_discovery() {
    let _oauth: OAuth = GraphDiscovery::Tenant("<YOUR_TENANT_ID>".into())
        .async_oauth()
        .await
        .unwrap();
}
