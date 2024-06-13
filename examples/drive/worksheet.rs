use graph_rs_sdk::http::Body;
use graph_rs_sdk::GraphClient;

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";
static DRIVE_ID: &str = "DRIVE_ID";
static ITEM_ID: &str = "ITEM_ID";

pub async fn update_range_by_address() {
    let client = GraphClient::new(ACCESS_TOKEN);

    //update single cell
    let range_address = "A1";
    let range_content = serde_json::json!({
        "values": [["Changed value"]]
    });

    let body = Body::from(range_content.to_string());
    let resp = client
        .drive(DRIVE_ID)
        .item(ITEM_ID)
        .workbook()
        .worksheet("Sheet1")
        .update_range_object_by_address(range_address, body)
        .send()
        .await
        .unwrap();

    let workbook_json: serde_json::Value = resp.json().await.unwrap();

    println!("workbook {:#?}", workbook_json);
}
