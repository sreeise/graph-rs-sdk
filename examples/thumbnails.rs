use graph_rs::prelude::*;

static ACCESS_TOKEN: &str = "ACCESS_TOKEN";

pub fn main() {
    list_thumbnails();
}

pub fn list_thumbnails() {
    let graph = Graph::new(ACCESS_TOKEN);
    let collection = graph.v1().me().drive().list_thumbnails().send().unwrap();
    println!("{:#?}", collection.body());
}
