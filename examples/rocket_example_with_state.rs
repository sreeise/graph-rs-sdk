use examples_common::TestServer;
use from_as::*;
use graph_rs_sdk::oauth::OAuth;
use serde::Deserialize;
use warp::Filter;

/*
This example shows using Rocket to authenticate with Microsoft OneDrive that
includes authorization with a state parameter in the request query.

This example uses the code flow: https://docs.microsoft.com/en-us/onedrive/developer/rest-api/getting-started/msa-oauth?view=odsp-graph-online

If you have not set up an application to call the Graph API for OneDrive
API then you will want to first read through the information in rocket_example.rs
before moving forward. A client id and client secret are needed to
call the overdrive API in order to authenticate users. This is done
through the Microsoft application portal or through Azure. Creating an
application will create an application ID which is the client id. Then,
under application secrets, a new password will need to be generated. This
password is the client secret. The rocket_example.rs file has more information
on how to set up an application.
*/
#[tokio::main]
async fn main() {
    // The client_id and client_secret must be changed before running this example.
    let mut oauth = OAuth::new();
    oauth
        .client_id("<YOUR_CLIENT_ID>")
        .client_secret("<YOUR_CLIENT_SECRET>")
        .add_scope("Files.Read")
        .add_scope("Files.ReadWrite")
        .add_scope("Files.Read.All")
        .add_scope("Files.ReadWrite.All")
        .add_scope("wl.offline_access")
        .redirect_uri("http://localhost:8000/redirect")
        .authorize_url("https://login.live.com/oauth20_authorize.srf?")
        .access_token_url("https://login.live.com/oauth20_token.srf")
        .refresh_token_url("https://login.live.com/oauth20_token.srf")
        .response_mode("query")
        .state("13534298")
        .logout_url("https://login.live.com/oauth20_logout.srf?")
        // If this is not set, the redirect_url given above will be used for the logout redirect.
        // See logout.rs for an example.
        .post_logout_redirect_uri("http://localhost:8000/redirect");

    let server_oauth = oauth.clone();
    let server = TestServer::serve_once(
        warp::get()
            .and(warp::path("redirect"))
            .and(warp::query::<Query>())
            .and(warp::any().map(move || server_oauth.clone()))
            .and_then(handle_redirect),
        ([127, 0, 0, 1], 8000),
    );

    // Get the oauth client and request a browser sign in
    // The url used is the same url given in method: OAuth::authorize_url()
    let mut request = oauth.build().code_flow();
    request.browser_authorization().open().unwrap();

    // Wait for server to receive the redirect and close
    server.await.expect("Failed to join server thread")
}

#[derive(Debug, Deserialize)]
struct Query {
    code: String,
    state: String,
}

async fn handle_redirect(
    query: Query,
    mut oauth: OAuth,
) -> Result<&'static str, std::convert::Infallible> {
    let Query { code, state } = query;

    // Print out the code and state for debugging purposes.
    println!("{:#?}", code);
    println!("{:#?}", state);

    // Assert that the state is the same as the one given in the original request.
    assert_eq!("13534298", &state);

    // Set the access code and request an access token.
    // Callers should handle the Result from requesting an access token
    // in case of an error here.
    oauth.response_type("token");
    oauth.state(&state);
    oauth.access_code(&code);

    // Request the access token.
    let access_token = oauth
        .build_async()
        .code_flow()
        .access_token()
        .send()
        .await
        .unwrap();

    oauth.access_token(access_token);

    // If all went well here we can print out the OAuth config with the Access Token.
    println!("{:#?}", &oauth);

    // TODO: remove?
    // Save our configuration to a file so we can retrieve it for other requests.
    // oauth
    //     .as_file("./examples/example_files/web_oauth.json")
    //     .unwrap();

    // Generic login page response.
    Ok("Successfully Logged In! You can close your browser.")
}
