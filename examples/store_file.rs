use rust_onedrive::jsonfile::JsonFile;
use rust_onedrive::oauth::OAuth;
fn main() {
    let oauth = OAuth::default();
    JsonFile::json_file("./examples/example_files/oauth.json", &oauth).unwrap();
    let oauth: OAuth = JsonFile::from_file("./examples/example_files/oauth.json").unwrap();
    println!("{:#?}", &oauth);
}
