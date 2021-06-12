use graph_rs_sdk::prelude::*;

// This example shows how to call delta links and delta requests.

static ACCESS_TOKEN: &str = "<ACCESS_TOKEN>";

fn main() {
    let client = Graph::new(ACCESS_TOKEN);

    let delta_recv = client
        .v1()
        .groups() // The group id won't be used here.
        .delta()
        .send();

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
