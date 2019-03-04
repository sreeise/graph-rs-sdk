use graph_oauth::oauth::OAuth;
use jsonfile::JsonFile;

fn main() {
    let mut oauth = web_client();

    // Run this first, then get the code from the browser URL. Afterward,
    // comment out the the browser_sign_in() method below and run set_code_request_token(your_access_token).
    oauth.browser_sign_in().unwrap();

    /*
    browser_flow() opens users default browser to the authentication page
    for Microsoft Accounts. OAuth knows what to open by
    the AccountType specified in native_client().

    Note that browser_flow() runs in different thread so the main thread
    is not blocked.

    Upon sign in, the page will redirect to the redirect given in
    get_oauth(). This redirect is specific to the URI set
    in the microsoft application portal. The redirect URI will
    return with a code. This code needs to be set in OAuth
    to make an access token request. Get the code from the
    browser url bar and
    */
}

fn web_client() -> OAuth {
    // Setting true in the new() method here tells OAuth
    // to use the default scopes that are set in the application portal.
    // A Microsoft URL that can retrieve these scopes is then used instead and
    // makes it easier to set up OAuth. The scopes can still be implemented
    // manually by setting OAuth::new to false and using method add_scope()
    let mut oauth = OAuth::default();
    oauth
        .client_id("<CLIENT_ID>")
        // Or whatever you set the redirect to for a web client.
        .redirect_url("http://localhost:8000/redirect")
        .client_secret("<CLIENT_SECRET>")
        .authorize_url("https://login.microsoftonline.com/common/oauth2/v2.0/authorize?")
        .access_token_url("https://login.microsoftonline.com/common/oauth2/v2.0/token?");

    oauth
}

#[allow(dead_code)]
fn set_code_request_token(access_code: &str) {
    let mut oauth = web_client();

    // Set the access code that will be used to request an
    // access token and/or refresh token.
    oauth.access_code(access_code);

    // Makes the POST request for an access token/refresh token.
    // This is stored in the struct AccessToken.
    match oauth.request_access_token() {
        Ok(_) => {
            println!("The request was successful\n");
            println!("{:#?}", &oauth);

            /*
            Stores OAuth as json using serde_json.

            To get OAuth back from the json file run:
            let mut oauth: OAuth = JsonFile::from_file("example/oauth.json").unwrap();
            */
            JsonFile::json_file("example/web_client_flow.json", &oauth).unwrap();
        },
        Err(e) => println!("There was an error: {:#?}", e),
    }
}
