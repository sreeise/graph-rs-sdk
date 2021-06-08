use examples_common::TestServer;
use graph_rs_sdk::oauth::OAuth;
use warp::{Filter, Reply};

/*
This example shows how to use a code_challenge and code_verifier
to perform the authorization code grant flow with proof key for
code exchange (PKCE).

For more info see: https://docs.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-auth-code-flow
And the PKCE RFC: https://tools.ietf.org/html/rfc7636
*/

static CLIENT_ID: &str = "<CLIENT_ID>";
static CLIENT_SECRET: &str = "<CLIENT_SECRET>";

#[tokio::main]
async fn main() {
    // Create a new Oauth config
    let mut oauth = OAuth::new();
    oauth
        .client_id(CLIENT_ID)
        .client_secret(CLIENT_SECRET)
        .add_scope("user.read")
        .add_scope("user.readwrite")
        .redirect_uri("http://localhost:8000/redirect")
        .authorize_url("https://login.microsoftonline.com/common/oauth2/v2.0/authorize")
        .access_token_url("https://login.microsoftonline.com/common/oauth2/v2.0/token")
        .refresh_token_url("https://login.microsoftonline.com/common/oauth2/v2.0/token")
        .response_type("code")
        .generate_sha256_challenge_and_verifier()
        .unwrap();

    // Make sure the server gets the same oauth configuration as the client
    let server_oauth = oauth.clone();
    let server = TestServer::serve_once(
        warp::get()
            .and(warp::path("redirect"))
            .and(warp::query::raw())
            .and(warp::any().map(move || server_oauth.clone()))
            .and_then(handle),
        ([127, 0, 0, 1], 8000),
    );

    oauth
        .build()
        .authorization_code_grant()
        .browser_authorization()
        .open()
        .expect("failed to open browser");

    // Wait for the server to get the redirect, then shut it down
    rx.recv().expect("Failed to receive");
    server.shutdown().await;
}

async fn handle(
    access_code: String,
    mut oauth: OAuth,
) -> Result<impl Reply, std::convert::Infallible> {
    // Print out the code for debugging purposes.
    println!("{:#?}", access_code);
    // Set the access code and request an access token.
    // Callers should handle the Result from requesting an access token
    // in case of an error here.
    let access_token = oauth
        .access_code(&access_code)
        .build_async()
        .authorization_code_grant()
        .access_token()
        .send()
        .await
        .expect("failed to send access token");

    oauth.access_token(access_token);
    println!("{:#?}", oauth);

    // Let the main thread know we've received a response and can
    // shut down (the server will stop listening, but will complete
    // requests in progress)
    tx.clone()
        .lock()
        .expect("poisoned!")
        .send(())
        .expect("failed to send");

    // Generic login page response.
    Ok(warp::reply::html(
        "Successfully Logged In! You can close your browser.",
    ))
}
