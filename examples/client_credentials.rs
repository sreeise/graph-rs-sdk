use examples_common::TestServer;
use from_as::*;
use graph_rs_sdk::oauth::OAuth;
use serde::Deserialize;
use warp::Filter;

// Client Credentials Grant
// If you have already given admin consent to a user you can skip
// browser authorization step and go strait to requesting an access token.
#[tokio::main]
async fn main() {
    let mut oauth = OAuth::new();
    oauth
        .client_id("<YOUR_CLIENT_ID>")
        .client_secret("<YOUR_CLIENT_SECRET>")
        .add_scope("https://graph.microsoft.com/.default")
        .redirect_uri("http://localhost:8000/redirect")
        .authorize_url("https://login.microsoftonline.com/common/adminconsent")
        .access_token_url("https://login.microsoftonline.com/common/oauth2/v2.0/token");

    let server_oauth = oauth.clone();
    let server = TestServer::serve_once(
        warp::get()
            .and(warp::path("redirect"))
            .and(warp::query::<Query>())
            .and(warp::any().map(move || server_oauth.clone()))
            .and_then(handle_redirect),
        ([127, 0, 0, 1], 8000),
    );

    oauth
        .build_async()
        .client_credentials()
        .browser_authorization()
        .open()
        .unwrap();

    // Wait for server to receive request and close
    server.await.expect("Failed to join server")
}

#[derive(Debug, Deserialize)]
struct Query {
    admin_consent: String,
    tenant: String,
}

async fn handle_redirect(
    query: Query,
    mut oauth: OAuth,
) -> Result<&'static str, std::convert::Infallible> {
    println!("Admin consent: {:#?}", query.admin_consent);
    println!("Tenant: {:#?}", query.tenant);

    let access_token = oauth
        .build_async()
        .client_credentials()
        .access_token()
        .send()
        .await
        .unwrap();

    oauth.access_token(access_token);

    println!("{:#?}", &oauth);

    // oauth
    //     .as_file("./examples/example_files/client_credentials.json")
    //     .unwrap();

    // Generic login page response.
    Ok("Successfully Logged In! You can close your browser.")
}
