use graph_rs::prelude::*;

// This example shows batch requests to perform multiple requests at once.
// The response may not return all at one time. In these cases a next link url
// is given to get the next response. The batch method will continue calling
// the next link until there is none. Each response is sent using a channel
// and the channel receiver will return a serde_json::Value each time.

// For more info on batch requests see https://docs.microsoft.com/en-us/graph/json-batching?context=graph%2Fapi%2F1.0&view=graph-rest-1.0

static USER_ID: &str = "USER_ID";
static ACCESS_TOKEN: &str = "ACCESS_TOKEN";

fn main() {
    let client = Graph::new(ACCESS_TOKEN);

    let json = serde_json::json!({
        "requests": [
            {
                "id": "1",
                "method": "GET",
                "url": format!("/users/{}/drive", USER_ID)
            },
            {
                "id": "2",
                "method": "GET",
                "url": format!("/users/{}/drive/root", USER_ID)
            },
            {
                "id": "3",
                "method": "GET",
                "url": format!("/users/{}/drive/recent", USER_ID)
            },
            {
                "id": "4",
                "method": "GET",
                "url": format!("/users/{}/drive/root/children", USER_ID)
            },
            {
                "id": "5",
                "method": "GET",
                "url": format!("/users/{}/drive/activities", USER_ID)
            }
        ]
    });

    let recv = client.v1().batch(&json).send().unwrap();

    match recv.recv() {
        Ok(value) => {
            println!("{:#?}", value);
        },
        Err(e) => {
            println!("Error: {:#?}", e);
        },
    }
}
