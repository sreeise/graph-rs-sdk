use graph_rs_sdk::prelude::*;

// This example shows how to call a delta request using a delta token received from a
// previous delta request.

static ACCESS_TOKEN: &str = "<ACCESS_TOKEN>";

// See State Tokens: https://docs.microsoft.com/en-us/graph/delta-query-overview#state-tokens
// Using the deltaToken from the last response of a delta query request, you'll get changes
// (additions, deletions, or updates) to users since the last request.
static DELTA_TOKEN: &str = "<DELTA_TOKEN>";

fn main() {
    let client = Graph::new(ACCESS_TOKEN);

    let delta_recv = client.v1().users().delta_token(DELTA_TOKEN).send();

    loop {
        match delta_recv.recv() {
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

                        // All next links have been called.
                        // Break here. The channel has been closed.
                        break;
                    }
                }
            }
            Err(e) => {
                println!("{:#?}", e);
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
fn delta_token_query() {
    let client = Graph::new(ACCESS_TOKEN);

    let _ = client.v1().users().delta().delta_token(DELTA_TOKEN).send();
}
