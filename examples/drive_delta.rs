use graph_rs::prelude::*;

static ACCESS_TOKEN: &str = "<ACCESS_TOKEN>";

fn main() {
    let client = Graph::new(ACCESS_TOKEN);

    let recv = client.v1().me().drive().delta().send().unwrap();

    match recv.recv() {
        Ok(value) => {
            println!("{:#?}", value);
        },
        Err(e) => {
            println!("Error: {:#?}", e);
        },
    }
}
