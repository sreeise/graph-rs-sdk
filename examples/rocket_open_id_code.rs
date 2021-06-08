use examples_common::TestServer;
use from_as::*;
use graph_rs_sdk::oauth::{IdToken, OAuth};
use serde::Deserialize;
use std::convert::TryFrom;
use warp::Filter;

#[tokio::main]
async fn main() {
    // Create an OAuth struct with the needed credentials.
    // See the following link for more info on open ID connect:
    // https://docs.microsoft.com/en-us/azure/active-directory/develop/v2-protocols-oidc
    let mut oauth = OAuth::new();
    oauth
        .client_id("<YOUR_CLIENT_ID>")
        .client_secret("<YOUR_CLIENT_SECRET>")
        .authorize_url("https://login.microsoftonline.com/common/oauth2/v2.0/authorize")
        .redirect_uri("http://localhost:8000/redirect")
        .access_token_url("https://login.microsoftonline.com/common/oauth2/v2.0/token")
        .refresh_token_url("https://login.microsoftonline.com/common/oauth2/v2.0/token")
        .response_type("id_token code")
        .response_mode("form_post")
        .add_scope("openid")
        .add_scope("Files.Read")
        .add_scope("Files.ReadWrite")
        .add_scope("Files.Read.All")
        .add_scope("Files.ReadWrite.All")
        .add_scope("offline_access")
        .nonce("7362CAEA-9CA5")
        .prompt("login")
        .state("12345");

    let server_oauth = oauth.clone();
    let server = TestServer::serve_once(
        warp::post()
            .and(warp::path("redirect"))
            .and(warp::body::form())
            .and(warp::any().map(move || server_oauth.clone()))
            .and_then(handle_redirect),
        ([127, 0, 0, 1], 8000),
    );

    // Use the OpenId trait from OAuth to request an access code.
    // The full name syntax is used here so it does not clash with methods
    // in the other grant types.
    oauth
        .build()
        .open_id_connect()
        .browser_authorization()
        .open()
        .unwrap();

    // Wait for server to receive the response and close
    server.await.expect("failed to join server thread")
}

#[derive(Debug, Deserialize)]
struct FormData {
    id_token: String,
}

async fn handle_redirect(
    form_data: FormData,
    mut oauth: OAuth,
) -> Result<&'static str, std::convert::Infallible> {
    let id_token = form_data.id_token;

    // Print the string for debugging in case the attempt to deserialize the response
    // in the TryFrom method below does not work..
    println!("Token response:\n{:#?}\n", id_token);

    // Use the TryFrom impl to get an IdToken from a string
    // and pass the IdToken to OAuth.
    let token: IdToken = IdToken::try_from(id_token).unwrap();
    println!("IdToken:\n{:#?}\n", token);

    let access_token = oauth
        .id_token(token)
        .build_async()
        .code_flow()
        .access_token()
        .send()
        .await
        .unwrap();

    oauth.access_token(access_token);

    // If all went well here we can print out the OAuth config with the Access Token.
    println!("OAuth:\n{:#?}\n", &oauth);

    // oauth
    //     .as_file("./examples/example_files/web_oauth.json")
    //     .unwrap();

    Ok("Successfully Logged In! You can close your browser.")
}
