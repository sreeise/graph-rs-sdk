use rust_onedrive::oauth::OAuth;
use rust_onedrive::transform::{FromFile, ToFile};

fn main() {
    let oauth = OAuth::default();
    oauth
        .to_file("./examples/example_files/oauth.json")
        .unwrap();
    let oauth: OAuth = OAuth::from_file("./examples/example_files/oauth.json").unwrap();
    println!("{:#?}", &oauth);
}
