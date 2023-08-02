use graph_oauth::identity::{DeviceCodeCredential, TokenCredential};
use graph_rs_sdk::oauth::{MsalTokenResponse, OAuthSerializer};
use graph_rs_sdk::GraphResult;
use std::time::Duration;

// https://learn.microsoft.com/en-us/azure/active-directory/develop/v2-oauth2-device-code

// Update the client id with your own.
fn get_oauth() -> OAuthSerializer {
    let client_id = "CLIENT_ID";
    let mut oauth = OAuthSerializer::new();

    oauth
        .client_id(client_id)
        .authorization_url("https://login.microsoftonline.com/common/oauth2/v2.0/devicecode")
        .refresh_token_url("https://login.microsoftonline.com/common/oauth2/v2.0/token")
        .access_token_url("https://login.microsoftonline.com/common/oauth2/v2.0/token")
        .add_scope("files.read")
        .add_scope("offline_access");

    oauth
}

fn device_code_credential() -> DeviceCodeCredential {
    let client_id = "CLIENT_ID";

    DeviceCodeCredential::builder()
        .with_scope(vec!["files.read", "offline_access"])
        .with_client_id(client_id)
        .build()
}

// When polling to wait on the user to enter a device code you should check the errors
// so that you know what to do next.
//
// authorization_pending: The user hasn't finished authenticating, but hasn't canceled the flow. Repeat the request after at least interval seconds.
// authorization_declined: The end user denied the authorization request. Stop polling and revert to an unauthenticated state.
// bad_verification_code: The device_code sent to the /token endpoint wasn't recognized. Verify that the client is sending the correct device_code in the request.
// expired_token: Value of expires_in has been exceeded and authentication is no longer possible with device_code. Stop polling and revert to an unauthenticated state.
async fn poll_for_access_token(
    device_code: &str,
    interval: u64,
    message: &str,
) -> GraphResult<serde_json::Value> {
    let mut credential = device_code_credential();

    let mut oauth = get_oauth();
    oauth.device_code(device_code);

    let mut request = oauth.build_async().device_code();
    let response = request.access_token().send().await?;

    println!("{response:#?}");

    let status = response.status();

    let body: serde_json::Value = response.json().await?;
    println!("{body:#?}");

    if !status.is_success() {
        loop {
            // Wait the amount of seconds that interval is.
            std::thread::sleep(Duration::from_secs(interval));

            let response = request.access_token().send().await?;

            let status = response.status();
            println!("{response:#?}");

            let body: serde_json::Value = response.json().await?;
            println!("{body:#?}");

            if status.is_success() {
                return Ok(body);
            } else {
                let option_error = body["error"].as_str();

                if let Some(error) = option_error {
                    match error {
                        "authorization_pending" => println!("Still waiting on user to sign in"),
                        "authorization_declined" => panic!("user declined to sign in"),
                        "bad_verification_code" => {
                            println!("Bad verification code. Message:\n{message:#?}")
                        }
                        "expired_token" => panic!("token has expired - user did not sign in"),
                        _ => {
                            panic!("This isn't the error we expected: {error:#?}");
                        }
                    }
                } else {
                    // Body should have error or we should bail.
                    panic!("Crap hit the fan");
                }
            }
        }
    }

    Ok(body)
}

// The authorization url for device code must be https://login.microsoftonline.com/{tenant}/oauth2/v2.0/devicecode
// where tenant can be common,
pub async fn device_code() -> GraphResult<()> {
    let mut credential = device_code_credential();
    let response = credential.get_token_async().await?;

    println!("{:#?}", response);
    let json: serde_json::Value = response.json().await?;
    println!("{:#?}", json);

    let device_code = json["device_code"].as_str().unwrap();
    let interval = json["interval"].as_u64().unwrap();
    let message = json["message"].as_str().unwrap();

    /*
    The authorization request is a POST and a successful response body will look similar to:

    Object {
        "device_code": String("FABABAAEAAAD--DLA3VO7QrddgJg7WevrgJ7Czy_TDsDClt2ELoEC8ePWFs"),
        "expires_in": Number(900),
        "interval": Number(5),
        "message": String("To sign in, use a web browser to open the page https://microsoft.com/devicelogin and enter the code FQK5HW3UF to authenticate."),
        "user_code": String("FQK5HW3UF"),
        "verification_uri": String("https://microsoft.com/devicelogin"),
    }
    */

    // Print the message to the user who needs to sign in:
    println!("{message:#?}");

    // Poll for the response to the token endpoint. This will go through once
    // the user has entered the code and signed in.
    let access_token_json = poll_for_access_token(device_code, interval, message).await?;
    let access_token: MsalTokenResponse = serde_json::from_value(access_token_json)?;
    println!("{access_token:#?}");

    // Get a refresh token. First pass the access token to the oauth instance.
    oauth.access_token(access_token);
    let mut handler = oauth.build_async().device_code();

    let response = handler.refresh_token().send().await?;
    println!("{response:#?}");

    let body: serde_json::Value = response.json().await?;
    println!("{body:#?}");

    Ok(())
}
