use from_as::*;
use graph_http::BlockingHttpClient;
use graph_rs::oauth::OAuth;
use graph_rs::prelude::*;
use std::convert::TryFrom;

fn main() {
    let oauth = OAuth::from_file("./examples/example_files/web_oauth.json").unwrap();
    let mut graph = Graph::try_from(&oauth).unwrap();

    // You can pick a function below to query common OneDrive resources.
    // For more common OneDrive API queries see the EP trait.

    // This will run all the API requests below.
    drive_root(&mut graph);
    drive_root_children(&mut graph);
    special_docs(&mut graph);
    special_docs_child(&mut graph);
}

fn drive_root(graph: &mut Graph<BlockingHttpClient>) {
    let drive_item: GraphResponse<serde_json::Value> =
        graph.v1().me().drive().root().send().unwrap();
    println!("{:#?}", drive_item);
}

fn drive_root_children(graph: &mut Graph<BlockingHttpClient>) {
    let drive_item = graph.v1().me().drive().root_children().send().unwrap();
    println!("{:#?}", drive_item);
}

fn special_docs(graph: &mut Graph<BlockingHttpClient>) {
    let drive_item = graph.v1().me().drive().special_documents().send().unwrap();
    println!("{:#?}", drive_item);
}

fn special_docs_child(graph: &mut Graph<BlockingHttpClient>) {
    let drive_item = graph
        .v1()
        .me()
        .drive()
        .special_documents_children()
        .send()
        .unwrap();
    println!("{:#?}", drive_item);
}
