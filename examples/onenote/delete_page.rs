use graph_rs_sdk::header::{HeaderValue, CONTENT_TYPE};
use graph_rs_sdk::prelude::*;

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";

// User id that you want to access onenote for.
static USER_ID: &str = "USER_ID";

static PAGE_ID: &str = "PAGE_ID";

async fn upload_page_content() {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client
        .user(USER_ID)
        .onenote()
        .page(PAGE_ID)
        .delete_pages()
        .send()
        .await
        .unwrap();

    println!("{response:#?}");
}
