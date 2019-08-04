use rust_onedrive::from_to::*;
use rust_onedrive::oauth::OAuth;

fn main() {
    let oauth = OAuth::new();
    oauth
        .to_json_file("./examples/example_files/oauth.json")
        .unwrap();
    let oauth: OAuth = OAuth::from_json_file("./examples/example_files/oauth.json").unwrap();
    println!("{:#?}", &oauth);
}
