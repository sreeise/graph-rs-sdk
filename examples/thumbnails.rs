use from_as::*;
use graph_rs::oauth::OAuth;
use graph_rs::prelude::*;
use graph_rs_types::entitytypes::ThumbnailSet;
use std::convert::TryFrom;

static ITEM_ID: &str = "ITEM_ID";

pub fn main() {
    get_thumbnails();
}

pub fn get_thumbnails() {
    let oauth: OAuth = OAuth::from_file("./examples/example_files/web_oauth.json").unwrap();
    let graph = Graph::try_from(&oauth).unwrap();
    let collection: Collection<ThumbnailSet> = graph
        .v1()
        .me()
        .drive()
        .thumbnails()
        .by_id(ITEM_ID)
        .send()
        .unwrap();
    println!("{:#?}", collection.value());
}
