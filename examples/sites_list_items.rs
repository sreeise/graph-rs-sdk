use graph_rs::prelude::*;

static ACCESS_TOKEN: &str = "<SITE_ID>";

static SITE_ID: &str = "<SITE_ID>";

static LIST_ID: &str = "<LIST_ID>";
static LIST_ITEM_ID: &str = "<LIST_ITEM_ID>";

fn main() {
    create_list();
    list_all_list_items();
    create_list_item();
    get_list_item();
    update_list_item();
    delete_list_item();
}

pub fn create_list() {
    let client = Graph::new(ACCESS_TOKEN);

    let list_result = client
        .v1()
        .sites(SITE_ID)
        .lists()
        .create(&serde_json::json!({
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
        .send();

    if let Ok(list_response) = list_result {
        println!("{:#?}", list_response);
    } else if let Err(e) = list_result {
        println!("Error: {:#?}", e);
    }
}

fn list_all_list_items() {
    let client = Graph::new(ACCESS_TOKEN);

    let list_item_response = client
        .v1()
        .sites(SITE_ID)
        .lists()
        .items()
        .list(LIST_ID)
        .send();

    println!("{:#?}", list_item_response);
}

fn create_list_item() {
    let client = Graph::new(ACCESS_TOKEN);

    let list_item_response = client
        .v1()
        .sites(SITE_ID)
        .lists()
        .items()
        .create(
            LIST_ID,
            &serde_json::json!({
                "ListItem": {
                    "fields": {
                        "Title": "Widget"
                    }
                }
            }),
        )
        .send();

    println!("{:#?}", list_item_response);
}

fn update_list_item() {
    let client = Graph::new(ACCESS_TOKEN);

    let list_item_response = client
        .v1()
        .sites(SITE_ID)
        .lists()
        .items()
        .update(
            LIST_ID,
            LIST_ITEM_ID,
            &serde_json::json!({
                "ListItem": {
                    "fields": {
                        "Color": "Fuchsia",
                        "Quantity": 934
                    }
                }
            }),
        )
        .send();

    println!("{:#?}", list_item_response);
}

fn get_list_item() {
    let client = Graph::new(ACCESS_TOKEN);

    let list_item_response = client
        .v1()
        .sites(SITE_ID)
        .lists()
        .items()
        .get(LIST_ID, LIST_ITEM_ID)
        .send();

    println!("{:#?}", list_item_response);
}

fn delete_list_item() {
    let client = Graph::new(ACCESS_TOKEN);

    let sites = client
        .v1()
        .sites(SITE_ID)
        .lists()
        .items()
        .delete(LIST_ID, LIST_ITEM_ID)
        .send();

    println!("{:#?}", sites);
}
