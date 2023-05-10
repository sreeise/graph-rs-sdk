#![allow(dead_code)]

// The BodyRead object is used to provide easy use of many different types.

/*
In request methods that require a body you can use the following values:

- Any thing that implements serde serialize
- Anything implementing Read or AsyncRead using BodyRead::from_reader and
   BodyRead::from_async_read
- reqwest::Body (Async requests only)
- reqwest::blocking::Body (Blocking requests only)
- FileConfig which is a helper object for downloading files but can also be used
  for uploading files.


BodyRead can also take serializable objects, reqwest::Body, and reqwest::blocking::Body but
you do not need to use BodyRead for those. You can pass these objects directly to the
body parameter for those api methods that need one.
 */

use graph_rs_sdk::http::BodyRead;
use graph_rs_sdk::Graph;
use std::fs::File;

fn main() {}

// When using reqwest::Body and reqwest::blocking::Body you should only use
// reqwest::Body if your using async and reqwest::blocking::Body when using
// blocking requests.

// If you use a reqwest::blocking::Body for an async method the tokio runtime
// will error and exit.

fn use_reqwest_blocking_body() {
    let body = reqwest::blocking::Body::from(String::new());

    let client = Graph::new("token");
    client
        .user("id")
        .get_mail_tips(body)
        .into_blocking()
        .send()
        .unwrap();
}

async fn use_reqwest_async_body() {
    let body = reqwest::Body::from(String::new());

    let client = Graph::new("token");
    client.user("id").get_mail_tips(body).send().await.unwrap();
}

// Using BodyRead

// BodyRead is a helper struct for using many different types
// as the body of a request.

fn use_body_read(file: File) {
    let _ = BodyRead::from_read(file).unwrap();
}

async fn use_async_body_read(file: tokio::fs::File) {
    let _ = BodyRead::from_async_read(file).await.unwrap();
}
