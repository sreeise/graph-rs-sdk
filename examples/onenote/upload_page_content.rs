use graph_rs_sdk::header::{HeaderValue, CONTENT_TYPE};
use graph_rs_sdk::http::FileConfig;
use graph_rs_sdk::*;
use std::fs::OpenOptions;
use std::io::Read;

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";

// User id that you want to access onenote for.
static USER_ID: &str = "USER_ID";

static FILE_PATH: &str = "./FILE.html";

// There is currently no way to upload the binary contents of a page via multipart as shown
// here: https://learn.microsoft.com/en-us/graph/api/section-post-pages?view=graph-rest-1.0

pub async fn upload_page_content() {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client
        .user(USER_ID)
        .onenote()
        .pages()
        .create_pages(&FileConfig::new(FILE_PATH))
        .header(CONTENT_TYPE, HeaderValue::from_static("text/html"))
        .send()
        .await
        .unwrap();

    println!("{response:#?}");
}

pub async fn upload_page_content_using_file() -> GraphResult<()> {
    let client = Graph::new(ACCESS_TOKEN);

    let mut file = OpenOptions::new().read(true).open(FILE_PATH)?;

    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    let response = client
        .user(USER_ID)
        .onenote()
        .pages()
        .create_pages(&buf)
        .header(CONTENT_TYPE, HeaderValue::from_static("text/html"))
        .send()
        .await
        .unwrap();

    println!("{response:#?}");

    Ok(())
}
