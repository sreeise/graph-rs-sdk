use graph_rs_sdk::prelude::*;

static ACCESS_TOKEN: &str = "<SITE_ID>";

static SITE_ID: &str = "<SITE_ID>";

static LIST_ID: &str = "<LIST_ID>";
static LIST_ITEM_ID: &str = "<LIST_ITEM_ID>";

#[tokio::main]
async fn main() {
    create_list().await;
    list_all_list_items().await;
    create_list_item().await;
    get_list_item().await;
    update_list_item().await;
    delete_list_item().await;
}

async fn create_list() {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client
        .site(SITE_ID)
        .lists()
        .create_lists(&serde_json::json!({
            "displayName": "Books",
            "columns": [
                {
                    "name": "Author",
                    "text": {}
                },
                {
                    "name": "PageCount",
                    "number": {}
                }
            ],
            "list": {
                "template": "genericList"
            }
        }))
        .send()
        .await
        .unwrap();

    println!("{response:#?}");
}

async fn list_all_list_items() {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client
        .site(SITE_ID)
        .list(LIST_ID)
        .items()
        .list_items()
        .send()
        .await
        .unwrap();

    println!("{response:#?}");
}

async fn create_list_item() {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client
        .site(SITE_ID)
        .list(LIST_ID)
        .items()
        .create_items(&serde_json::json!({
            "ListItem": {
                "fields": {
                    "Title": "Widget"
                }
            }
        }))
        .send()
        .await
        .unwrap();

    println!("{response:#?}");
}

async fn update_list_item() {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client
        .site(SITE_ID)
        .list(LIST_ID)
        .item(LIST_ITEM_ID)
        .update_items(&serde_json::json!({
            "ListItem": {
                "fields": {
                    "Color": "Fuchsia",
                    "Quantity": 934
                }
            }
        }))
        .send()
        .await
        .unwrap();

    println!("{response:#?}");
}

async fn get_list_item() {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client
        .site(SITE_ID)
        .list(LIST_ID)
        .item(LIST_ITEM_ID)
        .get_items()
        .send()
        .await
        .unwrap();

    println!("{response:#?}");
}

async fn delete_list_item() {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client
        .site(SITE_ID)
        .list(LIST_ID)
        .item(LIST_ITEM_ID)
        .delete_items()
        .send()
        .await
        .unwrap();

    println!("{response:#?}");
}
