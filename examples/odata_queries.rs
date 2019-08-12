use rust_onedrive::prelude::*;

// Add query parameters to a request.

// For more info on query parameters see: https://docs.microsoft.com/en-us/onedrive/developer/rest-api/concepts/optional-query-parameters?view=odsp-graph-online#expanding-collections

// See rocket_example.rs, native_client.rs, or web_client.rs for getting
// access_token/refresh_tokens

#[allow(dead_code)]
fn expand_example() {
    let drive = Drive::new("ACCESS_TOKEN");

    let mut req = drive.v1().drive_root();
    req.expand(&["children"]);

    let collection = req.send();
    println!("{:#?}", collection);
}

#[allow(dead_code)]
fn search_example() {
    let drive = Drive::new("ACCESS_TOKEN");

    let mut req = drive.v1().drive_root();
    req.search("name");

    let collection = req.send().unwrap();
    println!("{:#?}", collection);
}

fn main() {
    expand_example();
    search_example();
}
