use graph_http::api_impl::GraphClientConfiguration;
use graph_oauth::ConfidentialClientApplication;
use http::HeaderMap;
use std::time::Duration;

// Use the GraphClientConfiguration for auth requests.
// When using GraphClientConfiguration in auth rqeuests the ClientApplication
// is ignored if provided.

// Additionally the configuration does not transfer over to GraphClient. This means
// that you will still need to create a GraphClientConfiguration (or clone and add the
// client application) for GraphClient if this is needed.

fn configure() {
    configure_ouath_client("<CLIENT_ID>", "<CLIENT_SECRET>", "<TENANT>");
}

fn configure_ouath_client(client_id: &str, client_secret: &str, tenant: &str) {
    let client_config = GraphClientConfiguration::new()
        .timeout(Duration::from_secs(30))
        .default_headers(HeaderMap::default())
        .retry(Some(10)) // retry 10 times if the request is not successful
        .concurrency_limit(Some(10)) // limit the number of concurrent requests on this client to 10
        .wait_for_retry_after_headers(true); // wait the amount of seconds specified by the Retry-After header of the response when we reach the throttling limits (429 Too Many Requests)

    let _confidential_client_application = ConfidentialClientApplication::builder(client_id)
        .with_client_secret(client_secret)
        .with_tenant(tenant)
        .with_config(&client_config)
        .build();
}
