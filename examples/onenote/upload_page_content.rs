use graph_rs_sdk::header::{HeaderValue, CONTENT_TYPE};
use graph_rs_sdk::http::FileConfig;
use graph_rs_sdk::prelude::*;

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";

// User id that you want to access onenote for.
static USER_ID: &str = "USER_ID";

pub async fn upload_page_content() {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client
        .user(USER_ID)
        .onenote()
        .pages()
        .create_pages(&FileConfig::new("./test_files/one_note_page.html"))
        .header(CONTENT_TYPE, HeaderValue::from_static("text/html"))
        .send()
        .await
        .unwrap();

    println!("{response:#?}");
}
