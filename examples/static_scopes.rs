use rust_onedrive::oauth::OAuth;
use rust_onedrive::oauth::scope;

// Use static scopes for OneDrive
fn main() {
    let mut oauth = OAuth::code_flow();
    oauth
        .add_scope(scope::FILES_READ)
        .add_scope(scope::FILES_READ_SELECTED);

    println!("{:#?}", oauth.join_scopes(" "));
}
