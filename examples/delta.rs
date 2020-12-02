use graph_http::traits::*;
use graph_rs::prelude::*;
use std::sync::mpsc::Receiver;

/*
This example shows how to call delta links and delta requests.
*/

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
                    },
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
                    },
                }
            },
            Err(e) => {
                println!("{:#?}", e);
                break;
            },
        }
    }
}

/*
Some responses may return a delta link that can be used
to get further changes. The Collection<T> type as well as
the serde_json::Value type have implementations for the
NextLink and DeltaLink trait that can be used to call the link and
any further next link values.

To call the delta link for a different type it must implement
the NextLink and DeltaLink traits.
*/

pub fn call_response_delta_link() {
    // Say for instance you retrieved a serde_json::Value
    // for a response which may look like the following:
    let json = serde_json::json!({
        "@odata.nextLink": "https://example.com/next",
        "@odata.deltaLink": "https://example.com/delta"
    });

    // Since the NextLink and DeltaLink traits are implemented
    // for serde_json::Value you can pass an access token
    // to the delta method and get a channel receiver
    // that will return any remaining calls to the next link.
    let recv_option = json.delta::<serde_json::Value>(ACCESS_TOKEN);

    if let Some(recv) = recv_option {
        loop {
            match recv.recv() {
                Ok(delta) => {
                    match delta {
                        Delta::Next(response) => {
                            println!("{:#?}", response);
                        },
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
                        },
                    }
                },
                Err(e) => {
                    println!("Error: {:#?}", e);
                },
            }
        }
    }
}

// Implementing NextLink and DeltaLink for a new type:

pub struct MyType {
    next_link: Option<String>,
    delta_link: Option<String>,
    // Any other fields ...
}

impl NextLink for MyType {
    fn next_link(&self) -> Option<String> {
        self.next_link.clone()
    }
}

impl DeltaLink for MyType {
    fn delta_link(&self) -> Option<String> {
        self.delta_link.clone()
    }
}

// Then you can use your own type to call
// the delta and next links.
#[allow(dead_code)]
fn delta_my_type() {
    let my_type = MyType {
        next_link: Some("https://example.com/next".into()),
        delta_link: Some("https://example.com/delta".into()),
    };

    let recv: Option<Receiver<Delta<serde_json::Value>>> = my_type.delta(ACCESS_TOKEN);

    if let Some(recv) = recv {
        loop {
            match recv.recv() {
                Ok(delta) => {
                    match delta {
                        Delta::Next(response) => {
                            println!("{:#?}", response);
                        },
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
                        },
                    }
                },
                Err(e) => {
                    println!("Error: {:#?}", e);
                },
            }
        }
    }
}
