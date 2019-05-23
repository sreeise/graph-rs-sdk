use graph_oauth::oauth::{GrantRequest, OAuth};

fn main() {
    let mut oauth: OAuth = OAuth::client_credentials_grant();
    oauth
        .authorize_url("https://login.microsoftonline.com/common/adminconsent?")
        .client_id("client_id")
        .state("state")
        .admin_consent(true)
        .redirect_uri("redirect_uri");
    let uri = oauth.encode_uri(GrantRequest::Authorization).unwrap();
    println!("{:#?}", uri);
}
