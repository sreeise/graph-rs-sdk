use graph_rs_sdk::prelude::*;

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";

// User id that you want to access onenote for.
static USER_ID: &str = "USER_ID";

static PAGE_ID: &str = "PAGE_ID";

pub async fn delete_page() {
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
