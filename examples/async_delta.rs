use graph_error::GraphFailure;
use graph_rs_sdk::prelude::*;

// This example shows how to call delta links and delta requests.

// For using a previous delta token see below.

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";

#[tokio::main]
async fn main() -> Result<(), GraphFailure> {
    let client = Graph::new_async(ACCESS_TOKEN);
    let mut delta_recv = client.v1().users().delta().send().await;

    loop {
        match delta_recv.recv().await {
            Some(delta) => match delta {
                Delta::Next(response) => {
                    println!("response: {:#?}", response);
                }
                Delta::Done(err) => {
                    if let Some(err) = err {
                        panic!("Error: {:#?}", err);
                    }
                    break;
                }
            },
            None => {
                println!("Got None");
                break;
            }
        }
    }
    Ok(())
}

// Using previous delta tokens

// See State Tokens: https://docs.microsoft.com/en-us/graph/delta-query-overview#state-tokens
// Using the deltaToken from the last response of a delta query request, you'll get changes
// (additions, deletions, or updates) to users since the last request.
static DELTA_TOKEN: &str = "<DELTA_TOKEN>";

async fn previous_delta_token() {
    let client = Graph::new(ACCESS_TOKEN);

    let delta_recv = client.v1().users().delta_token(DELTA_TOKEN).send();

    loop {
        match delta_recv.recv().await {
            Some(delta) => match delta {
                Delta::Next(response) => {
                    println!("response: {:#?}", response);
                }
                Delta::Done(err) => {
                    if let Some(err) = err {
                        panic!("Error: {:#?}", err);
                    }
                    break;
                }
            },
            None => {
                println!("Got None");
                break;
            }
        }
    }
}

// Delta Token URL query method

// A method for setting the delta token query is also provided. Using the delta token
// query method does not differ from the above request. Using the query is the same
// delta request with a previous delta token.

// The delta token example in the main method above is just syntactical sugar for convenience.
// Both methods set the deltaToken URL query with the provided delta token.
async fn delta_token_query() {
    let client = Graph::new_async(ACCESS_TOKEN);

    let mut receiver = client
        .v1()
        .users()
        .delta()
        .delta_token(DELTA_TOKEN)
        .send()
        .await;

    loop {
        match receiver.recv().await {
            Some(delta) => match delta {
                Delta::Next(response) => {
                    println!("response: {:#?}", response);
                }
                Delta::Done(err) => {
                    if let Some(err) = err {
                        panic!("Error: {:#?}", err);
                    }
                    break;
                }
            },
            None => {
                println!("Got None");
                break;
            }
        }
    }
}
