use graph_rs_sdk::*;

static ACCESS_TOKEN: &str = "<SITE_ID>";

static SITE_ID: &str = "<SITE_ID>";

static LIST_ID: &str = "<LIST_ID>";

static LIST_ITEM_ID: &str = "<LIST_ITEM_ID>";

pub async fn create_list() -> GraphResult<()> {
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
        .await?;

    println!("{response:#?}");

    Ok(())
}

pub async fn list_all_list_items() -> GraphResult<()> {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client
        .site(SITE_ID)
        .list(LIST_ID)
        .items()
        .list_items()
        .send()
        .await?;

    println!("{response:#?}");

    let body: serde_json::Value = response.json().await.unwrap();
    println!("{body:#?}");

    Ok(())
}

pub async fn create_list_item() -> GraphResult<()> {
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
        .await?;

    println!("{response:#?}");

    Ok(())
}

pub async fn update_list_item() -> GraphResult<()> {
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
        .await?;

    println!("{response:#?}");

    Ok(())
}

pub async fn get_list_item() -> GraphResult<()> {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client
        .site(SITE_ID)
        .list(LIST_ID)
        .item(LIST_ITEM_ID)
        .get_items()
        .send()
        .await?;

    println!("{response:#?}");

    let body: serde_json::Value = response.json().await.unwrap();
    println!("{body:#?}");

    Ok(())
}

pub async fn delete_list_item() -> GraphResult<()> {
    let client = Graph::new(ACCESS_TOKEN);

    let response = client
        .site(SITE_ID)
        .list(LIST_ID)
        .item(LIST_ITEM_ID)
        .delete_items()
        .send()
        .await?;

    println!("{response:#?}");

    Ok(())
}
