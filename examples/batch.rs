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
                "url": format!("/users/{}/drive/special/documents", USER_ID)
            }
        ]
    });

    let recv = client.batch(&json).send().await.unwrap();

    loop {
        match recv.recv() {
            Ok(delta) => {
                match delta {
                    Delta::Next(response) => {
                        println!("{:#?}", response);
                    }
                    Delta::Done(err) => {
                        println!("All Done");

                        // If the delta request ended in an error Delta::Done
                        // will return Some(GraphFailure)
                        if let Some(err) = err {
                            println!("Error: {:#?}", err);
                            println!("Description: {:#?}", err);
                        }

                        // Break here. The channel has been closed.
                        break;
                    }
                }
            }
            Err(e) => {
                println!("Error {:#?}", e);
                println!("Description: {:#?}", e);
                break;
            }
        }
    }
}
