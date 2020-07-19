use graph_error::GraphFailure;
use graph_rs::prelude::*;

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";
static USER_ID: &str = "USER_ID";

#[tokio::main]
async fn main() -> Result<(), GraphFailure> {
    let client = Graph::new_async(ACCESS_TOKEN);
    let mut delta_recv = client.v1().users(USER_ID).delta().send().await;

    loop {
        match delta_recv.recv().await {
            Some(delta) => match delta {
                Delta::Next(response) => {
                    println!("response: {:#?}", response);
                },
                Delta::Done(err) => {
                    if let Some(err) = err {
                        panic!("Error: {:#?}", err);
                    }
                    break;
                },
            },
            None => {
                println!("Got None");
                break;
            },
        }
    }
    Ok(())
}
