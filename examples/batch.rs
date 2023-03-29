use graph_rs_sdk::prelude::*;

// This example shows batch requests to perform multiple requests at once.
// The response may not return all at one time. In these cases a next link url
// is given to get the next response. The batch method will continue calling
// the next link until there is none. Each response is sent using a channel
// and the channel receiver will return each response.

// For more info on batch requests see https://docs.microsoft.com/en-us/graph/json-batching?context=graph%2Fapi%2F1.0&view=graph-rest-1.0

static USER_ID: &str = "USER_ID";
static ACCESS_TOKEN: &str = "ACCESS_TOKEN";

#[tokio::main]
async fn main() {
    let client = Graph::new(ACCESS_TOKEN);
    let json = serde_json::json!({
        "requests": [
            {
                "id": "1",
                "method": "GET",
                "url": format!("/users/{USER_ID}/drive")
            },
            {
                "id": "2",
                "method": "GET",
                "url": format!("/users/{USER_ID}/drive/root")
            },
            {
                "id": "3",
                "method": "GET",
                "url": format!("/users/{USER_ID}/drive/recent")
            },
            {
                "id": "4",
                "method": "GET",
                "url": format!("/users/{USER_ID}/drive/root/children")
            },
            {
                "id": "5",
                "method": "GET",
                "url": format!("/users/{USER_ID}/drive/special/documents")
            },
        ]
    });

    let response = client.batch(&json).send().await.unwrap();

    let body: serde_json::Value = response.json().await.unwrap();
    println!("{body:#?}");
}
